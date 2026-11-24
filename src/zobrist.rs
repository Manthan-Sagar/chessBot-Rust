//! 64-bit Zobrist hashing keys and incremental hash generation.

use crate::bitboard::*;
use crate::types::{square, Color};

#[derive(Clone)]
pub struct ZobristKeys {
    pub piece_keys: [[u64; 64]; 12],
    pub enpassant_keys: [u64; 64],
    pub castle_keys: [u64; 16],
    pub side_key: u64,
}

impl ZobristKeys {
    pub fn new() -> Self {
        let mut rng_state: u32 = 1804289383;

        let mut next_u32 = || -> u32 {
            let mut number = rng_state;
            number ^= number << 13;
            number ^= number >> 17;
            number ^= number << 5;
            rng_state = number;
            number
        };

        let mut next_u64 = || -> u64 {
            let n1 = (next_u32() & 0xFFFF) as u64;
            let n2 = (next_u32() & 0xFFFF) as u64;
            let n3 = (next_u32() & 0xFFFF) as u64;
            let n4 = (next_u32() & 0xFFFF) as u64;
            n1 | (n2 << 16) | (n3 << 32) | (n4 << 48)
        };

        let mut piece_keys = [[0u64; 64]; 12];
        for piece in 0..12 {
            for sq in 0..64 {
                piece_keys[piece][sq] = next_u64();
            }
        }

        let mut enpassant_keys = [0u64; 64];
        for sq in 0..64 {
            enpassant_keys[sq] = next_u64();
        }

        let mut castle_keys = [0u64; 16];
        for idx in 0..16 {
            castle_keys[idx] = next_u64();
        }

        let side_key = next_u64();

        Self {
            piece_keys,
            enpassant_keys,
            castle_keys,
            side_key,
        }
    }

    pub fn generate_hash_key(
        &self,
        bitboards: &[Bitboard; 12],
        side: Color,
        enpassant: usize,
        castle: u8,
    ) -> u64 {
        let mut key: u64 = 0;

        for piece in 0..12 {
            let mut bb = bitboards[piece];
            while bb != 0 {
                let sq = get_ls1b_index(bb).unwrap();
                key ^= self.piece_keys[piece][sq];
                pop_bit(&mut bb, sq);
            }
        }

        if enpassant != square::NO_SQ && enpassant < 64 {
            key ^= self.enpassant_keys[enpassant];
        }

        key ^= self.castle_keys[castle as usize & 0xf];

        if side == Color::Black {
            key ^= self.side_key;
        }

        key
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_zobrist_keys_generation() {
        let zobrist = ZobristKeys::new();
        // Check that keys are non-zero and distinct
        assert_ne!(zobrist.side_key, 0);
        assert_ne!(zobrist.piece_keys[0][0], zobrist.piece_keys[0][1]);
        assert_ne!(zobrist.castle_keys[0], zobrist.castle_keys[1]);
    }
}
