//! Static evaluation: material, piece-square tables, pawn structure, mobility, and king safety.

use crate::bitboard::*;
use crate::board::Board;
use crate::magic::AttackTables;
use crate::types::*;

pub const OPENING: usize = 0;
pub const ENDGAME: usize = 1;

pub const PAWN: usize = 0;
pub const KNIGHT: usize = 1;
pub const BISHOP: usize = 2;
pub const ROOK: usize = 3;
pub const QUEEN: usize = 4;
pub const KING: usize = 5;

pub const MATERIAL_SCORE: [[i32; 12]; 2] = [
    // Opening material score
    [82, 337, 365, 477, 1025, 12000, -82, -337, -365, -477, -1025, -12000],
    // Endgame material score
    [94, 281, 297, 512,  936, 12000, -94, -281, -297, -512,  -936, -12000],
];

pub const POSITIONAL_SCORE: [[[i32; 64]; 6]; 2] = [
    // Opening positional piece scores
    [
        // Pawn
        [
            0,   0,   0,   0,   0,   0,  0,   0,
            98, 134,  61,  95,  68, 126, 34, -11,
            -6,   7,  26,  31,  65,  56, 25, -20,
            -14,  13,   6,  21,  23,  12, 17, -23,
            -27,  -2,  -5,  12,  17,   6, 10, -25,
            -26,  -4,  -4, -10,   3,   3, 33, -12,
            -35,  -1, -20, -23, -15,  24, 38, -22,
            0,   0,   0,   0,   0,   0,  0,   0,
        ],
        // Knight
        [
            -167, -89, -34, -49,  61, -97, -15, -107,
            -73, -41,  72,  36,  23,  62,   7,  -17,
            -47,  60,  37,  65,  84, 129,  73,   44,
            -9,  17,  19,  53,  37,  69,  18,   22,
            -13,   4,  16,  13,  28,  19,  21,   -8,
            -23,  -9,  12,  10,  19,  17,  25,  -16,
            -29, -53, -12,  -3,  -1,  18, -14,  -19,
            -105, -21, -58, -33, -17, -28, -19,  -23,
        ],
        // Bishop
        [
            -29,   4, -82, -37, -25, -42,   7,  -8,
            -26,  16, -18, -13,  30,  59,  18, -47,
            -16,  37,  43,  40,  35,  50,  37,  -2,
            -4,   5,  19,  50,  37,  37,   7,  -2,
            -6,  13,  13,  26,  34,  12,  10,   4,
            0,  15,  15,  15,  14,  27,  18,  10,
            4,  15,  16,   0,   7,  21,  33,   1,
            -33,  -3, -14, -21, -13, -12, -39, -21,
        ],
        // Rook
        [
            32,  42,  32,  51, 63,  9,  31,  43,
            27,  32,  58,  62, 80, 67,  26,  44,
            -5,  19,  26,  36, 17, 45,  61,  16,
            -24, -11,   7,  26, 24, 35,  -8, -20,
            -36, -26, -12,  -1,  9, -7,   6, -23,
            -45, -25, -16, -17,  3,  0,  -5, -33,
            -44, -16, -20,  -9, -1, 11,  -6, -71,
            -19, -13,   1,  17, 16,  7, -37, -26,
        ],
        // Queen
        [
            -28,   0,  29,  12,  59,  44,  43,  45,
            -24, -39,  -5,   1, -16,  57,  28,  54,
            -13, -17,   7,   8,  29,  56,  47,  57,
            -27, -27, -16, -16,  -1,  17,  -2,   1,
            -9, -26,  -9, -10,  -2,  -4,   3,  -3,
            -14,   2, -11,  -2,  -5,   2,  14,   5,
            -35,  -8,  11,   2,   8,  15,  -3,   1,
            -1, -18,  -9,  10, -15, -25, -31, -50,
        ],
        // King
        [
            -65,  23,  16, -15, -56, -34,   2,  13,
            29,  -1, -20,  -7,  -8,  -4, -38, -29,
            -9,  24,   2, -16, -20,   6,  22, -22,
            -17, -20, -12, -27, -30, -25, -14, -36,
            -49,  -1, -27, -39, -46, -44, -33, -51,
            -14, -14, -22, -46, -44, -30, -15, -27,
            1,   7,  -8, -64, -43, -16,   9,   8,
            -15,  36,  12, -54,   8, -28,  24,  14,
        ],
    ],
    // Endgame positional piece scores
    [
        // Pawn
        [
            0,   0,   0,   0,   0,   0,   0,   0,
            178, 173, 158, 134, 147, 132, 165, 187,
            94, 100,  85,  67,  56,  53,  82,  84,
            32,  24,  13,   5,  -2,   4,  17,  17,
            13,   9,  -3,  -7,  -7,  -8,   3,  -1,
            4,   7,  -6,   1,   0,  -5,  -1,  -8,
            13,   8,   8,  10,  13,   0,   2,  -7,
            0,   0,   0,   0,   0,   0,   0,   0,
        ],
        // Knight
        [
            -58, -38, -13, -28, -31, -27, -63, -99,
            -25,  -8, -25,  -2,  -9, -25, -24, -52,
            -24, -20,  10,   9,  -1,  -9, -19, -41,
            -17,   3,  22,  22,  22,  11,   8, -18,
            -18,  -6,  16,  25,  16,  17,   4, -18,
            -23,  -3,  -1,  15,  10,  -3, -20, -22,
            -42, -20, -10,  -5,  -2, -20, -23, -44,
            -29, -51, -23, -15, -22, -18, -50, -64,
        ],
        // Bishop
        [
            -14, -21, -11,  -8, -7,  -9, -17, -24,
            -8,  -4,   7, -12, -3, -13,  -4, -14,
            2,  -8,   0,  -1, -2,   6,   0,   4,
            -3,   9,  12,   9, 14,  10,   3,   2,
            -6,   3,  13,  19,  7,  10,  -3,  -9,
            -12,  -3,   8,  10, 13,   3,  -7, -15,
            -14, -18,  -7,  -1,  4,  -9, -15, -27,
            -23,  -9, -23,  -5, -9, -16,  -5, -17,
        ],
        // Rook
        [
            13, 10, 18, 15, 12,  12,   8,   5,
            11, 13, 13, 11, -3,   3,   8,   3,
            7,  7,  7,  5,  4,  -3,  -5,  -3,
            4,  3, 13,  1,  2,   1,  -1,   2,
            3,  5,  8,  4, -5,  -6,  -8, -11,
            -4,  0, -5, -1, -7, -12,  -8, -16,
            -6, -6,  0,  2, -9,  -9, -11,  -3,
            -9,  2,  3, -1, -5, -13,   4, -20,
        ],
        // Queen
        [
            -9,  22,  22,  27,  27,  19,  10,  20,
            -17,  20,  32,  41,  58,  25,  30,   0,
            -20,   6,   9,  49,  47,  35,  19,   9,
            3,  22,  24,  45,  57,  40,  57,  36,
            -18,  28,  19,  47,  31,  34,  39,  23,
            -16, -27,  15,   6,   9,  17,  10,   5,
            -22, -23, -30, -16, -16, -23, -36, -32,
            -33, -28, -22, -43,  -5, -32, -20, -41,
        ],
        // King
        [
            -74, -35, -18, -18, -11,  15,   4, -17,
            -12,  17,  14,  17,  17,  38,  23,  11,
            10,  17,  23,  15,  20,  45,  44,  13,
            -8,  22,  24,  27,  26,  33,  26,   3,
            -18,  -4,  21,  24,  27,  23,   9, -11,
            -19,  -3,  11,  21,  23,  16,   7,  -9,
            -27, -11,   4,  13,  14,   4,  -5, -17,
            -53, -34, -21, -11, -28, -14, -24, -43,
        ],
    ],
];

