//! 64-bit Bitboard operations and file masks.

pub type Bitboard = u64;

pub const NOT_A_FILE: Bitboard = 18374403900871474942;
pub const NOT_H_FILE: Bitboard = 9187201950435737471;
pub const NOT_HG_FILE: Bitboard = 4557430888798830399;
pub const NOT_AB_FILE: Bitboard = 18229723555195321596;

#[inline(always)]
pub fn set_bit(bb: &mut Bitboard, square: usize) {
    *bb |= 1u64 << square;
}

#[inline(always)]
pub fn get_bit(bb: Bitboard, square: usize) -> bool {
    (bb & (1u64 << square)) != 0
}

#[inline(always)]
pub fn pop_bit(bb: &mut Bitboard, square: usize) {
    *bb &= !(1u64 << square);
}

#[inline(always)]
pub fn count_bits(bb: Bitboard) -> u32 {
    bb.count_ones()
}

#[inline(always)]
pub fn get_ls1b_index(bb: Bitboard) -> Option<usize> {
    if bb != 0 {
        Some(bb.trailing_zeros() as usize)
    } else {
        None
    }
}

pub fn format_bitboard(bb: Bitboard) -> String {
    let mut out = String::new();
    out.push_str("\n");
    for rank in 0..8 {
        out.push_str(&format!("  {} ", 8 - rank));
        for file in 0..8 {
            let square = rank * 8 + file;
            let bit = if get_bit(bb, square) { 1 } else { 0 };
            out.push_str(&format!(" {}", bit));
        }
        out.push('\n');
    }
    out.push_str("\n     a b c d e f g h\n\n");
    out.push_str(&format!("     Bitboard: 0x{:016x}ULL\n\n", bb));
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_set_get_pop_bit() {
        let mut bb: Bitboard = 0;
        assert_eq!(get_bit(bb, 0), false);
        set_bit(&mut bb, 0);
        assert_eq!(get_bit(bb, 0), true);
        assert_eq!(bb, 1);

        set_bit(&mut bb, 63);
        assert_eq!(get_bit(bb, 63), true);
        assert_eq!(count_bits(bb), 2);

        pop_bit(&mut bb, 0);
        assert_eq!(get_bit(bb, 0), false);
        assert_eq!(count_bits(bb), 1);
        assert_eq!(bb, 1u64 << 63);
    }

    #[test]
    fn test_count_and_ls1b() {
        assert_eq!(count_bits(0), 0);
        assert_eq!(get_ls1b_index(0), None);

        let bb: Bitboard = (1 << 5) | (1 << 12);
        assert_eq!(count_bits(bb), 2);
        assert_eq!(get_ls1b_index(bb), Some(5));

        let mut temp = bb;
        if let Some(sq) = get_ls1b_index(temp) {
            pop_bit(&mut temp, sq);
        }
        assert_eq!(get_ls1b_index(temp), Some(12));
    }

    #[test]
    fn test_masks() {
        // Square 0 is a8 (file 0). NOT_A_FILE should have bit 0 as 0.
        assert_eq!(get_bit(NOT_A_FILE, 0), false);
        // Square 1 is b8 (file 1). NOT_A_FILE should have bit 1 as 1.
        assert_eq!(get_bit(NOT_A_FILE, 1), true);

        // Square 7 is h8 (file 7). NOT_H_FILE should have bit 7 as 0.
        assert_eq!(get_bit(NOT_H_FILE, 7), false);
        // Square 6 is g8. NOT_H_FILE should have bit 6 as 1.
        assert_eq!(get_bit(NOT_H_FILE, 6), true);
    }
}
