//! Negamax alpha-beta search with quiescence, move ordering, TT, LMR, and iterative deepening.

use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::time::Instant;

use crate::bitboard::*;
use crate::board::Board;
use crate::evaluate::{evaluate, EvalMasks};
use crate::magic::AttackTables;
use crate::movegen::{generate_moves, make_move, MoveFlag};
use crate::moves::{Move, MoveList};
use crate::see::see;
use crate::tt::{TranspositionTable, HASH_FLAG_ALPHA, HASH_FLAG_BETA, HASH_FLAG_EXACT, NO_HASH_ENTRY};
use crate::types::*;
use crate::zobrist::ZobristKeys;

pub const INFINITY: i32 = 50000;
pub const MATE_VALUE: i32 = 49000;
pub const MATE_SCORE: i32 = 48000;
pub const MAX_PLY: usize = 64;
pub const FULL_DEPTH_MOVES: usize = 4;
pub const REDUCTION_LIMIT: i32 = 3;

pub const MVV_LVA: [[i32; 12]; 12] = [
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
    [105, 205, 305, 405, 505, 605, 105, 205, 305, 405, 505, 605],
    [104, 204, 304, 404, 504, 604, 104, 204, 304, 404, 504, 604],
    [103, 203, 303, 403, 503, 603, 103, 203, 303, 403, 503, 603],
    [102, 202, 302, 402, 502, 602, 102, 202, 302, 402, 502, 602],
    [101, 201, 301, 401, 501, 601, 101, 201, 301, 401, 501, 601],
    [100, 200, 300, 400, 500, 600, 100, 200, 300, 400, 500, 600],
];

#[derive(Clone, Default)]
pub struct SearchLimits {
    pub depth: Option<usize>,
    pub movetime: Option<u64>,
    pub time: Option<u64>,
    pub inc: u64,
    pub movestogo: Option<usize>,
}

pub struct Searcher {
    pub nodes: u64,
    pub stopped: Arc<AtomicBool>,
    pub start_time: Instant,
    pub time_limit_ms: Option<u64>,
    pub killer_moves: [[Move; MAX_PLY]; 2],
    pub history_moves: [[i32; 64]; 12],
    pub counter_moves: [[Move; 64]; 12],
    pub lmr_table: [[i32; 64]; 64],
    pub pv_table: [[Move; MAX_PLY]; MAX_PLY],
    pub pv_length: [usize; MAX_PLY],
    pub follow_pv: bool,
    pub score_pv: bool,
}

impl Searcher {
    pub fn new(stopped: Arc<AtomicBool>) -> Self {
        let mut lmr_table = [[0i32; 64]; 64];
        for d in 1..64 {
            for m in 1..64 {
                let red = (0.75 + (d as f64).ln() * (m as f64).ln() / 2.25) as i32;
                lmr_table[d][m] = red.max(1);
            }
        }

        Self {
            nodes: 0,
            stopped,
            start_time: Instant::now(),
            time_limit_ms: None,
            killer_moves: [[Move::NULL; MAX_PLY]; 2],
            history_moves: [[0; 64]; 12],
            counter_moves: [[Move::NULL; 64]; 12],
            lmr_table,
            pv_table: [[Move::NULL; MAX_PLY]; MAX_PLY],
            pv_length: [0; MAX_PLY],
            follow_pv: false,
            score_pv: false,
        }
    }

    pub fn check_time(&self) -> bool {
        if self.stopped.load(Ordering::Relaxed) {
            return true;
        }
        if let Some(limit) = self.time_limit_ms {
            if self.start_time.elapsed().as_millis() as u64 >= limit {
                self.stopped.store(true, Ordering::Relaxed);
                return true;
            }
        }
        false
    }