pub const MIRROR_SCORE: [usize; 64] = [
    56, 57, 58, 59, 60, 61, 62, 63,
    48, 49, 50, 51, 52, 53, 54, 55,
    40, 41, 42, 43, 44, 45, 46, 47,
    32, 33, 34, 35, 36, 37, 38, 39,
    24, 25, 26, 27, 28, 29, 30, 31,
    16, 17, 18, 19, 20, 21, 22, 23,
     8,  9, 10, 11, 12, 13, 14, 15,
     0,  1,  2,  3,  4,  5,  6,  7,
];

pub const GET_RANK: [usize; 64] = [
    7, 7, 7, 7, 7, 7, 7, 7,
    6, 6, 6, 6, 6, 6, 6, 6,
    5, 5, 5, 5, 5, 5, 5, 5,
    4, 4, 4, 4, 4, 4, 4, 4,
    3, 3, 3, 3, 3, 3, 3, 3,
    2, 2, 2, 2, 2, 2, 2, 2,
    1, 1, 1, 1, 1, 1, 1, 1,
    0, 0, 0, 0, 0, 0, 0, 0,
];

pub const DOUBLE_PAWN_PENALTY_OPENING: i32 = -5;
pub const DOUBLE_PAWN_PENALTY_ENDGAME: i32 = -10;
pub const ISOLATED_PAWN_PENALTY_OPENING: i32 = -5;
pub const ISOLATED_PAWN_PENALTY_ENDGAME: i32 = -10;
pub const PASSED_PAWN_BONUS: [i32; 8] = [0, 10, 30, 50, 75, 100, 150, 200];
pub const SEMI_OPEN_FILE_SCORE: i32 = 10;
pub const OPEN_FILE_SCORE: i32 = 15;

