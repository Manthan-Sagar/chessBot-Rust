use std::sync::atomic::AtomicBool;
use std::sync::Arc;

use chess_engine::board::{Board, START_POSITION};
use chess_engine::evaluate::EvalMasks;
use chess_engine::magic::AttackTables;
use chess_engine::search::{SearchLimits, Searcher};
use chess_engine::tt::TranspositionTable;
use chess_engine::uci::UciEngine;
use chess_engine::zobrist::ZobristKeys;

#[test]
fn test_uci_multipv_option() {
    let mut engine = UciEngine::new();
    assert_eq!(engine.multipv, 1);

    engine.handle_command("setoption name MultiPV value 3");
    assert_eq!(engine.multipv, 3);

    engine.handle_command("setoption name MultiPV value 5");
    assert_eq!(engine.multipv, 5);

    // Clamped to max 5
    engine.handle_command("setoption name MultiPV value 10");
    assert_eq!(engine.multipv, 5);

    // Clamped to min 1
    engine.handle_command("setoption name MultiPV value 0");
    assert_eq!(engine.multipv, 1);
}

#[test]
fn test_multipv_search() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let masks = EvalMasks::new();
    let mut tt = TranspositionTable::new(16);
    let stopped = Arc::new(AtomicBool::new(false));
    let mut searcher = Searcher::new(stopped);
    searcher.multipv = 3;

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

    assert!(best_move.is_some(), "MultiPV search should produce a best move");
}
