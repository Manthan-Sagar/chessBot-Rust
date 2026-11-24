//! Chess board representation, FEN parsing, and board state management.

use crate::bitboard::*;
use crate::types::*;
use crate::zobrist::ZobristKeys;

pub const START_POSITION: &str = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1 ";
pub const TRICKY_POSITION: &str = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 ";
pub const KILLER_POSITION: &str = "rnbqkb1r/pp1p1pPp/8/2p1pP2/1P1P4/3P3P/P1P1P3/RNBQKBNR w KQkq e6 0 1";
pub const CMK_POSITION: &str = "r2q1rk1/ppp2ppp/2n1bn2/2b1p3/3pP3/3P1NPP/PPP1NPB1/R1BQ1RK1 b - - 0 9 ";
pub const REPETITIONS: &str = "2r3k1/R7/8/1R6/8/8/P4KPP/8 w - - 0 40 ";

pub const CASTLING_RIGHTS: [u8; 64] = [
     7, 15, 15, 15,  3, 15, 15, 11,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    15, 15, 15, 15, 15, 15, 15, 15,
    13, 15, 15, 15, 12, 15, 15, 14,
];

#[derive(Clone, Copy, Debug)]
pub struct BoardState {
    pub bitboards: [Bitboard; 12],
    pub occupancies: [Bitboard; 3],
    pub side: Color,
    pub enpassant: usize,
    pub castle: u8,
    pub hash_key: u64,
}

#[derive(Clone)]
pub struct Board {
    pub bitboards: [Bitboard; 12],
    pub occupancies: [Bitboard; 3],
    pub side: Color,
    pub enpassant: usize,
    pub castle: u8,
    pub hash_key: u64,
    pub repetition_table: [u64; 1000],
    pub repetition_index: usize,
    pub ply: usize,
}

impl Board {
    pub fn new() -> Self {
        Self {
            bitboards: [0; 12],
            occupancies: [0; 3],
            side: Color::White,
            enpassant: square::NO_SQ,
            castle: 0,
            hash_key: 0,
            repetition_table: [0; 1000],
            repetition_index: 0,
            ply: 0,
        }
    }

    pub fn reset(&mut self) {
        self.bitboards = [0; 12];
        self.occupancies = [0; 3];
        self.side = Color::White;
        self.enpassant = square::NO_SQ;
        self.castle = 0;
        self.hash_key = 0;
        self.repetition_table = [0; 1000];
        self.repetition_index = 0;
        self.ply = 0;
    }

    #[inline(always)]
    pub fn save_state(&self) -> BoardState {
        BoardState {
            bitboards: self.bitboards,
            occupancies: self.occupancies,
            side: self.side,
            enpassant: self.enpassant,
            castle: self.castle,
            hash_key: self.hash_key,
        }
    }

    #[inline(always)]
    pub fn restore_state(&mut self, state: &BoardState) {
        self.bitboards = state.bitboards;
        self.occupancies = state.occupancies;
        self.side = state.side;
        self.enpassant = state.enpassant;
        self.castle = state.castle;
        self.hash_key = state.hash_key;
    }

    pub fn update_occupancies(&mut self) {
        self.occupancies = [0; 3];
        for piece in 0..6 {
            self.occupancies[Color::White as usize] |= self.bitboards[piece];
        }
        for piece in 6..12 {
            self.occupancies[Color::Black as usize] |= self.bitboards[piece];
        }
        self.occupancies[Color::Both as usize] =
            self.occupancies[Color::White as usize] | self.occupancies[Color::Black as usize];
    }

    pub fn parse_fen(&mut self, fen: &str, zobrist: &ZobristKeys) {
        self.reset();
        let tokens: Vec<&str> = fen.split_whitespace().collect();
        if tokens.is_empty() {
            return;
        }

        // 1. Piece placement
        let mut rank = 0;
        let mut file = 0;
        for c in tokens[0].chars() {
            if c == '/' {
                rank += 1;
                file = 0;
            } else if let Some(digit) = c.to_digit(10) {
                file += digit as usize;
            } else if let Some(piece) = Piece::from_char(c) {
                let sq = rank * 8 + file;
                set_bit(&mut self.bitboards[piece as usize], sq);
                file += 1;
            }
        }

        // 2. Active side
        if tokens.len() > 1 {
            self.side = if tokens[1] == "w" {
                Color::White
            } else {
                Color::Black
            };
        }

        // 3. Castling rights
        if tokens.len() > 2 {
            for c in tokens[2].chars() {
                match c {
                    'K' => self.castle |= CASTLE_WK,
                    'Q' => self.castle |= CASTLE_WQ,
                    'k' => self.castle |= CASTLE_BK,
                    'q' => self.castle |= CASTLE_BQ,
                    _ => {}
                }
            }
        }

        // 4. En passant target square
        if tokens.len() > 3 && tokens[3] != "-" {
            self.enpassant = parse_square(tokens[3]).unwrap_or(square::NO_SQ);
        } else {
            self.enpassant = square::NO_SQ;
        }

        self.update_occupancies();
        self.hash_key = zobrist.generate_hash_key(&self.bitboards, self.side, self.enpassant, self.castle);
    }

    #[inline(always)]
    pub fn is_repetition(&self) -> bool {
        for i in 0..self.repetition_index {
            if self.repetition_table[i] == self.hash_key {
                return true;
            }
        }
        false
    }

    pub fn print(&self) {
        println!();
        for rank in 0..8 {
            print!("  {} ", 8 - rank);
            for file in 0..8 {
                let sq = rank * 8 + file;
                let mut found = false;
                for piece in 0..12 {
                    if get_bit(self.bitboards[piece], sq) {
                        print!(" {}", Piece::from_char(PIECE_CHARS[piece]).unwrap().to_char());
                        found = true;
                        break;
                    }
                }
                if !found {
                    print!(" .");
                }
            }
            println!();
        }
        println!("\n     a b c d e f g h\n");
        println!("     Side:     {:?}", self.side);
        println!("     Enpass:   {}", square_to_str(self.enpassant));
        println!(
            "     Castling: {}{}{}{}",
            if self.castle & CASTLE_WK != 0 { 'K' } else { '-' },
            if self.castle & CASTLE_WQ != 0 { 'Q' } else { '-' },
            if self.castle & CASTLE_BK != 0 { 'k' } else { '-' },
            if self.castle & CASTLE_BQ != 0 { 'q' } else { '-' }
        );
        println!("     Hash:     0x{:016x}", self.hash_key);
        println!();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_board_start_position() {
        let zobrist = ZobristKeys::new();
        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);

        assert_eq!(board.side, Color::White);
        assert_eq!(board.castle, CASTLE_WK | CASTLE_WQ | CASTLE_BK | CASTLE_BQ);
        assert_eq!(board.enpassant, square::NO_SQ);
        assert_eq!(count_bits(board.occupancies[Color::White as usize]), 16);
        assert_eq!(count_bits(board.occupancies[Color::Black as usize]), 16);
        assert_eq!(count_bits(board.occupancies[Color::Both as usize]), 32);
        assert_ne!(board.hash_key, 0);
    }

    #[test]
    fn test_board_tricky_position() {
        let zobrist = ZobristKeys::new();
        let mut board = Board::new();
        board.parse_fen(TRICKY_POSITION, &zobrist);

        assert_eq!(board.side, Color::White);
        assert_eq!(board.castle, CASTLE_WK | CASTLE_WQ | CASTLE_BK | CASTLE_BQ);
        assert_eq!(board.enpassant, square::NO_SQ);
    }
}