pub const BISHOP_UNIT: i32 = 4;
pub const QUEEN_UNIT: i32 = 9;
pub const BISHOP_MOBILITY_OPENING: i32 = 5;
pub const BISHOP_MOBILITY_ENDGAME: i32 = 5;
pub const QUEEN_MOBILITY_OPENING: i32 = 1;
pub const QUEEN_MOBILITY_ENDGAME: i32 = 2;
pub const KING_SHIELD_BONUS: i32 = 5;

pub struct EvalMasks {
    pub file_masks: [Bitboard; 64],
    pub rank_masks: [Bitboard; 64],
    pub isolated_masks: [Bitboard; 64],
    pub white_passed_masks: [Bitboard; 64],
    pub black_passed_masks: [Bitboard; 64],
}

impl EvalMasks {
    pub fn new() -> Self {
        let set_file_rank_mask = |file_number: i32, rank_number: i32| -> Bitboard {
            let mut mask: Bitboard = 0;
            for r in 0..8 {
                for f in 0..8 {
                    let sq = (r * 8 + f) as usize;
                    if file_number != -1 {
                        if f == file_number {
                            set_bit(&mut mask, sq);
                        }
                    } else if rank_number != -1 {
                        if r == rank_number {
                            set_bit(&mut mask, sq);
                        }
                    }
                }
            }
            mask
        };

        let mut file_masks = [0u64; 64];
        let mut rank_masks = [0u64; 64];
        let mut isolated_masks = [0u64; 64];
        let mut white_passed_masks = [0u64; 64];
        let mut black_passed_masks = [0u64; 64];

        for r in 0..8 {
            for f in 0..8 {
                let sq = r * 8 + f;
                file_masks[sq] = set_file_rank_mask(f as i32, -1);
                rank_masks[sq] = set_file_rank_mask(-1, r as i32);

                isolated_masks[sq] |= set_file_rank_mask(f as i32 - 1, -1);
                isolated_masks[sq] |= set_file_rank_mask(f as i32 + 1, -1);

                white_passed_masks[sq] |= set_file_rank_mask(f as i32 - 1, -1);
                white_passed_masks[sq] |= set_file_rank_mask(f as i32, -1);
                white_passed_masks[sq] |= set_file_rank_mask(f as i32 + 1, -1);
                for i in 0..(8 - r) {
                    white_passed_masks[sq] &= !rank_masks[(7 - i) * 8 + f];
                }

                black_passed_masks[sq] |= set_file_rank_mask(f as i32 - 1, -1);
                black_passed_masks[sq] |= set_file_rank_mask(f as i32, -1);
                black_passed_masks[sq] |= set_file_rank_mask(f as i32 + 1, -1);
                for i in 0..=(r as usize) {
                    black_passed_masks[sq] &= !rank_masks[i * 8 + f];
                }
            }
        }

        Self {
            file_masks,
            rank_masks,
            isolated_masks,
            white_passed_masks,
            black_passed_masks,
        }
    }
}