    #[inline(always)]
    pub fn score_move(
        &self,
        board: &Board,
        m: Move,
        ply: usize,
        tt_move: Option<Move>,
        prev_move: Option<Move>,
        attack_tables: &AttackTables,
    ) -> i32 {
        if self.score_pv && ply < MAX_PLY && self.pv_table[0][ply] == m {
            return 20000;
        }

        if let Some(best) = tt_move {
            if best == m {
                return 15000;
            }
        }

        if m.is_capture() {
            let target_sq = m.target();
            let mut target_piece = Piece::P as usize;
            let (start_p, end_p) = if board.side == Color::White {
                (Piece::PawnBlack as usize, Piece::KingBlack as usize)
            } else {
                (Piece::P as usize, Piece::K as usize)
            };

            for p in start_p..=end_p {
                if get_bit(board.bitboards[p], target_sq) {
                    target_piece = p;
                    break;
                }
            }

            // SEE move ordering:
            // Winning or equal captures (SEE >= 0) sorted at the top (10000 + MVV-LVA).
            // Losing captures (SEE < 0) sorted below quiet moves (-10000 + MVV-LVA).
            if see(board, m, 0, attack_tables) {
                return MVV_LVA[m.piece() as usize][target_piece] + 10000;
            } else {
                return MVV_LVA[m.piece() as usize][target_piece] - 10000;
            }
        }

        if ply < MAX_PLY {
            if self.killer_moves[0][ply] == m {
                return 9000;
            } else if self.killer_moves[1][ply] == m {
                return 8000;
            }
        }

        // Countermove heuristic
        if let Some(prev) = prev_move {
            if self.counter_moves[prev.piece() as usize][prev.target()] == m {
                return 7000;
            }
        }

        self.history_moves[m.piece() as usize][m.target()]
    }

    pub fn sort_moves(
        &self,
        board: &Board,
        move_list: &mut MoveList,
        ply: usize,
        tt_move: Option<Move>,
        prev_move: Option<Move>,
        attack_tables: &AttackTables,
    ) {
        let mut scores = [0i32; 256];
        for i in 0..move_list.count {
            scores[i] = self.score_move(board, move_list.get(i), ply, tt_move, prev_move, attack_tables);
        }

        for i in 0..move_list.count {
            for j in (i + 1)..move_list.count {
                if scores[j] > scores[i] {
                    scores.swap(i, j);
                    move_list.moves.swap(i, j);
                }
            }
        }
    }

    pub fn quiescence(
        &mut self,
        mut alpha: i32,
        beta: i32,
        board: &mut Board,
        attack_tables: &AttackTables,
        masks: &EvalMasks,
        zobrist: &ZobristKeys,
    ) -> i32 {
        if (self.nodes & 2047) == 0 && self.check_time() {
            return 0;
        }

        self.nodes += 1;

        if board.ply >= MAX_PLY - 1 {
            return evaluate(board, attack_tables, masks);
        }

        let eval = evaluate(board, attack_tables, masks);

        if eval >= beta {
            return beta;
        }

        if eval > alpha {
            alpha = eval;
        }

        let king_sq = if board.side == Color::White {
            get_ls1b_index(board.bitboards[Piece::K as usize]).unwrap()
        } else {
            get_ls1b_index(board.bitboards[Piece::KingBlack as usize]).unwrap()
        };
        let in_check = attack_tables.is_square_attacked(
            king_sq,
            board.side.opposite(),
            &board.bitboards,
            board.occupancies[Color::Both as usize],
        );

        let mut move_list = MoveList::new();
        generate_moves(board, attack_tables, &mut move_list);
        self.sort_moves(board, &mut move_list, board.ply, None, None, attack_tables);

        let state = board.save_state();

        for i in 0..move_list.count {
            let m = move_list.get(i);

            // SEE pruning in quiescence: skip captures with negative SEE (losing trades)
            if !in_check && !see(board, m, 0, attack_tables) {
                continue;
            }

            // Delta pruning: skip quiet/low-value captures unable to raise alpha
            if !in_check && m.promoted().is_none() {
                let target_sq = m.target();
                let mut victim_val = 100;
                let (start_p, end_p) = if board.side == Color::White {
                    (Piece::PawnBlack as usize, Piece::QueenBlack as usize)
                } else {
                    (Piece::P as usize, Piece::Q as usize)
                };
                for p in start_p..=end_p {
                    if get_bit(board.bitboards[p], target_sq) {
                        victim_val = match p {
                            0 | 6 => 100,
                            1 | 7 => 320,
                            2 | 8 => 330,
                            3 | 9 => 500,
                            4 | 10 => 900,
                            _ => 100,
                        };
                        break;
                    }
                }
                if eval + victim_val + 200 < alpha {
                    continue;
                }
            }

            board.ply += 1;
            board.repetition_index += 1;
            board.repetition_table[board.repetition_index] = board.hash_key;

            if !make_move(board, m, MoveFlag::OnlyCaptures, attack_tables, zobrist) {
                board.ply -= 1;
                board.repetition_index -= 1;
                continue;
            }

            let score = -self.quiescence(-beta, -alpha, board, attack_tables, masks, zobrist);

            board.ply -= 1;
            board.repetition_index -= 1;
            board.restore_state(&state);

            if self.stopped.load(Ordering::Relaxed) {
                return 0;
            }

            if score > alpha {
                alpha = score;
                if score >= beta {
                    return beta;
                }
            }
        }

        alpha
    }

