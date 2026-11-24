use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use chess_engine::board::Board;
use chess_engine::evaluate::EvalMasks;
use chess_engine::magic::AttackTables;
use chess_engine::search::{SearchLimits, Searcher};
use chess_engine::tt::TranspositionTable;
use chess_engine::zobrist::ZobristKeys;

#[test]
fn test_regression_aspiration_window_mate_defense() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let masks = EvalMasks::new();
    let mut tt = TranspositionTable::new(16);
    let stopped = Arc::new(AtomicBool::new(false));
    let mut searcher = Searcher::new(stopped);

    let mut board = Board::new();
    // FEN from Game 1, ply 53 blunder position:
    // White to move. Black threatens Qh3 followed by Qg2#.
    // White must play Qd3 (c2d3).
    board.parse_fen(
        "N1b2r1k/b1P2ppB/3n4/p7/6q1/2P2pP1/P1Q2P1P/R4RK1 w - - 1 27",
        &zobrist,
    );

    let limits = SearchLimits {
        depth: Some(6),
        ..Default::default()
    };

    let best_move = searcher.search_position(
        6,
        &limits,
        &mut board,
        &attack_tables,
        &masks,
        &zobrist,
        &mut tt,
    );

    assert!(best_move.is_some());
    assert_eq!(
        best_move.unwrap().to_uci(),
        "c2d3",
        "White must play c2d3 (Qd3) to defend the mate threat!"
    );
}

#[test]
fn test_regression_avoid_queenside_castling_into_open_files() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let masks = EvalMasks::new();
    let mut tt = TranspositionTable::new(16);
    let stopped = Arc::new(AtomicBool::new(false));
    let mut searcher = Searcher::new(stopped);

    let mut board = Board::new();
    // FEN from Game 2 Scandinavian position where Black should not castle queenside into open b/c files:
    board.parse_fen(
        "r3kb1r/p1q1pppp/2p2n2/5b2/2N5/5N2/PPPP1PPP/R1BQ1RK1 b kq - 3 9",
        &zobrist,
    );

    let limits = SearchLimits {
        depth: Some(8),
        ..Default::default()
    };

    let best_move = searcher.search_position(
        8,
        &limits,
        &mut board,
        &attack_tables,
        &masks,
        &zobrist,
        &mut tt,
    );

    assert!(best_move.is_some());
    assert_ne!(
        best_move.unwrap().to_uci(),
        "e8c8",
        "Black should not castle queenside (e8c8) into open files!"
    );
}
