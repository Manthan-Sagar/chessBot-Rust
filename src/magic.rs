//! Magic bitboard precomputations and attack lookups.

use crate::attacks::*;
use crate::bitboard::*;
use crate::types::{Color, Piece};

pub const BISHOP_RELEVANT_BITS: [usize; 64] = [
    6, 5, 5, 5, 5, 5, 5, 6, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 9, 9, 7, 5, 5, 
    5, 5, 7, 7, 7, 7, 5, 5, 
    5, 5, 5, 5, 5, 5, 5, 5, 
    6, 5, 5, 5, 5, 5, 5, 6,
];

pub const ROOK_RELEVANT_BITS: [usize; 64] = [
    12, 11, 11, 11, 11, 11, 11, 12, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    11, 10, 10, 10, 10, 10, 10, 11, 
    12, 11, 11, 11, 11, 11, 11, 12,
];

pub const ROOK_MAGIC_NUMBERS: [u64; 64] = [
    0x8a80104000800020,
    0x140002000100040,
    0x2801880a0017001,
    0x100081001000420,
    0x200020010080420,
    0x3001c0002010008,
    0x8480008002000100,
    0x2080088004402900,
    0x800098204000,
    0x2024401000200040,
    0x100802000801000,
    0x120800800801000,
    0x208808088000400,
    0x2802200800400,
    0x2200800100020080,
    0x801000060821100,
    0x80044006422000,
    0x100808020004000,
    0x12108a0010204200,
    0x140848010000802,
    0x481828014002800,
    0x8094004002004100,
    0x4010040010010802,
    0x20008806104,
    0x100400080208000,
    0x2040002120081000,
    0x21200680100081,
    0x20100080080080,
    0x2000a00200410,
    0x20080800400,
    0x80088400100102,
    0x80004600042881,
    0x4040008040800020,
    0x440003000200801,
    0x4200011004500,
    0x188020010100100,
    0x14800401802800,
    0x2080040080800200,
    0x124080204001001,
    0x200046502000484,
    0x480400080088020,
    0x1000422010034000,
    0x30200100110040,
    0x100021010009,
    0x2002080100110004,
    0x202008004008002,
    0x20020004010100,
    0x2048440040820001,
    0x101002200408200,
    0x40802000401080,
    0x4008142004410100,
    0x2060820c0120200,
    0x1001004080100,
    0x20c020080040080,
    0x2935610830022400,
    0x44440041009200,
    0x280001040802101,
    0x2100190040002085,
    0x80c0084100102001,
    0x4024081001000421,
    0x20030a0244872,
    0x12001008414402,
    0x2006104900a0804,
    0x1004081002402,
];

pub const BISHOP_MAGIC_NUMBERS: [u64; 64] = [
    0x40040844404084,
    0x2004208a004208,
    0x10190041080202,
    0x108060845042010,
    0x581104180800210,
    0x2112080446200010,
    0x1080820820060210,
    0x3c0808410220200,
    0x4050404440404,
    0x21001420088,
    0x24d0080801082102,
    0x1020a0a020400,
    0x40308200402,
    0x4011002100800,
    0x401484104104005,
    0x801010402020200,
    0x400210c3880100,
    0x404022024108200,
    0x810018200204102,
    0x4002801a02003,
    0x85040820080400,
    0x810102c808880400,
    0xe900410884800,
    0x8002020480840102,
    0x220200865090201,
    0x2010100a02021202,
    0x152048408022401,
    0x20080002081110,
    0x4001001021004000,
    0x800040400a011002,
    0xe4004081011002,
    0x1c004001012080,
    0x8004200962a00220,
    0x8422100208500202,
    0x2000402200300c08,
    0x8646020080080080,
    0x80020a0200100808,
    0x2010004880111000,
    0x623000a080011400,
    0x42008c0340209202,
    0x209188240001000,
    0x400408a884001800,
    0x110400a6080400,
    0x1840060a44020800,
    0x90080104000041,
    0x201011000808101,
    0x1a2208080504f080,
    0x8012020600211212,
    0x500861011240000,
    0x180806108200800,
    0x4000020e01040044,
    0x300000261044000a,
    0x802241102020002,
    0x20906061210001,
    0x5a84841004010310,
    0x4010801011c04,
    0xa010109502200,
    0x4a02012000,
    0x500201010098b028,
    0x8040002811040900,
    0x28000010020204,
    0x6000020202d0240,
    0x8918844842082200,
    0x4010011029020020,
];