#[inline(always)]
pub fn get_game_phase(board: &Board) -> i32 {
    let mut phase = 0;
    phase += (count_bits(board.bitboards[Piece::N as usize]) + count_bits(board.bitboards[Piece::KnightBlack as usize])) as i32 * 1;
    phase += (count_bits(board.bitboards[Piece::B as usize]) + count_bits(board.bitboards[Piece::BishopBlack as usize])) as i32 * 1;
    phase += (count_bits(board.bitboards[Piece::R as usize]) + count_bits(board.bitboards[Piece::RookBlack as usize])) as i32 * 2;
    phase += (count_bits(board.bitboards[Piece::Q as usize]) + count_bits(board.bitboards[Piece::QueenBlack as usize])) as i32 * 4;
    phase.min(24)
}

#[inline(always)]
fn chebyshev_distance(sq1: usize, sq2: usize) -> i32 {
    let r1 = (sq1 / 8) as i32;
    let f1 = (sq1 % 8) as i32;
    let r2 = (sq2 / 8) as i32;
    let f2 = (sq2 % 8) as i32;
    (r1 - r2).abs().max((f1 - f2).abs())
}

pub fn evaluate(board: &Board, attack_tables: &AttackTables, masks: &EvalMasks) -> i32 {
    let phase = get_game_phase(board);

    let mut score_opening: i32 = 0;
    let mut score_endgame: i32 = 0;
    let occ_both = board.occupancies[Color::Both as usize];

    // Bishop pair bonuses
    if count_bits(board.bitboards[Piece::B as usize]) >= 2 {
        score_opening += 35;
        score_endgame += 35;
    }
    if count_bits(board.bitboards[Piece::BishopBlack as usize]) >= 2 {
        score_opening -= 35;
        score_endgame -= 35;
    }

    // Pawn attack zones for knight mobility safety
    let mut white_pawn_attacks = 0u64;
    let mut wp = board.bitboards[Piece::P as usize];
    while wp != 0 {
        let sq = get_ls1b_index(wp).unwrap();
        pop_bit(&mut wp, sq);
        white_pawn_attacks |= attack_tables.pawn_attacks[Color::White as usize][sq];
    }

    let mut black_pawn_attacks = 0u64;
    let mut bp = board.bitboards[Piece::PawnBlack as usize];
    while bp != 0 {
        let sq = get_ls1b_index(bp).unwrap();
        pop_bit(&mut bp, sq);
        black_pawn_attacks |= attack_tables.pawn_attacks[Color::Black as usize][sq];
    }

    let white_king_sq = get_ls1b_index(board.bitboards[Piece::K as usize]).unwrap_or(60);
    let black_king_sq = get_ls1b_index(board.bitboards[Piece::KingBlack as usize]).unwrap_or(4);

    for bb_piece in 0..12 {
        let mut bitboard = board.bitboards[bb_piece];
        while bitboard != 0 {
            let square = get_ls1b_index(bitboard).unwrap();
            pop_bit(&mut bitboard, square);

            score_opening += MATERIAL_SCORE[OPENING][bb_piece];
            score_endgame += MATERIAL_SCORE[ENDGAME][bb_piece];

            match bb_piece {
                0 => { // White Pawn (P)
                    score_opening += POSITIONAL_SCORE[OPENING][PAWN][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][PAWN][square];

                    let double_pawns = count_bits(board.bitboards[Piece::P as usize] & masks.file_masks[square]);
                    if double_pawns > 1 {
                        score_opening += (double_pawns as i32 - 1) * DOUBLE_PAWN_PENALTY_OPENING;
                        score_endgame += (double_pawns as i32 - 1) * DOUBLE_PAWN_PENALTY_ENDGAME;
                    }

                    if (board.bitboards[Piece::P as usize] & masks.isolated_masks[square]) == 0 {
                        score_opening += ISOLATED_PAWN_PENALTY_OPENING;
                        score_endgame += ISOLATED_PAWN_PENALTY_ENDGAME;
                    }

                    let is_passed = (masks.white_passed_masks[square] & board.bitboards[Piece::PawnBlack as usize]) == 0;
                    if is_passed {
                        score_opening += PASSED_PAWN_BONUS[GET_RANK[square]];
                        score_endgame += PASSED_PAWN_BONUS[GET_RANK[square]];

                        // Protected passed pawn bonus
                        if (attack_tables.pawn_attacks[Color::Black as usize][square] & board.bitboards[Piece::P as usize]) != 0 {
                            score_opening += 20;
                            score_endgame += 35;
                        }

                        // Endgame king proximity to passed pawn
                        if phase < 14 {
                            let wk_dist = chebyshev_distance(white_king_sq, square);
                            let bk_dist = chebyshev_distance(black_king_sq, square);
                            score_endgame += (bk_dist - wk_dist) * 6;
                        }
                    }
                }
                1 => { // White Knight (N)
                    score_opening += POSITIONAL_SCORE[OPENING][KNIGHT][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][KNIGHT][square];

                    // Knight mobility
                    let safe_sqs = attack_tables.knight_attacks[square] & !black_pawn_attacks & !board.occupancies[Color::White as usize];
                    let mob = count_bits(safe_sqs) as i32 - 4;
                    score_opening += mob * 4;
                    score_endgame += mob * 4;

                    // Knight Outpost on ranks 4-6 (r in 2..=4) supported by own pawn
                    let r = square / 8;
                    if r >= 2 && r <= 4 {
                        let supported = (attack_tables.pawn_attacks[Color::Black as usize][square] & board.bitboards[Piece::P as usize]) != 0;
                        if supported {
                            score_opening += 25;
                            score_endgame += 15;
                        }
                    }
                }
                2 => { // White Bishop (B)
                    score_opening += POSITIONAL_SCORE[OPENING][BISHOP][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][BISHOP][square];

                    let attacks_count = count_bits(attack_tables.get_bishop_attacks(square, occ_both)) as i32;
                    score_opening += (attacks_count - BISHOP_UNIT) * BISHOP_MOBILITY_OPENING;
                    score_endgame += (attacks_count - BISHOP_UNIT) * BISHOP_MOBILITY_ENDGAME;
                }
                3 => { // White Rook (R)
                    score_opening += POSITIONAL_SCORE[OPENING][ROOK][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][ROOK][square];

                    // Rook on 7th rank (rank index 1)
                    if square / 8 == 1 {
                        score_opening += 35;
                        score_endgame += 35;
                    }

                    if (board.bitboards[Piece::P as usize] & masks.file_masks[square]) == 0 {
                        score_opening += SEMI_OPEN_FILE_SCORE;
                        score_endgame += SEMI_OPEN_FILE_SCORE;
                    }

                    if ((board.bitboards[Piece::P as usize] | board.bitboards[Piece::PawnBlack as usize]) & masks.file_masks[square]) == 0 {
                        score_opening += OPEN_FILE_SCORE;
                        score_endgame += OPEN_FILE_SCORE;
                    }
                }
                4 => { // White Queen (Q)
                    score_opening += POSITIONAL_SCORE[OPENING][QUEEN][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][QUEEN][square];

                    let attacks_count = count_bits(attack_tables.get_queen_attacks(square, occ_both)) as i32;
                    score_opening += (attacks_count - QUEEN_UNIT) * QUEEN_MOBILITY_OPENING;
                    score_endgame += (attacks_count - QUEEN_UNIT) * QUEEN_MOBILITY_ENDGAME;
                }
                5 => { // White King (K)
                    score_opening += POSITIONAL_SCORE[OPENING][KING][square];
                    score_endgame += POSITIONAL_SCORE[ENDGAME][KING][square];

                    let file = (square % 8) as i32;
                    for f in [file - 1, file, file + 1] {
                        if f >= 0 && f <= 7 {
                            let f_mask = masks.file_masks[f as usize];
                            if (board.bitboards[Piece::P as usize] & f_mask) == 0 {
                                score_opening -= SEMI_OPEN_FILE_SCORE;
                                score_endgame -= SEMI_OPEN_FILE_SCORE;
                            }
                            if ((board.bitboards[Piece::P as usize] | board.bitboards[Piece::PawnBlack as usize]) & f_mask) == 0 {
                                score_opening -= OPEN_FILE_SCORE;
                                score_endgame -= OPEN_FILE_SCORE;
                            }
                        }
                    }

                    let shield = count_bits(attack_tables.king_attacks[square] & board.occupancies[Color::White as usize]) as i32;
                    score_opening += shield * KING_SHIELD_BONUS;
                    score_endgame += shield * KING_SHIELD_BONUS;
                }
                6 => { // Black Pawn (p)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][PAWN][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][PAWN][mirror_sq];

                    let double_pawns = count_bits(board.bitboards[Piece::PawnBlack as usize] & masks.file_masks[square]);
                    if double_pawns > 1 {
                        score_opening -= (double_pawns as i32 - 1) * DOUBLE_PAWN_PENALTY_OPENING;
                        score_endgame -= (double_pawns as i32 - 1) * DOUBLE_PAWN_PENALTY_ENDGAME;
                    }

                    if (board.bitboards[Piece::PawnBlack as usize] & masks.isolated_masks[square]) == 0 {
                        score_opening -= ISOLATED_PAWN_PENALTY_OPENING;
                        score_endgame -= ISOLATED_PAWN_PENALTY_ENDGAME;
                    }

                    let is_passed = (masks.black_passed_masks[square] & board.bitboards[Piece::P as usize]) == 0;
                    if is_passed {
                        score_opening -= PASSED_PAWN_BONUS[GET_RANK[square]];
                        score_endgame -= PASSED_PAWN_BONUS[GET_RANK[square]];

                        // Protected passed pawn bonus
                        if (attack_tables.pawn_attacks[Color::White as usize][square] & board.bitboards[Piece::PawnBlack as usize]) != 0 {
                            score_opening -= 20;
                            score_endgame -= 35;
                        }

                        // Endgame king proximity to passed pawn
                        if phase < 14 {
                            let wk_dist = chebyshev_distance(white_king_sq, square);
                            let bk_dist = chebyshev_distance(black_king_sq, square);
                            score_endgame -= (wk_dist - bk_dist) * 6;
                        }
                    }
                }
                7 => { // Black Knight (n)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][KNIGHT][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][KNIGHT][mirror_sq];

                    // Knight mobility
                    let safe_sqs = attack_tables.knight_attacks[square] & !white_pawn_attacks & !board.occupancies[Color::Black as usize];
                    let mob = count_bits(safe_sqs) as i32 - 4;
                    score_opening -= mob * 4;
                    score_endgame -= mob * 4;

                    // Knight Outpost on ranks 4-6 (r in 3..=5) supported by own pawn
                    let r = square / 8;
                    if r >= 3 && r <= 5 {
                        let supported = (attack_tables.pawn_attacks[Color::White as usize][square] & board.bitboards[Piece::PawnBlack as usize]) != 0;
                        if supported {
                            score_opening -= 25;
                            score_endgame -= 15;
                        }
                    }
                }
                8 => { // Black Bishop (b)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][BISHOP][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][BISHOP][mirror_sq];

                    let attacks_count = count_bits(attack_tables.get_bishop_attacks(square, occ_both)) as i32;
                    score_opening -= (attacks_count - BISHOP_UNIT) * BISHOP_MOBILITY_OPENING;
                    score_endgame -= (attacks_count - BISHOP_UNIT) * BISHOP_MOBILITY_ENDGAME;
                }
                9 => { // Black Rook (r)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][ROOK][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][ROOK][mirror_sq];

                    // Rook on 2nd rank (rank index 6)
                    if square / 8 == 6 {
                        score_opening -= 35;
                        score_endgame -= 35;
                    }

                    if (board.bitboards[Piece::PawnBlack as usize] & masks.file_masks[square]) == 0 {
                        score_opening -= SEMI_OPEN_FILE_SCORE;
                        score_endgame -= SEMI_OPEN_FILE_SCORE;
                    }

                    if ((board.bitboards[Piece::P as usize] | board.bitboards[Piece::PawnBlack as usize]) & masks.file_masks[square]) == 0 {
                        score_opening -= OPEN_FILE_SCORE;
                        score_endgame -= OPEN_FILE_SCORE;
                    }
                }
                10 => { // Black Queen (q)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][QUEEN][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][QUEEN][mirror_sq];

                    let attacks_count = count_bits(attack_tables.get_queen_attacks(square, occ_both)) as i32;
                    score_opening -= (attacks_count - QUEEN_UNIT) * QUEEN_MOBILITY_OPENING;
                    score_endgame -= (attacks_count - QUEEN_UNIT) * QUEEN_MOBILITY_ENDGAME;
                }
                11 => { // Black King (k)
                    let mirror_sq = MIRROR_SCORE[square];
                    score_opening -= POSITIONAL_SCORE[OPENING][KING][mirror_sq];
                    score_endgame -= POSITIONAL_SCORE[ENDGAME][KING][mirror_sq];

                    let file = (square % 8) as i32;
                    for f in [file - 1, file, file + 1] {
                        if f >= 0 && f <= 7 {
                            let f_mask = masks.file_masks[f as usize];
                            if (board.bitboards[Piece::PawnBlack as usize] & f_mask) == 0 {
                                score_opening += SEMI_OPEN_FILE_SCORE;
                                score_endgame += SEMI_OPEN_FILE_SCORE;
                            }
                            if ((board.bitboards[Piece::P as usize] | board.bitboards[Piece::PawnBlack as usize]) & f_mask) == 0 {
                                score_opening += OPEN_FILE_SCORE;
                                score_endgame += OPEN_FILE_SCORE;
                            }
                        }
                    }

                    let shield = count_bits(attack_tables.king_attacks[square] & board.occupancies[Color::Black as usize]) as i32;
                    score_opening -= shield * KING_SHIELD_BONUS;
                    score_endgame -= shield * KING_SHIELD_BONUS;
                }
                _ => {}
            }
        }
    }

    // Continuous 24-phase linear interpolation
    let score = (score_opening * phase + score_endgame * (24 - phase)) / 24;

    if board.side == Color::White {
        score
    } else {
        -score
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::START_POSITION;
    use crate::zobrist::ZobristKeys;

    #[test]
    fn test_start_position_evaluation() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let masks = EvalMasks::new();
        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);

        let eval = evaluate(&board, &attack_tables, &masks);
        assert!(eval.abs() < 50, "Start position eval is {}", eval);
    }
}
