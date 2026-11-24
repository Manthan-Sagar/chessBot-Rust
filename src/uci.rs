//! UCI (Universal Chess Interface) protocol parser and command loop.

use std::io::{self, BufRead};
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;

use crate::board::{Board, START_POSITION};
use crate::evaluate::EvalMasks;
use crate::magic::AttackTables;
use crate::movegen::{generate_moves, make_move, MoveFlag};
use crate::moves::{Move, MoveList};
use crate::perft::perft_divide;
use crate::search::{SearchLimits, Searcher, MAX_PLY};
use crate::tt::TranspositionTable;
use crate::types::{parse_square, Color, Piece};
use crate::zobrist::ZobristKeys;

pub struct UciEngine {
    pub board: Board,
    pub attack_tables: AttackTables,
    pub masks: EvalMasks,
    pub zobrist: ZobristKeys,
    pub tt: TranspositionTable,
    pub stopped: Arc<AtomicBool>,
}

impl UciEngine {
    pub fn new() -> Self {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let masks = EvalMasks::new();
        let tt = TranspositionTable::new(64); // 64 MB hash table
        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);
        let stopped = Arc::new(AtomicBool::new(false));

        Self {
            board,
            attack_tables,
            masks,
            zobrist,
            tt,
            stopped,
        }
    }

    pub fn parse_move(&mut self, move_str: &str) -> Option<Move> {
        if move_str.len() < 4 {
            return None;
        }

        let from_sq = parse_square(&move_str[0..2])?;
        let to_sq = parse_square(&move_str[2..4])?;
        let prom_char = if move_str.len() >= 5 {
            move_str.chars().nth(4)
        } else {
            None
        };

        let mut move_list = MoveList::new();
        generate_moves(&self.board, &self.attack_tables, &mut move_list);

        for i in 0..move_list.count {
            let m = move_list.get(i);
            if m.source() == from_sq && m.target() == to_sq {
                if let Some(prom) = m.promoted() {
                    let expected_char = match prom {
                        Piece::Q | Piece::QueenBlack => 'q',
                        Piece::R | Piece::RookBlack => 'r',
                        Piece::B | Piece::BishopBlack => 'b',
                        Piece::N | Piece::KnightBlack => 'n',
                        _ => ' ',
                    };
                    if Some(expected_char) == prom_char {
                        return Some(m);
                    }
                } else if prom_char.is_none() {
                    return Some(m);
                }
            }
        }

        None
    }

    pub fn parse_position(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        if tokens.len() < 2 {
            return;
        }

        let mut move_index = None;

        if tokens[1] == "startpos" {
            self.board.parse_fen(START_POSITION, &self.zobrist);
            if let Some(pos) = tokens.iter().position(|&x| x == "moves") {
                move_index = Some(pos + 1);
            }
        } else if tokens[1] == "fen" {
            // Find where "moves" starts, if any
            let moves_pos = tokens.iter().position(|&x| x == "moves");
            let fen_tokens = if let Some(m_pos) = moves_pos {
                move_index = Some(m_pos + 1);
                &tokens[2..m_pos]
            } else {
                &tokens[2..]
            };
            let fen_str = fen_tokens.join(" ");
            self.board.parse_fen(&fen_str, &self.zobrist);
        }

        if let Some(start_idx) = move_index {
            for &move_str in &tokens[start_idx..] {
                if let Some(m) = self.parse_move(move_str) {
                    self.board.repetition_index += 1;
                    self.board.repetition_table[self.board.repetition_index] = self.board.hash_key;
                    make_move(
                        &mut self.board,
                        m,
                        MoveFlag::AllMoves,
                        &self.attack_tables,
                        &self.zobrist,
                    );
                } else {
                    break;
                }
            }
        }
    }

    pub fn parse_go(&mut self, command: &str) {
        let tokens: Vec<&str> = command.split_whitespace().collect();
        let mut limits = SearchLimits::default();
        let mut max_depth = MAX_PLY;

        let mut i = 1;
        while i < tokens.len() {
            match tokens[i] {
                "perft" => {
                    if i + 1 < tokens.len() {
                        let depth: usize = tokens[i + 1].parse().unwrap_or(1);
                        perft_divide(depth, &mut self.board, &self.attack_tables, &self.zobrist);
                        return;
                    }
                }
                "depth" => {
                    if i + 1 < tokens.len() {
                        max_depth = tokens[i + 1].parse().unwrap_or(MAX_PLY);
                        limits.depth = Some(max_depth);
                        i += 1;
                    }
                }
                "movetime" => {
                    if i + 1 < tokens.len() {
                        limits.movetime = tokens[i + 1].parse().ok();
                        i += 1;
                    }
                }
                "wtime" => {
                    if i + 1 < tokens.len() {
                        if self.board.side == Color::White {
                            limits.time = tokens[i + 1].parse().ok();
                        }
                        i += 1;
                    }
                }
                "btime" => {
                    if i + 1 < tokens.len() {
                        if self.board.side == Color::Black {
                            limits.time = tokens[i + 1].parse().ok();
                        }
                        i += 1;
                    }
                }
                "winc" => {
                    if i + 1 < tokens.len() {
                        if self.board.side == Color::White {
                            limits.inc = tokens[i + 1].parse().unwrap_or(0);
                        }
                        i += 1;
                    }
                }
                "binc" => {
                    if i + 1 < tokens.len() {
                        if self.board.side == Color::Black {
                            limits.inc = tokens[i + 1].parse().unwrap_or(0);
                        }
                        i += 1;
                    }
                }
                "movestogo" => {
                    if i + 1 < tokens.len() {
                        limits.movestogo = tokens[i + 1].parse().ok();
                        i += 1;
                    }
                }
                _ => {}
            }
            i += 1;
        }

        let mut searcher = Searcher::new(self.stopped.clone());
        searcher.search_position(
            max_depth,
            &limits,
            &mut self.board,
            &self.attack_tables,
            &self.masks,
            &self.zobrist,
            &mut self.tt,
        );
    }

    pub fn handle_command(&mut self, line: &str) -> bool {
        let trimmed = line.trim();
        if trimmed.is_empty() {
            return true;
        }

        if trimmed == "uci" {
            println!("id name chess-engine");
            println!("id author Antigravity");
            println!("uciok");
        } else if trimmed == "isready" {
            println!("readyok");
        } else if trimmed == "ucinewgame" {
            self.tt.clear();
            self.board.parse_fen(START_POSITION, &self.zobrist);
        } else if trimmed.starts_with("position") {
            self.parse_position(trimmed);
        } else if trimmed.starts_with("go") {
            self.parse_go(trimmed);
        } else if trimmed == "stop" {
            self.stopped.store(true, Ordering::Relaxed);
        } else if trimmed == "quit" {
            self.stopped.store(true, Ordering::Relaxed);
            return false;
        } else if trimmed == "d" || trimmed == "print" {
            self.board.print();
        }

        true
    }

    pub fn uci_loop(&mut self) {
        let stdin = io::stdin();
        for line in stdin.lock().lines() {
            match line {
                Ok(cmd) => {
                    if !self.handle_command(&cmd) {
                        break;
                    }
                }
                Err(_) => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::square;

    #[test]
    fn test_uci_isready_and_uciok() {
        let mut engine = UciEngine::new();
        assert!(engine.handle_command("uci"));
        assert!(engine.handle_command("isready"));
        assert!(engine.handle_command("ucinewgame"));
    }

    #[test]
    fn test_uci_position_and_moves() {
        let mut engine = UciEngine::new();
        engine.handle_command("position startpos moves e2e4 e7e5");
        assert_eq!(engine.board.side, Color::White);
        assert_eq!(engine.board.enpassant, square::e6);

        // Another quiet move clears en passant
        engine.handle_command("position startpos moves e2e4 e7e5 g1f3");
        assert_eq!(engine.board.side, Color::Black);
        assert_eq!(engine.board.enpassant, square::NO_SQ);

        // Position with FEN
        engine.handle_command("position fen r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1 moves e1g1");
        assert_eq!(engine.board.side, Color::Black);
    }
}