    pub fn negamax(
        &mut self,
        mut alpha: i32,
        beta: i32,
        mut depth: i32,
        board: &mut Board,
        attack_tables: &AttackTables,
        masks: &EvalMasks,
        zobrist: &ZobristKeys,
        tt: &mut TranspositionTable,
        prev_move: Option<Move>,
    ) -> i32 {
        if board.ply < MAX_PLY {
            self.pv_length[board.ply] = board.ply;
        }

        if board.ply > 0 && board.is_repetition() {
            return 0;
        }

        let pv_node = (beta - alpha) > 1;
        let mut tt_move = None;

        if board.ply > 0 {
            if let Some((score, best_m)) = tt.read(board.hash_key, alpha, beta, depth, board.ply) {
                tt_move = best_m;
                if !pv_node && score != NO_HASH_ENTRY {
                    return score;
                }
            }
        }

        if (self.nodes & 2047) == 0 && self.check_time() {
            return 0;
        }

        if depth <= 0 {
            return self.quiescence(alpha, beta, board, attack_tables, masks, zobrist);
        }

        if board.ply >= MAX_PLY - 1 {
            return evaluate(board, attack_tables, masks);
        }

        self.nodes += 1;

        // In check extension
        let king_sq = if board.side == Color::White {
            get_ls1b_index(board.bitboards[Piece::K as usize]).unwrap()
        } else {
            get_ls1b_index(board.bitboards[Piece::KingBlack as usize]).unwrap()
        };
        let in_check = attack_tables.is_square_attacked(
            king_sq,
            board.side.opposite(),
            &board.bitboards,
            board.occupancies[Color::Both as usize],
        );

        if in_check {
            depth += 1;
        }

        // Static evaluation (used for RFP and Futility Pruning)
        let static_eval = if !in_check {
            evaluate(board, attack_tables, masks)
        } else {
            -INFINITY
        };

        // Reverse Futility Pruning (Static Null Move Pruning)
        if depth <= 3 && !in_check && !pv_node && board.ply > 0 {
            if static_eval - 120 * depth >= beta {
                return beta;
            }
        }

        // Null move pruning
        if depth >= 3 && !in_check && board.ply > 0 {
            let state = board.save_state();
            board.ply += 1;
            board.repetition_index += 1;
            board.repetition_table[board.repetition_index] = board.hash_key;

            if board.enpassant != square::NO_SQ {
                board.hash_key ^= zobrist.enpassant_keys[board.enpassant];
            }
            board.enpassant = square::NO_SQ;
            board.side = board.side.opposite();
            board.hash_key ^= zobrist.side_key;

            let score = -self.negamax(
                -beta,
                -beta + 1,
                depth - 1 - 2,
                board,
                attack_tables,
                masks,
                zobrist,
                tt,
                None,
            );

            board.ply -= 1;
            board.repetition_index -= 1;
            board.restore_state(&state);

            if self.stopped.load(Ordering::Relaxed) {
                return 0;
            }

            if score >= beta {
                return beta;
            }
        }

        let mut move_list = MoveList::new();
        generate_moves(board, attack_tables, &mut move_list);

        if self.follow_pv {
            self.score_pv = false;
            for i in 0..move_list.count {
                if board.ply < MAX_PLY && self.pv_table[0][board.ply] == move_list.get(i) {
                    self.score_pv = true;
                    self.follow_pv = true;
                    break;
                }
            }
        }

        self.sort_moves(board, &mut move_list, board.ply, tt_move, prev_move, attack_tables);

        let mut legal_moves = 0;
        let mut moves_searched = 0;
        let mut quiet_moves_searched = 0;
        let mut quiet_moves = [Move::NULL; 64];
        let mut quiet_count = 0;
        let mut hash_flag = HASH_FLAG_ALPHA;
        let mut best_move = None;
        let state = board.save_state();

        for i in 0..move_list.count {
            let m = move_list.get(i);

            // Late Move Pruning (LMP): at non-PV shallow depth, prune excess quiet moves
            if !pv_node
                && depth <= 4
                && !in_check
                && !m.is_capture()
                && m.promoted().is_none()
            {
                let lmp_threshold = 3 + 3 * depth * depth;
                if quiet_moves_searched >= lmp_threshold as usize {
                    continue;
                }
            }

            // Futility Pruning for quiet moves at shallow depth
            if depth <= 2
                && !in_check
                && !pv_node
                && !m.is_capture()
                && m.promoted().is_none()
                && legal_moves > 0
                && alpha < MATE_SCORE
                && static_eval + 150 * depth <= alpha
            {
                continue;
            }

            board.ply += 1;
            board.repetition_index += 1;
            board.repetition_table[board.repetition_index] = board.hash_key;

            if !make_move(board, m, MoveFlag::AllMoves, attack_tables, zobrist) {
                board.ply -= 1;
                board.repetition_index -= 1;
                continue;
            }

            legal_moves += 1;
            if !m.is_capture() {
                quiet_moves_searched += 1;
                if quiet_count < 64 {
                    quiet_moves[quiet_count] = m;
                    quiet_count += 1;
                }
            }
            let mut score;

            // Full depth search on first move
            if moves_searched == 0 {
                score = -self.negamax(
                    -beta,
                    -alpha,
                    depth - 1,
                    board,
                    attack_tables,
                    masks,
                    zobrist,
                    tt,
                    Some(m),
                );
            } else {
                // Late Move Reduction (LMR) using precomputed logarithmic table
                if moves_searched >= FULL_DEPTH_MOVES
                    && depth >= REDUCTION_LIMIT
                    && !in_check
                    && !m.is_capture()
                    && m.promoted().is_none()
                {
                    let reduction = if depth <= 5 {
                        1
                    } else {
                        self.lmr_table[(depth as usize).min(63)][moves_searched.min(63)]
                            .clamp(1, depth - 3)
                    };
                    score = -self.negamax(
                        -alpha - 1,
                        -alpha,
                        depth - 1 - reduction,
                        board,
                        attack_tables,
                        masks,
                        zobrist,
                        tt,
                        Some(m),
                    );
                } else {
                    score = alpha + 1;
                }

                // Principal Variation Search (PVS)
                if score > alpha {
                    score = -self.negamax(
                        -alpha - 1,
                        -alpha,
                        depth - 1,
                        board,
                        attack_tables,
                        masks,
                        zobrist,
                        tt,
                        Some(m),
                    );

                    if score > alpha && score < beta {
                        score = -self.negamax(
                            -beta,
                            -alpha,
                            depth - 1,
                            board,
                            attack_tables,
                            masks,
                            zobrist,
                            tt,
                            Some(m),
                        );
                    }
                }
            }

            board.ply -= 1;
            board.repetition_index -= 1;
            board.restore_state(&state);

            if self.stopped.load(Ordering::Relaxed) {
                return 0;
            }

            moves_searched += 1;

            if score > alpha {
                hash_flag = HASH_FLAG_EXACT;
                best_move = Some(m);
                alpha = score;

                if board.ply < MAX_PLY {
                    self.pv_table[board.ply][board.ply] = m;
                    for next_ply in (board.ply + 1)..self.pv_length[board.ply + 1] {
                        self.pv_table[board.ply][next_ply] = self.pv_table[board.ply + 1][next_ply];
                    }
                    self.pv_length[board.ply] = self.pv_length[board.ply + 1];
                }

                if !m.is_capture() {
                    self.history_moves[m.piece() as usize][m.target()] += depth;
                }

                // Beta cutoff
                if score >= beta {
                    tt.write(board.hash_key, beta, depth, HASH_FLAG_BETA, board.ply, Some(m));
                    if !m.is_capture() {
                        if board.ply < MAX_PLY {
                            self.killer_moves[1][board.ply] = self.killer_moves[0][board.ply];
                            self.killer_moves[0][board.ply] = m;
                        }
                        if let Some(prev) = prev_move {
                            self.counter_moves[prev.piece() as usize][prev.target()] = m;
                        }
                        // History heuristic bonus & malus
                        let bonus = (depth * depth).min(400);
                        self.history_moves[m.piece() as usize][m.target()] += bonus;
                        self.history_moves[m.piece() as usize][m.target()] =
                            self.history_moves[m.piece() as usize][m.target()].clamp(-20000, 20000);

                        // History Malus: penalize earlier quiet moves that failed to cause a cutoff
                        for k in 0..quiet_count.saturating_sub(1) {
                            let qm = quiet_moves[k];
                            self.history_moves[qm.piece() as usize][qm.target()] -= bonus;
                            self.history_moves[qm.piece() as usize][qm.target()] =
                                self.history_moves[qm.piece() as usize][qm.target()].clamp(-20000, 20000);
                        }
                    }
                    return beta;
                }
            }
        }

        if legal_moves == 0 {
            if in_check {
                return -MATE_VALUE + board.ply as i32;
            } else {
                return 0;
            }
        }

        tt.write(board.hash_key, alpha, depth, hash_flag, board.ply, best_move);
        alpha
    }