pub struct AttackTables {
    pub pawn_attacks: [[Bitboard; 64]; 2],
    pub knight_attacks: [Bitboard; 64],
    pub king_attacks: [Bitboard; 64],
    pub bishop_masks: [Bitboard; 64],
    pub rook_masks: [Bitboard; 64],
    pub bishop_attacks: Vec<[Bitboard; 512]>,
    pub rook_attacks: Vec<[Bitboard; 4096]>,
}

impl AttackTables {
    pub fn new() -> Self {
        let mut pawn_attacks = [[0u64; 64]; 2];
        let mut knight_attacks = [0u64; 64];
        let mut king_attacks = [0u64; 64];
        let mut bishop_masks = [0u64; 64];
        let mut rook_masks = [0u64; 64];

        for sq in 0..64 {
            pawn_attacks[Color::White as usize][sq] = mask_pawn_attacks(Color::White, sq);
            pawn_attacks[Color::Black as usize][sq] = mask_pawn_attacks(Color::Black, sq);
            knight_attacks[sq] = mask_knight_attacks(sq);
            king_attacks[sq] = mask_king_attacks(sq);
            bishop_masks[sq] = mask_bishop_attacks(sq);
            rook_masks[sq] = mask_rook_attacks(sq);
        }

        let mut bishop_attacks = vec![[0u64; 512]; 64];
        let mut rook_attacks = vec![[0u64; 4096]; 64];

        // Init bishop attack table
        for sq in 0..64 {
            let mask = bishop_masks[sq];
            let relevant_bits = count_bits(mask) as usize;
            let occupancy_indices = 1 << relevant_bits;

            for index in 0..occupancy_indices {
                let occ = set_occupancy(index, relevant_bits, mask);
                let magic_index = (occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[sq]) >> (64 - BISHOP_RELEVANT_BITS[sq])) as usize;
                bishop_attacks[sq][magic_index] = bishop_attacks_on_the_fly(sq, occ);
            }
        }

        // Init rook attack table
        for sq in 0..64 {
            let mask = rook_masks[sq];
            let relevant_bits = count_bits(mask) as usize;
            let occupancy_indices = 1 << relevant_bits;

            for index in 0..occupancy_indices {
                let occ = set_occupancy(index, relevant_bits, mask);
                let magic_index = (occ.wrapping_mul(ROOK_MAGIC_NUMBERS[sq]) >> (64 - ROOK_RELEVANT_BITS[sq])) as usize;
                rook_attacks[sq][magic_index] = rook_attacks_on_the_fly(sq, occ);
            }
        }

