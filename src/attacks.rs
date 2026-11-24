//! Precalculated leaper piece attacks and sliding attack ray generators.

use crate::bitboard::*;
use crate::types::Color;

pub fn mask_pawn_attacks(side: Color, square: usize) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let mut bitboard: Bitboard = 0;
    set_bit(&mut bitboard, square);

    if side == Color::White {
        if ((bitboard >> 7) & NOT_A_FILE) != 0 {
            attacks |= bitboard >> 7;
        }
        if ((bitboard >> 9) & NOT_H_FILE) != 0 {
            attacks |= bitboard >> 9;
        }
    } else {
        if ((bitboard << 7) & NOT_H_FILE) != 0 {
            attacks |= bitboard << 7;
        }
        if ((bitboard << 9) & NOT_A_FILE) != 0 {
            attacks |= bitboard << 9;
        }
    }

    attacks
}

pub fn mask_knight_attacks(square: usize) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let mut bitboard: Bitboard = 0;
    set_bit(&mut bitboard, square);

    if ((bitboard >> 17) & NOT_H_FILE) != 0 { attacks |= bitboard >> 17; }
    if ((bitboard >> 15) & NOT_A_FILE) != 0 { attacks |= bitboard >> 15; }
    if ((bitboard >> 10) & NOT_HG_FILE) != 0 { attacks |= bitboard >> 10; }
    if ((bitboard >> 6) & NOT_AB_FILE) != 0 { attacks |= bitboard >> 6; }
    if ((bitboard << 17) & NOT_A_FILE) != 0 { attacks |= bitboard << 17; }
    if ((bitboard << 15) & NOT_H_FILE) != 0 { attacks |= bitboard << 15; }
    if ((bitboard << 10) & NOT_AB_FILE) != 0 { attacks |= bitboard << 10; }
    if ((bitboard << 6) & NOT_HG_FILE) != 0 { attacks |= bitboard << 6; }

    attacks
}

pub fn mask_king_attacks(square: usize) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let mut bitboard: Bitboard = 0;
    set_bit(&mut bitboard, square);

    if (bitboard >> 8) != 0 { attacks |= bitboard >> 8; }
    if ((bitboard >> 9) & NOT_H_FILE) != 0 { attacks |= bitboard >> 9; }
    if ((bitboard >> 7) & NOT_A_FILE) != 0 { attacks |= bitboard >> 7; }
    if ((bitboard >> 1) & NOT_H_FILE) != 0 { attacks |= bitboard >> 1; }
    if (bitboard << 8) != 0 { attacks |= bitboard << 8; }
    if ((bitboard << 9) & NOT_A_FILE) != 0 { attacks |= bitboard << 9; }
    if ((bitboard << 7) & NOT_H_FILE) != 0 { attacks |= bitboard << 7; }
    if ((bitboard << 1) & NOT_A_FILE) != 0 { attacks |= bitboard << 1; }

    attacks
}

pub fn mask_bishop_attacks(square: usize) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let tr = (square / 8) as i32;
    let tf = (square % 8) as i32;

    let mut r = tr + 1; let mut f = tf + 1;
    while r <= 6 && f <= 6 {
        attacks |= 1u64 << (r * 8 + f);
        r += 1; f += 1;
    }
    r = tr - 1; f = tf + 1;
    while r >= 1 && f <= 6 {
        attacks |= 1u64 << (r * 8 + f);
        r -= 1; f += 1;
    }
    r = tr + 1; f = tf - 1;
    while r <= 6 && f >= 1 {
        attacks |= 1u64 << (r * 8 + f);
        r += 1; f -= 1;
    }
    r = tr - 1; f = tf - 1;
    while r >= 1 && f >= 1 {
        attacks |= 1u64 << (r * 8 + f);
        r -= 1; f -= 1;
    }

    attacks
}

pub fn mask_rook_attacks(square: usize) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let tr = (square / 8) as i32;
    let tf = (square % 8) as i32;

    for r in (tr + 1)..=6 {
        attacks |= 1u64 << (r * 8 + tf);
    }
    for r in (1..tr).rev() {
        attacks |= 1u64 << (r * 8 + tf);
    }
    for f in (tf + 1)..=6 {
        attacks |= 1u64 << (tr * 8 + f);
    }
    for f in (1..tf).rev() {
        attacks |= 1u64 << (tr * 8 + f);
    }

    attacks
}

