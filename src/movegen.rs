//! Pseudo-legal move generation and move execution.

use crate::bitboard::*;
use crate::board::{Board, BoardState, CASTLING_RIGHTS};
use crate::magic::AttackTables;
use crate::moves::{Move, MoveList};
use crate::types::*;
use crate::zobrist::ZobristKeys;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MoveFlag {
    AllMoves,
    OnlyCaptures,
}

pub fn generate_moves(board: &Board, attack_tables: &AttackTables, move_list: &mut MoveList) {
    move_list.count = 0;

    let side = board.side;
    let occ_both = board.occupancies[Color::Both as usize];
    let occ_white = board.occupancies[Color::White as usize];
    let occ_black = board.occupancies[Color::Black as usize];

    if side == Color::White {
        // 1. White pawns
        let mut bitboard = board.bitboards[Piece::P as usize];
        while bitboard != 0 {
            let source_square = get_ls1b_index(bitboard).unwrap();
            pop_bit(&mut bitboard, source_square);

            // Target square 1 ahead
            let target_square = source_square as isize - 8;
            if target_square >= square::a8 as isize && !get_bit(occ_both, target_square as usize) {
                let target_sq = target_square as usize;
                // Pawn promotion
                if source_square >= square::a7 && source_square <= square::h7 {
                    move_list.add(Move::encode(source_square, target_sq, Piece::P, Some(Piece::Q), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_sq, Piece::P, Some(Piece::R), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_sq, Piece::P, Some(Piece::B), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_sq, Piece::P, Some(Piece::N), false, false, false, false));
                } else {
                    move_list.add(Move::encode(source_square, target_sq, Piece::P, None, false, false, false, false));

                    // Two squares ahead
                    if (source_square >= square::a2 && source_square <= square::h2)
                        && !get_bit(occ_both, target_sq - 8)
                    {
                        move_list.add(Move::encode(source_square, target_sq - 8, Piece::P, None, false, true, false, false));
                    }
                }
            }

            // Pawn captures
            let mut attacks = attack_tables.pawn_attacks[Color::White as usize][source_square] & occ_black;
            while attacks != 0 {
                let target_square = get_ls1b_index(attacks).unwrap();
                pop_bit(&mut attacks, target_square);

                if source_square >= square::a7 && source_square <= square::h7 {
                    move_list.add(Move::encode(source_square, target_square, Piece::P, Some(Piece::Q), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::P, Some(Piece::R), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::P, Some(Piece::B), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::P, Some(Piece::N), true, false, false, false));
                } else {
                    move_list.add(Move::encode(source_square, target_square, Piece::P, None, true, false, false, false));
                }
            }

            // En passant captures
            if board.enpassant != square::NO_SQ {
                let ep_attacks = attack_tables.pawn_attacks[Color::White as usize][source_square]
                    & (1u64 << board.enpassant);
                if ep_attacks != 0 {
                    let target_ep = get_ls1b_index(ep_attacks).unwrap();
                    move_list.add(Move::encode(source_square, target_ep, Piece::P, None, true, false, true, false));
                }
            }
        }

        // White king castling
        if (board.castle & CASTLE_WK) != 0 {
            if !get_bit(occ_both, square::f1) && !get_bit(occ_both, square::g1) {
                if !attack_tables.is_square_attacked(square::e1, Color::Black, &board.bitboards, occ_both)
                    && !attack_tables.is_square_attacked(square::f1, Color::Black, &board.bitboards, occ_both)
                {
                    move_list.add(Move::encode(square::e1, square::g1, Piece::K, None, false, false, false, true));
                }
            }
        }
        if (board.castle & CASTLE_WQ) != 0 {
            if !get_bit(occ_both, square::d1) && !get_bit(occ_both, square::c1) && !get_bit(occ_both, square::b1) {
                if !attack_tables.is_square_attacked(square::e1, Color::Black, &board.bitboards, occ_both)
                    && !attack_tables.is_square_attacked(square::d1, Color::Black, &board.bitboards, occ_both)
                {
                    move_list.add(Move::encode(square::e1, square::c1, Piece::K, None, false, false, false, true));
                }
            }
        }
    } else {
        // 2. Black pawns
        let mut bitboard = board.bitboards[Piece::PawnBlack as usize];
        while bitboard != 0 {
            let source_square = get_ls1b_index(bitboard).unwrap();
            pop_bit(&mut bitboard, source_square);

            let target_square = source_square + 8;
            if target_square <= square::h1 && !get_bit(occ_both, target_square) {
                if source_square >= square::a2 && source_square <= square::h2 {
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::QueenBlack), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::RookBlack), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::BishopBlack), false, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::KnightBlack), false, false, false, false));
                } else {
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, None, false, false, false, false));

                    if (source_square >= square::a7 && source_square <= square::h7)
                        && !get_bit(occ_both, target_square + 8)
                    {
                        move_list.add(Move::encode(source_square, target_square + 8, Piece::PawnBlack, None, false, true, false, false));
                    }
                }
            }

            let mut attacks = attack_tables.pawn_attacks[Color::Black as usize][source_square] & occ_white;
            while attacks != 0 {
                let target_square = get_ls1b_index(attacks).unwrap();
                pop_bit(&mut attacks, target_square);

                if source_square >= square::a2 && source_square <= square::h2 {
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::QueenBlack), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::RookBlack), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::BishopBlack), true, false, false, false));
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, Some(Piece::KnightBlack), true, false, false, false));
                } else {
                    move_list.add(Move::encode(source_square, target_square, Piece::PawnBlack, None, true, false, false, false));
                }
            }

            if board.enpassant != square::NO_SQ {
                let ep_attacks = attack_tables.pawn_attacks[Color::Black as usize][source_square]
                    & (1u64 << board.enpassant);
                if ep_attacks != 0 {
                    let target_ep = get_ls1b_index(ep_attacks).unwrap();
                    move_list.add(Move::encode(source_square, target_ep, Piece::PawnBlack, None, true, false, true, false));
                }
            }
        }

        // Black king castling
        if (board.castle & CASTLE_BK) != 0 {
            if !get_bit(occ_both, square::f8) && !get_bit(occ_both, square::g8) {
                if !attack_tables.is_square_attacked(square::e8, Color::White, &board.bitboards, occ_both)
                    && !attack_tables.is_square_attacked(square::f8, Color::White, &board.bitboards, occ_both)
                {
                    move_list.add(Move::encode(square::e8, square::g8, Piece::KingBlack, None, false, false, false, true));
                }
            }
        }
        if (board.castle & CASTLE_BQ) != 0 {
            if !get_bit(occ_both, square::d8) && !get_bit(occ_both, square::c8) && !get_bit(occ_both, square::b8) {
                if !attack_tables.is_square_attacked(square::e8, Color::White, &board.bitboards, occ_both)
                    && !attack_tables.is_square_attacked(square::d8, Color::White, &board.bitboards, occ_both)
                {
                    move_list.add(Move::encode(square::e8, square::c8, Piece::KingBlack, None, false, false, false, true));
                }
            }
        }
    }

    // Knights
    let knight_piece = if side == Color::White { Piece::N } else { Piece::KnightBlack };
    let mut bitboard = board.bitboards[knight_piece as usize];
    let same_occ = if side == Color::White { occ_white } else { occ_black };
    let opp_occ = if side == Color::White { occ_black } else { occ_white };

    while bitboard != 0 {
        let source_square = get_ls1b_index(bitboard).unwrap();
        pop_bit(&mut bitboard, source_square);

        let mut attacks = attack_tables.knight_attacks[source_square] & !same_occ;
        while attacks != 0 {
            let target_square = get_ls1b_index(attacks).unwrap();
            pop_bit(&mut attacks, target_square);

            let is_cap = get_bit(opp_occ, target_square);
            move_list.add(Move::encode(source_square, target_square, knight_piece, None, is_cap, false, false, false));
        }
    }

    // Bishops
    let bishop_piece = if side == Color::White { Piece::B } else { Piece::BishopBlack };
    let mut bitboard = board.bitboards[bishop_piece as usize];
    while bitboard != 0 {
        let source_square = get_ls1b_index(bitboard).unwrap();
        pop_bit(&mut bitboard, source_square);

        let mut attacks = attack_tables.get_bishop_attacks(source_square, occ_both) & !same_occ;
        while attacks != 0 {
            let target_square = get_ls1b_index(attacks).unwrap();
            pop_bit(&mut attacks, target_square);

            let is_cap = get_bit(opp_occ, target_square);
            move_list.add(Move::encode(source_square, target_square, bishop_piece, None, is_cap, false, false, false));
        }
    }

    // Rooks
    let rook_piece = if side == Color::White { Piece::R } else { Piece::RookBlack };
    let mut bitboard = board.bitboards[rook_piece as usize];
    while bitboard != 0 {
        let source_square = get_ls1b_index(bitboard).unwrap();
        pop_bit(&mut bitboard, source_square);

        let mut attacks = attack_tables.get_rook_attacks(source_square, occ_both) & !same_occ;
        while attacks != 0 {
            let target_square = get_ls1b_index(attacks).unwrap();
            pop_bit(&mut attacks, target_square);

            let is_cap = get_bit(opp_occ, target_square);
            move_list.add(Move::encode(source_square, target_square, rook_piece, None, is_cap, false, false, false));
        }
    }

    // Queens
    let queen_piece = if side == Color::White { Piece::Q } else { Piece::QueenBlack };
    let mut bitboard = board.bitboards[queen_piece as usize];
    while bitboard != 0 {
        let source_square = get_ls1b_index(bitboard).unwrap();
        pop_bit(&mut bitboard, source_square);

        let mut attacks = attack_tables.get_queen_attacks(source_square, occ_both) & !same_occ;
        while attacks != 0 {
            let target_square = get_ls1b_index(attacks).unwrap();
            pop_bit(&mut attacks, target_square);

            let is_cap = get_bit(opp_occ, target_square);
            move_list.add(Move::encode(source_square, target_square, queen_piece, None, is_cap, false, false, false));
        }
    }

    // Kings
    let king_piece = if side == Color::White { Piece::K } else { Piece::KingBlack };
    let mut bitboard = board.bitboards[king_piece as usize];
    while bitboard != 0 {
        let source_square = get_ls1b_index(bitboard).unwrap();
        pop_bit(&mut bitboard, source_square);

        let mut attacks = attack_tables.king_attacks[source_square] & !same_occ;
        while attacks != 0 {
            let target_square = get_ls1b_index(attacks).unwrap();
            pop_bit(&mut attacks, target_square);

            let is_cap = get_bit(opp_occ, target_square);
            move_list.add(Move::encode(source_square, target_square, king_piece, None, is_cap, false, false, false));
        }
    }
}

