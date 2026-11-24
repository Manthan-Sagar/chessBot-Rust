//! Perft (Performance Test) move path enumeration.

use crate::board::Board;
use crate::magic::AttackTables;
use crate::movegen::{generate_moves, make_move, MoveFlag};
use crate::moves::MoveList;
use crate::zobrist::ZobristKeys;

pub fn perft_driver(
    depth: usize,
    board: &mut Board,
    attack_tables: &AttackTables,
    zobrist: &ZobristKeys,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, attack_tables, &mut move_list);

    let mut nodes: u64 = 0;
    let state = board.save_state();

    for i in 0..move_list.count {
        let m = move_list.get(i);
        if !make_move(board, m, MoveFlag::AllMoves, attack_tables, zobrist) {
            continue;
        }

        nodes += perft_driver(depth - 1, board, attack_tables, zobrist);
        board.restore_state(&state);
    }

    nodes
}

pub fn perft_divide(
    depth: usize,
    board: &mut Board,
    attack_tables: &AttackTables,
    zobrist: &ZobristKeys,
) -> u64 {
    if depth == 0 {
        return 1;
    }

    let mut move_list = MoveList::new();
    generate_moves(board, attack_tables, &mut move_list);

    let mut total_nodes: u64 = 0;
    let state = board.save_state();

    println!("\n     Performance test (perft divide):\n");

    for i in 0..move_list.count {
        let m = move_list.get(i);
        if !make_move(board, m, MoveFlag::AllMoves, attack_tables, zobrist) {
            continue;
        }

        let cum_nodes = perft_driver(depth - 1, board, attack_tables, zobrist);
        total_nodes += cum_nodes;
        board.restore_state(&state);

        println!("     move: {} nodes: {}", m.to_uci(), cum_nodes);
    }

    println!("\n     Total nodes: {}\n", total_nodes);
    total_nodes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::board::{START_POSITION, TRICKY_POSITION};

    #[test]
    fn test_perft_startpos_depth1_to_4() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen(START_POSITION, &zobrist);

        assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 20);
        assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 400);
        assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 8902);
        assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 197281);
    }

    #[test]
    fn test_perft_kiwipete_depth1_to_3() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen(TRICKY_POSITION, &zobrist);

        assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 48);
        assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 2039);
        assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 97862);
    }

    #[test]
    fn test_perft_pos3_depth1_to_3() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &zobrist);

        assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 14);
        assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 191);
        assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 2812);
    }

    #[test]
    fn test_perft_pos4_depth1_to_3() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &zobrist);

        assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 6);
        assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 264);
        assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 9467);
    }

    #[test]
    fn test_perft_pos5_depth1_to_3() {
        let zobrist = ZobristKeys::new();
        let attack_tables = AttackTables::new();
        let mut board = Board::new();
        board.parse_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &zobrist);

        assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 44);
        assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 1486);
        assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 62379);
    }
}