    pub fn search_position(
        &mut self,
        max_depth: usize,
        limits: &SearchLimits,
        board: &mut Board,
        attack_tables: &AttackTables,
        masks: &EvalMasks,
        zobrist: &ZobristKeys,
        tt: &mut TranspositionTable,
    ) -> Option<Move> {
        self.nodes = 0;
        self.stopped.store(false, Ordering::Relaxed);
        self.start_time = Instant::now();

        // Calculate time limit
        if let Some(movetime) = limits.movetime {
            self.time_limit_ms = Some(movetime);
        } else if let Some(time) = limits.time {
            let moves_to_go = (limits.movestogo.unwrap_or(30) as u64).clamp(10, 50);
            let time_alloc = (time / moves_to_go) + (limits.inc * 3 / 4);
            // Safety margin
            self.time_limit_ms = Some(time_alloc.min(time.saturating_sub(30)));
        } else {
            self.time_limit_ms = None;
        }

        self.follow_pv = false;
        self.score_pv = false;
        self.killer_moves = [[Move::NULL; MAX_PLY]; 2];
        self.history_moves = [[0; 64]; 12];
        self.counter_moves = [[Move::NULL; 64]; 12];
        self.pv_table = [[Move::NULL; MAX_PLY]; MAX_PLY];
        self.pv_length = [0; MAX_PLY];

        let mut current_depth = 1;
        let mut alpha = -INFINITY;
        let mut beta = INFINITY;
        let mut best_move = None;

        while current_depth <= max_depth {
            if self.stopped.load(Ordering::Relaxed) {
                break;
            }

            self.follow_pv = true;
            let score = self.negamax(
                alpha,
                beta,
                current_depth as i32,
                board,
                attack_tables,
                masks,
                zobrist,
                tt,
                None,
            );

            // If stopped during negamax, do not trust this partial search result
            if self.stopped.load(Ordering::Relaxed) {
                break;
            }

            // Aspiration window fell outside: widen to full window and retry at SAME current_depth
            if (score <= alpha) || (score >= beta) {
                alpha = -INFINITY;
                beta = INFINITY;
                continue;
            }

            alpha = score - 50;
            beta = score + 50;

            if self.pv_length[0] > 0 {
                best_move = Some(self.pv_table[0][0]);
                let elapsed_ms = self.start_time.elapsed().as_millis().max(1) as u64;
                let nps = (self.nodes as f64 / (elapsed_ms as f64 / 1000.0)) as u64;

                let mut pv_str = String::new();
                for i in 0..self.pv_length[0] {
                    pv_str.push_str(&self.pv_table[0][i].to_uci());
                    pv_str.push(' ');
                }

                if score > -MATE_VALUE && score < -MATE_SCORE {
                    println!(
                        "info score mate {} depth {} nodes {} time {} nps {} pv {}",
                        -(score + MATE_VALUE) / 2 - 1,
                        current_depth,
                        self.nodes,
                        elapsed_ms,
                        nps,
                        pv_str.trim()
                    );
                } else if score > MATE_SCORE && score < MATE_VALUE {
                    println!(
                        "info score mate {} depth {} nodes {} time {} nps {} pv {}",
                        (MATE_VALUE - score) / 2 + 1,
                        current_depth,
                        self.nodes,
                        elapsed_ms,
                        nps,
                        pv_str.trim()
                    );
                } else {
                    println!(
                        "info score cp {} depth {} nodes {} time {} nps {} pv {}",
                        score,
                        current_depth,
                        self.nodes,
                        elapsed_ms,
                        nps,
                        pv_str.trim()
                    );
                }
            }

            current_depth += 1;
        }

        if let Some(m) = best_move {
            println!("bestmove {}", m.to_uci());
        } else {
            // Fallback: pick any legal move
            let mut move_list = MoveList::new();
            generate_moves(board, attack_tables, &mut move_list);
            let state = board.save_state();
            for i in 0..move_list.count {
                let m = move_list.get(i);
                if make_move(board, m, MoveFlag::AllMoves, attack_tables, zobrist) {
                    board.restore_state(&state);
                    println!("bestmove {}", m.to_uci());
                    return Some(m);
                }
            }
            println!("bestmove (none)");
        }

        best_move
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::START_POSITION;

    #[test]
    fn test_search_start_position() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let masks = EvalMasks::new();
        let mut tt = TranspositionTable::new(16);
        let stopped = Arc::new(AtomicBool::new(false));
        let mut searcher = Searcher::new(stopped);

        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);