        Self {
            pawn_attacks,
            knight_attacks,
            king_attacks,
            bishop_masks,
            rook_masks,
            bishop_attacks,
            rook_attacks,
        }
    }

    #[inline(always)]
    pub fn get_bishop_attacks(&self, square: usize, occupancy: Bitboard) -> Bitboard {
        let mut occ = occupancy & self.bishop_masks[square];
        occ = occ.wrapping_mul(BISHOP_MAGIC_NUMBERS[square]);
        occ >>= 64 - BISHOP_RELEVANT_BITS[square];
        self.bishop_attacks[square][occ as usize]
    }

    #[inline(always)]
    pub fn get_rook_attacks(&self, square: usize, occupancy: Bitboard) -> Bitboard {
        let mut occ = occupancy & self.rook_masks[square];
        occ = occ.wrapping_mul(ROOK_MAGIC_NUMBERS[square]);
        occ >>= 64 - ROOK_RELEVANT_BITS[square];
        self.rook_attacks[square][occ as usize]
    }

    #[inline(always)]
    pub fn get_queen_attacks(&self, square: usize, occupancy: Bitboard) -> Bitboard {
        self.get_bishop_attacks(square, occupancy) | self.get_rook_attacks(square, occupancy)
    }

    #[inline(always)]
    pub fn is_square_attacked(
        &self,
        square: usize,
        by_side: Color,
        bitboards: &[Bitboard; 12],
        occupancy_both: Bitboard,
    ) -> bool {
        if by_side == Color::White {
            // Attacked by white pawns?
            if (self.pawn_attacks[Color::Black as usize][square] & bitboards[Piece::P as usize]) != 0 {
                return true;
            }
            // Attacked by white knights?
            if (self.knight_attacks[square] & bitboards[Piece::N as usize]) != 0 {
                return true;
            }
            // Attacked by white bishops?
            if (self.get_bishop_attacks(square, occupancy_both) & bitboards[Piece::B as usize]) != 0 {
                return true;
            }
            // Attacked by white rooks?
            if (self.get_rook_attacks(square, occupancy_both) & bitboards[Piece::R as usize]) != 0 {
                return true;
            }
            // Attacked by white queens?
            if (self.get_queen_attacks(square, occupancy_both) & bitboards[Piece::Q as usize]) != 0 {
                return true;
            }
            // Attacked by white king?
            if (self.king_attacks[square] & bitboards[Piece::K as usize]) != 0 {
                return true;
            }
        } else {
            // Attacked by black pawns?
            if (self.pawn_attacks[Color::White as usize][square] & bitboards[Piece::PawnBlack as usize]) != 0 {
                return true;
            }
            // Attacked by black knights?
            if (self.knight_attacks[square] & bitboards[Piece::KnightBlack as usize]) != 0 {
                return true;
            }
            // Attacked by black bishops?
            if (self.get_bishop_attacks(square, occupancy_both) & bitboards[Piece::BishopBlack as usize]) != 0 {
                return true;
            }
            // Attacked by black rooks?
            if (self.get_rook_attacks(square, occupancy_both) & bitboards[Piece::RookBlack as usize]) != 0 {
                return true;
            }
            // Attacked by black queens?
            if (self.get_queen_attacks(square, occupancy_both) & bitboards[Piece::QueenBlack as usize]) != 0 {
                return true;
            }
            // Attacked by black king?
            if (self.king_attacks[square] & bitboards[Piece::KingBlack as usize]) != 0 {
                return true;
            }
        }

        false
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magic_bishop_matches_on_the_fly() {
        let tables = AttackTables::new();
        // Test all squares with empty board
        for sq in 0..64 {
            let otf = bishop_attacks_on_the_fly(sq, 0);
            let magic = tables.get_bishop_attacks(sq, 0);
            assert_eq!(otf, magic, "Mismatch on empty board for bishop sq {}", sq);
        }

        // Test with arbitrary occupancies
        let occupancies = [
            0x0000001000000000u64,
            0x0000284482000000u64,
            0x1234567890abcdefu64,
            0xffffffffffffffffu64,
        ];
        for &occ in &occupancies {
            for sq in 0..64 {
                let otf = bishop_attacks_on_the_fly(sq, occ);
                let magic = tables.get_bishop_attacks(sq, occ);
                assert_eq!(otf, magic, "Mismatch with occ 0x{:x} for bishop sq {}", occ, sq);
            }
        }
    }

    #[test]
    fn test_magic_rook_matches_on_the_fly() {
        let tables = AttackTables::new();
        for sq in 0..64 {
            let otf = rook_attacks_on_the_fly(sq, 0);
            let magic = tables.get_rook_attacks(sq, 0);
            assert_eq!(otf, magic, "Mismatch on empty board for rook sq {}", sq);
        }

        let occupancies = [
            0x0000001000000000u64,
            0x0000284482000000u64,
            0x1234567890abcdefu64,
            0xffffffffffffffffu64,
        ];
        for &occ in &occupancies {
            for sq in 0..64 {
                let otf = rook_attacks_on_the_fly(sq, occ);
                let magic = tables.get_rook_attacks(sq, occ);
                assert_eq!(otf, magic, "Mismatch with occ 0x{:x} for rook sq {}", occ, sq);
            }
        }
    }
}
