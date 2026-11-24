//! 32-bit move encoding and move list data structures.

use crate::types::{square_to_str, Piece};

/// 32-bit move encoding:
/// 0000 0000 0000 0000 0011 1111    source square       0x3f
/// 0000 0000 0000 1111 1100 0000    target square       0xfc0
/// 0000 0000 1111 0000 0000 0000    piece               0xf000
/// 0000 1111 0000 0000 0000 0000    promoted piece      0xf0000
/// 0001 0000 0000 0000 0000 0000    capture flag        0x100000
/// 0010 0000 0000 0000 0000 0000    double push flag    0x200000
/// 0100 0000 0000 0000 0000 0000    enpassant flag      0x400000
/// 1000 0000 0000 0000 0000 0000    castling flag       0x800000
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub struct Move(pub u32);

impl Move {
    pub const NULL: Move = Move(0);

    #[inline(always)]
    pub fn encode(
        source: usize,
        target: usize,
        piece: Piece,
        promoted: Option<Piece>,
        capture: bool,
        double_push: bool,
        enpassant: bool,
        castling: bool,
    ) -> Self {
        let piece_code = piece as u32;
        let promoted_code = promoted.map_or(0, |p| p as u32);
        let cap = if capture { 1 } else { 0 };
        let dbl = if double_push { 1 } else { 0 };
        let ep = if enpassant { 1 } else { 0 };
        let cst = if castling { 1 } else { 0 };

        let val = (source as u32)
            | ((target as u32) << 6)
            | (piece_code << 12)
            | (promoted_code << 16)
            | (cap << 20)
            | (dbl << 21)
            | (ep << 22)
            | (cst << 23);
        Move(val)
    }

    #[inline(always)]
    pub fn source(self) -> usize {
        (self.0 & 0x3f) as usize
    }

    #[inline(always)]
    pub fn target(self) -> usize {
        ((self.0 & 0xfc0) >> 6) as usize
    }

    #[inline(always)]
    pub fn piece(self) -> Piece {
        unsafe { std::mem::transmute(((self.0 & 0xf000) >> 12) as u8) }
    }

    #[inline(always)]
    pub fn promoted(self) -> Option<Piece> {
        let code = ((self.0 & 0xf0000) >> 16) as u8;
        if code != 0 {
            Some(unsafe { std::mem::transmute(code) })
        } else {
            None
        }
    }

    #[inline(always)]
    pub fn is_capture(self) -> bool {
        (self.0 & 0x100000) != 0
    }

    #[inline(always)]
    pub fn is_double_push(self) -> bool {
        (self.0 & 0x200000) != 0
    }

    #[inline(always)]
    pub fn is_enpassant(self) -> bool {
        (self.0 & 0x400000) != 0
    }

    #[inline(always)]
    pub fn is_castling(self) -> bool {
        (self.0 & 0x800000) != 0
    }

    pub fn to_uci(self) -> String {
        let mut s = format!("{}{}", square_to_str(self.source()), square_to_str(self.target()));
        if let Some(prom) = self.promoted() {
            let prom_char = match prom {
                Piece::Q | Piece::QueenBlack => 'q',
                Piece::R | Piece::RookBlack => 'r',
                Piece::B | Piece::BishopBlack => 'b',
                Piece::N | Piece::KnightBlack => 'n',
                _ => ' ',
            };
            if prom_char != ' ' {
                s.push(prom_char);
            }
        }
        s
    }
}

pub struct MoveList {
    pub moves: [Move; 256],
    pub count: usize,
}

impl MoveList {
    pub fn new() -> Self {
        Self {
            moves: [Move::NULL; 256],
            count: 0,
        }
    }

    #[inline(always)]
    pub fn add(&mut self, m: Move) {
        self.moves[self.count] = m;
        self.count += 1;
    }

    #[inline(always)]
    pub fn get(&self, index: usize) -> Move {
        self.moves[index]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::square;

    #[test]
    fn test_move_encoding_decoding() {
        let m = Move::encode(
            square::e2,
            square::e4,
            Piece::P,
            None,
            false,
            true,
            false,
            false,
        );
        assert_eq!(m.source(), square::e2);
        assert_eq!(m.target(), square::e4);
        assert_eq!(m.piece(), Piece::P);
        assert_eq!(m.promoted(), None);
        assert_eq!(m.is_capture(), false);
        assert_eq!(m.is_double_push(), true);
        assert_eq!(m.is_enpassant(), false);
        assert_eq!(m.is_castling(), false);
        assert_eq!(m.to_uci(), "e2e4");
    }

    #[test]
    fn test_promotion_move() {
        let m = Move::encode(
            square::e7,
            square::e8,
            Piece::P,
            Some(Piece::Q),
            false,
            false,
            false,
            false,
        );
        assert_eq!(m.source(), square::e7);
        assert_eq!(m.target(), square::e8);
        assert_eq!(m.promoted(), Some(Piece::Q));
        assert_eq!(m.to_uci(), "e7e8q");
    }
}