        let limits = SearchLimits {
            depth: Some(3),
            ..Default::default()
        };

        let best_move = searcher.search_position(
            3,
            &limits,
            &mut board,
            &attack_tables,
            &masks,
            &zobrist,
            &mut tt,
        );

        assert!(best_move.is_some());
    }

    #[test]
    fn test_search_mate_in_one() {
        // Scholar's mate position: White to play and mate with Qxf7#
        // 1. e4 e5 2. Qh5 Nc6 3. Bc4 Nf6 4. Qxf7#
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let masks = EvalMasks::new();
        let mut tt = TranspositionTable::new(16);
        let stopped = Arc::new(AtomicBool::new(false));
        let mut searcher = Searcher::new(stopped);

        let mut board = Board::new();
        board.parse_fen("r1bqkb1r/pppp1ppp/2n2n2/4p2Q/2B1P3/8/PPPP1PPP/RNB1K1NR w KQkq - 4 4", &zobrist);

        let limits = SearchLimits {
            depth: Some(2),
            ..Default::default()
        };

        let best_move = searcher.search_position(
            2,
            &limits,
            &mut board,
            &attack_tables,
            &masks,
            &zobrist,
            &mut tt,
        );

        assert_eq!(best_move.unwrap().to_uci(), "h5f7");
    }

    #[test]
    fn test_lmr_table_generation() {
        let stopped = Arc::new(AtomicBool::new(false));
        let searcher = Searcher::new(stopped);
        assert_eq!(searcher.lmr_table[0][0], 0);
        assert!(searcher.lmr_table[3][4] >= 1);
        assert!(searcher.lmr_table[8][10] >= 2);
        assert!(searcher.lmr_table[16][16] >= 3);
        // Table should be non-decreasing with depth and move count
        assert!(searcher.lmr_table[10][10] >= searcher.lmr_table[5][5]);
    }

    #[test]
    fn test_counter_move_scoring() {
        let stopped = Arc::new(AtomicBool::new(false));
        let mut searcher = Searcher::new(stopped);
        let board = Board::new();

        let prev = Move::encode(square::e2, square::e4, Piece::P, None, false, true, false, false);
        let reply = Move::encode(square::e7, square::e5, Piece::PawnBlack, None, false, true, false, false);

        searcher.counter_moves[prev.piece() as usize][prev.target()] = reply;

        let attack_tables = AttackTables::new();
        let score = searcher.score_move(&board, reply, 0, None, Some(prev), &attack_tables);
        assert_eq!(score, 7000);
    }
}
