//! Transposition Table (hash table) for search acceleration.

use crate::moves::Move;

pub const HASH_FLAG_EXACT: u8 = 0;
pub const HASH_FLAG_ALPHA: u8 = 1;
pub const HASH_FLAG_BETA: u8 = 2;
pub const NO_HASH_ENTRY: i32 = 100000;
pub const MATE_SCORE: i32 = 48000;

#[derive(Clone, Copy, Default)]
pub struct TTEntry {
    pub hash_key: u64,
    pub depth: i32,
    pub flag: u8,
    pub score: i32,
    pub best_move: Option<Move>,
}

pub struct TranspositionTable {
    pub entries: Vec<TTEntry>,
    pub num_entries: usize,
}

impl TranspositionTable {
    pub fn new(mb: usize) -> Self {
        let size_bytes = mb * 1024 * 1024;
        let num_entries = size_bytes / std::mem::size_of::<TTEntry>();
        Self {
            entries: vec![TTEntry::default(); num_entries],
            num_entries,
        }
    }

    pub fn clear(&mut self) {
        for entry in self.entries.iter_mut() {
            *entry = TTEntry::default();
        }
    }

    #[inline(always)]
    pub fn read(
        &self,
        hash_key: u64,
        alpha: i32,
        beta: i32,
        depth: i32,
        ply: usize,
    ) -> Option<(i32, Option<Move>)> {
        let index = (hash_key % (self.num_entries as u64)) as usize;
        let entry = &self.entries[index];

        if entry.hash_key == hash_key {
            let mut score = entry.score;
            if score < -MATE_SCORE {
                score += ply as i32;
            } else if score > MATE_SCORE {
                score -= ply as i32;
            }

            if entry.depth >= depth {
                if entry.flag == HASH_FLAG_EXACT {
                    return Some((score, entry.best_move));
                }
                if entry.flag == HASH_FLAG_ALPHA && score <= alpha {
                    return Some((alpha, entry.best_move));
                }
                if entry.flag == HASH_FLAG_BETA && score >= beta {
                    return Some((beta, entry.best_move));
                }
            }

            return Some((NO_HASH_ENTRY, entry.best_move));
        }

        None
    }

    #[inline(always)]
    pub fn write(
        &mut self,
        hash_key: u64,
        mut score: i32,
        depth: i32,
        flag: u8,
        ply: usize,
        best_move: Option<Move>,
    ) {
        let index = (hash_key % (self.num_entries as u64)) as usize;

        if score < -MATE_SCORE {
            score -= ply as i32;
        } else if score > MATE_SCORE {
            score += ply as i32;
        }

        self.entries[index] = TTEntry {
            hash_key,
            depth,
            flag,
            score,
            best_move,
        };
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_tt_write_and_read() {
        let mut tt = TranspositionTable::new(16);
        assert_eq!(tt.read(12345, -100, 100, 3, 0), None);

        tt.write(12345, 50, 4, HASH_FLAG_EXACT, 0, None);
        let res = tt.read(12345, -100, 100, 3, 0);
        assert!(res.is_some());
        let (score, _) = res.unwrap();
        assert_eq!(score, 50);
    }
}