pub fn make_move(
    board: &mut Board,
    m: Move,
    move_flag: MoveFlag,
    attack_tables: &AttackTables,
    zobrist: &ZobristKeys,
) -> bool {
    if move_flag == MoveFlag::OnlyCaptures && !m.is_capture() {
        return false;
    }

    let state: BoardState = board.save_state();

    let source = m.source();
    let target = m.target();
    let piece = m.piece();
    let promoted = m.promoted();
    let capture = m.is_capture();
    let double_push = m.is_double_push();
    let enpass = m.is_enpassant();
    let castling = m.is_castling();

    // Move piece
    pop_bit(&mut board.bitboards[piece as usize], source);
    set_bit(&mut board.bitboards[piece as usize], target);

    board.hash_key ^= zobrist.piece_keys[piece as usize][source];
    board.hash_key ^= zobrist.piece_keys[piece as usize][target];

    // Handle captures
    if capture {
        let (start_p, end_p) = if board.side == Color::White {
            (Piece::PawnBlack as usize, Piece::KingBlack as usize)
        } else {
            (Piece::P as usize, Piece::K as usize)
        };

        for bb_piece in start_p..=end_p {
            if get_bit(board.bitboards[bb_piece], target) {
                pop_bit(&mut board.bitboards[bb_piece], target);
                board.hash_key ^= zobrist.piece_keys[bb_piece][target];
                break;
            }
        }
    }

    // Handle pawn promotions
    if let Some(prom) = promoted {
        if board.side == Color::White {
            pop_bit(&mut board.bitboards[Piece::P as usize], target);
            board.hash_key ^= zobrist.piece_keys[Piece::P as usize][target];
        } else {
            pop_bit(&mut board.bitboards[Piece::PawnBlack as usize], target);
            board.hash_key ^= zobrist.piece_keys[Piece::PawnBlack as usize][target];
        }
        set_bit(&mut board.bitboards[prom as usize], target);
        board.hash_key ^= zobrist.piece_keys[prom as usize][target];
    }

    // Handle en passant captures
    if enpass {
        if board.side == Color::White {
            pop_bit(&mut board.bitboards[Piece::PawnBlack as usize], target + 8);
            board.hash_key ^= zobrist.piece_keys[Piece::PawnBlack as usize][target + 8];
        } else {
            pop_bit(&mut board.bitboards[Piece::P as usize], target - 8);
            board.hash_key ^= zobrist.piece_keys[Piece::P as usize][target - 8];
        }
    }

    // Reset enpassant square
    if board.enpassant != square::NO_SQ {
        board.hash_key ^= zobrist.enpassant_keys[board.enpassant];
    }
    board.enpassant = square::NO_SQ;

    // Handle double pawn push
    if double_push {
        if board.side == Color::White {
            board.enpassant = target + 8;
            board.hash_key ^= zobrist.enpassant_keys[target + 8];
        } else {
            board.enpassant = target - 8;
            board.hash_key ^= zobrist.enpassant_keys[target - 8];
        }
    }

    // Handle castling moves
    if castling {
        match target {
            square::g1 => {
                pop_bit(&mut board.bitboards[Piece::R as usize], square::h1);
                set_bit(&mut board.bitboards[Piece::R as usize], square::f1);
                board.hash_key ^= zobrist.piece_keys[Piece::R as usize][square::h1];
                board.hash_key ^= zobrist.piece_keys[Piece::R as usize][square::f1];
            }
            square::c1 => {
                pop_bit(&mut board.bitboards[Piece::R as usize], square::a1);
                set_bit(&mut board.bitboards[Piece::R as usize], square::d1);
                board.hash_key ^= zobrist.piece_keys[Piece::R as usize][square::a1];
                board.hash_key ^= zobrist.piece_keys[Piece::R as usize][square::d1];
            }
            square::g8 => {
                pop_bit(&mut board.bitboards[Piece::RookBlack as usize], square::h8);
                set_bit(&mut board.bitboards[Piece::RookBlack as usize], square::f8);
                board.hash_key ^= zobrist.piece_keys[Piece::RookBlack as usize][square::h8];
                board.hash_key ^= zobrist.piece_keys[Piece::RookBlack as usize][square::f8];
            }
            square::c8 => {
                pop_bit(&mut board.bitboards[Piece::RookBlack as usize], square::a8);
                set_bit(&mut board.bitboards[Piece::RookBlack as usize], square::d8);
                board.hash_key ^= zobrist.piece_keys[Piece::RookBlack as usize][square::a8];
                board.hash_key ^= zobrist.piece_keys[Piece::RookBlack as usize][square::d8];
            }
            _ => {}
        }
    }

    // Update castling rights
    board.hash_key ^= zobrist.castle_keys[board.castle as usize];
    board.castle &= CASTLING_RIGHTS[source];
    board.castle &= CASTLING_RIGHTS[target];
    board.hash_key ^= zobrist.castle_keys[board.castle as usize];

    // Update occupancies
    board.update_occupancies();

    // Change side
    board.side = board.side.opposite();
    board.hash_key ^= zobrist.side_key;

    // Check if king has been exposed to check
    let king_sq = if board.side == Color::Black {
        get_ls1b_index(board.bitboards[Piece::K as usize]).unwrap()
    } else {
        get_ls1b_index(board.bitboards[Piece::KingBlack as usize]).unwrap()
    };

    if attack_tables.is_square_attacked(
        king_sq,
        board.side,
        &board.bitboards,
        board.occupancies[Color::Both as usize],
    ) {
        board.restore_state(&state);
        return false;
    }

    true
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::START_POSITION;

    #[test]
    fn test_start_pos_move_count() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);

        let mut move_list = MoveList::new();
        generate_moves(&board, &attack_tables, &mut move_list);
        assert_eq!(move_list.count, 20);

        // Make all moves and verify all 20 are legal
        let mut legal_count = 0;
        for i in 0..move_list.count {
            let state = board.save_state();
            if make_move(&mut board, move_list.get(i), MoveFlag::AllMoves, &attack_tables, &zobrist) {
                legal_count += 1;
                board.restore_state(&state);
            }
        }
        assert_eq!(legal_count, 20);
    }
}
