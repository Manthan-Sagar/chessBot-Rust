//! Static Exchange Evaluation (SEE).
//! Simulates capture and recapture sequences on a target square using ray and leaper attacks
//! to determine the net material outcome without full tree search.

use crate::bitboard::*;
use crate::board::Board;
use crate::magic::AttackTables;
use crate::moves::Move;
use crate::types::*;

pub const SEE_PAWN: i32 = 100;
pub const SEE_KNIGHT: i32 = 320;
pub const SEE_BISHOP: i32 = 330;
pub const SEE_ROOK: i32 = 500;
pub const SEE_QUEEN: i32 = 900;
pub const SEE_KING: i32 = 20000;

#[inline(always)]
pub fn piece_value(p: Piece) -> i32 {
    match p {
        Piece::P | Piece::PawnBlack => SEE_PAWN,
        Piece::N | Piece::KnightBlack => SEE_KNIGHT,
        Piece::B | Piece::BishopBlack => SEE_BISHOP,
        Piece::R | Piece::RookBlack => SEE_ROOK,
        Piece::Q | Piece::QueenBlack => SEE_QUEEN,
        Piece::K | Piece::KingBlack => SEE_KING,
    }
}

/// Finds the least valuable attacker of `target` for `side` within `occupied` bitboard.
fn get_least_valuable_attacker(
    board: &Board,
    side: Color,
    target: usize,
    occupied: u64,
    attack_tables: &AttackTables,
) -> Option<(usize, i32, Piece)> {
    let (pawn_p, knight_p, bishop_p, rook_p, queen_p, king_p) = if side == Color::White {
        (Piece::P, Piece::N, Piece::B, Piece::R, Piece::Q, Piece::K)
    } else {
        (
            Piece::PawnBlack,
            Piece::KnightBlack,
            Piece::BishopBlack,
            Piece::RookBlack,
            Piece::QueenBlack,
            Piece::KingBlack,
        )
    };

    // 1. Pawn
    let pawn_attacks = if side == Color::White {
        attack_tables.pawn_attacks[Color::Black as usize][target]
    } else {
        attack_tables.pawn_attacks[Color::White as usize][target]
    };
    let pawns = pawn_attacks & board.bitboards[pawn_p as usize] & occupied;
    if pawns != 0 {
        let sq = get_ls1b_index(pawns).unwrap();
        return Some((sq, SEE_PAWN, pawn_p));
    }

    // 2. Knight
    let knights = attack_tables.knight_attacks[target] & board.bitboards[knight_p as usize] & occupied;
    if knights != 0 {
        let sq = get_ls1b_index(knights).unwrap();
        return Some((sq, SEE_KNIGHT, knight_p));
    }

    // 3. Bishop
    let bishops = attack_tables.get_bishop_attacks(target, occupied) & board.bitboards[bishop_p as usize] & occupied;
    if bishops != 0 {
        let sq = get_ls1b_index(bishops).unwrap();
        return Some((sq, SEE_BISHOP, bishop_p));
    }

    // 4. Rook
    let rooks = attack_tables.get_rook_attacks(target, occupied) & board.bitboards[rook_p as usize] & occupied;
    if rooks != 0 {
        let sq = get_ls1b_index(rooks).unwrap();
        return Some((sq, SEE_ROOK, rook_p));
    }

    // 5. Queen
    let queens = attack_tables.get_queen_attacks(target, occupied) & board.bitboards[queen_p as usize] & occupied;
    if queens != 0 {
        let sq = get_ls1b_index(queens).unwrap();
        return Some((sq, SEE_QUEEN, queen_p));
    }

    // 6. King
    let kings = attack_tables.king_attacks[target] & board.bitboards[king_p as usize] & occupied;
    if kings != 0 {
        let king_sq = get_ls1b_index(kings).unwrap();
        // King may only capture if target square is not defended by opponent
        let occ_without_king = occupied ^ (1u64 << king_sq);
        let attacked_by_opp = attack_tables.is_square_attacked(
            target,
            side.opposite(),
            &board.bitboards,
            occ_without_king,
        );
        if !attacked_by_opp {
            return Some((king_sq, SEE_KING, king_p));
        }
    }

    None
}

/// Evaluates the net material gain of `m` via Static Exchange Evaluation.
pub fn see_eval(board: &Board, m: Move, attack_tables: &AttackTables) -> i32 {
    let source = m.source();
    let target = m.target();

    let mut victim_val = 0;
    if m.is_enpassant() {
        victim_val = SEE_PAWN;
    } else if m.is_capture() {
        let (start_p, end_p) = if board.side == Color::White {
            (Piece::PawnBlack as usize, Piece::KingBlack as usize)
        } else {
            (Piece::P as usize, Piece::K as usize)
        };
        for p in start_p..=end_p {
            if get_bit(board.bitboards[p], target) {
                victim_val = match p {
                    0 | 6 => SEE_PAWN,
                    1 | 7 => SEE_KNIGHT,
                    2 | 8 => SEE_BISHOP,
                    3 | 9 => SEE_ROOK,
                    4 | 10 => SEE_QUEEN,
                    5 | 11 => SEE_KING,
                    _ => 0,
                };
                break;
            }
        }
    }

    if let Some(prom) = m.promoted() {
        victim_val += piece_value(prom) - SEE_PAWN;
    }

    let mut gain = [0i32; 32];
    gain[0] = victim_val;

    let mut current_piece_on_target_val = if let Some(prom) = m.promoted() {
        piece_value(prom)
    } else {
        piece_value(m.piece())
    };

    let mut occupied = board.occupancies[Color::Both as usize] ^ (1u64 << source);
    if m.is_enpassant() {
        let ep_sq = if board.side == Color::White {
            target + 8
        } else {
            target - 8
        };
        occupied &= !(1u64 << ep_sq);
    }

    let mut side = board.side.opposite();
    let mut d = 0;

    while d < 31 {
        let (sq, val, _) = match get_least_valuable_attacker(board, side, target, occupied, attack_tables) {
            Some(x) => x,
            None => break,
        };
        d += 1;
        gain[d] = current_piece_on_target_val;
        current_piece_on_target_val = val;
        occupied &= !(1u64 << sq);
        side = side.opposite();
    }

    let mut score = 0;
    while d > 0 {
        score = (gain[d] - score).max(0);
        d -= 1;
    }

    gain[0] - score
}

/// Returns true if Static Exchange Evaluation of `m` is >= `threshold`.
#[inline(always)]
pub fn see(board: &Board, m: Move, threshold: i32, attack_tables: &AttackTables) -> bool {
    see_eval(board, m, attack_tables) >= threshold
}