pub fn bishop_attacks_on_the_fly(square: usize, block: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let tr = (square / 8) as i32;
    let tf = (square % 8) as i32;

    let mut r = tr + 1; let mut f = tf + 1;
    while r <= 7 && f <= 7 {
        let sq = (r * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
        r += 1; f += 1;
    }
    r = tr - 1; f = tf + 1;
    while r >= 0 && f <= 7 {
        let sq = (r * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
        r -= 1; f += 1;
    }
    r = tr + 1; f = tf - 1;
    while r <= 7 && f >= 0 {
        let sq = (r * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
        r += 1; f -= 1;
    }
    r = tr - 1; f = tf - 1;
    while r >= 0 && f >= 0 {
        let sq = (r * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
        r -= 1; f -= 1;
    }

    attacks
}

pub fn rook_attacks_on_the_fly(square: usize, block: Bitboard) -> Bitboard {
    let mut attacks: Bitboard = 0;
    let tr = (square / 8) as i32;
    let tf = (square % 8) as i32;

    for r in (tr + 1)..=7 {
        let sq = (r * 8 + tf) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
    }
    for r in (0..tr).rev() {
        let sq = (r * 8 + tf) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
    }
    for f in (tf + 1)..=7 {
        let sq = (tr * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
    }
    for f in (0..tf).rev() {
        let sq = (tr * 8 + f) as usize;
        attacks |= 1u64 << sq;
        if (1u64 << sq) & block != 0 { break; }
    }

    attacks
}

pub fn set_occupancy(index: usize, bits_in_mask: usize, mut attack_mask: Bitboard) -> Bitboard {
    let mut occupancy: Bitboard = 0;
    for count in 0..bits_in_mask {
        if let Some(square) = get_ls1b_index(attack_mask) {
            pop_bit(&mut attack_mask, square);
            if (index & (1 << count)) != 0 {
                occupancy |= 1u64 << square;
            }
        }
    }
    occupancy
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::square::*;

    #[test]
    fn test_pawn_attacks() {
        // e4 is square 36
        // White pawn on e4 attacks d5 (27) and f5 (29)
        let w_attacks = mask_pawn_attacks(Color::White, e4);
        assert!(get_bit(w_attacks, d5));
        assert!(get_bit(w_attacks, f5));
        assert_eq!(count_bits(w_attacks), 2);

        // a4 is square 32. White pawn on a4 only attacks b5 (25)
        let w_a4 = mask_pawn_attacks(Color::White, a4);
        assert!(get_bit(w_a4, b5));
        assert_eq!(count_bits(w_a4), 1);

        // Black pawn on e5 (28) attacks d4 (35) and f4 (37)
        let b_attacks = mask_pawn_attacks(Color::Black, e5);
        assert!(get_bit(b_attacks, d4));
        assert!(get_bit(b_attacks, f4));
        assert_eq!(count_bits(b_attacks), 2);
    }

    #[test]
    fn test_knight_attacks() {
        // Knight on e4 (center) has 8 moves
        let attacks = mask_knight_attacks(e4);
        assert_eq!(count_bits(attacks), 8);
        assert!(get_bit(attacks, d6));
        assert!(get_bit(attacks, f6));
        assert!(get_bit(attacks, c5));
        assert!(get_bit(attacks, g5));
        assert!(get_bit(attacks, c3));
        assert!(get_bit(attacks, g3));
        assert!(get_bit(attacks, d2));
        assert!(get_bit(attacks, f2));

        // Knight on a1 (corner) has 2 moves: b3, c2
        let corner = mask_knight_attacks(a1);
        assert_eq!(count_bits(corner), 2);
        assert!(get_bit(corner, b3));
        assert!(get_bit(corner, c2));
    }

    #[test]
    fn test_king_attacks() {
        // King on e4 has 8 moves
        let attacks = mask_king_attacks(e4);
        assert_eq!(count_bits(attacks), 8);

        // King on a1 has 3 moves: a2, b1, b2
        let corner = mask_king_attacks(a1);
        assert_eq!(count_bits(corner), 3);
        assert!(get_bit(corner, a2));
        assert!(get_bit(corner, b1));
        assert!(get_bit(corner, b2));
    }

    #[test]
    fn test_rook_attacks_blocked() {
        // Rook on e4 with a blocker on e6 and e2
        let mut block: Bitboard = 0;
        set_bit(&mut block, e6);
        set_bit(&mut block, e2);
        let attacks = rook_attacks_on_the_fly(e4, block);
        // Should reach e6 and e2, but NOT e7, e8, e1
        assert!(get_bit(attacks, e5));
        assert!(get_bit(attacks, e6));
        assert!(!get_bit(attacks, e7));
        assert!(get_bit(attacks, e3));
        assert!(get_bit(attacks, e2));
        assert!(!get_bit(attacks, e1));
    }
}
