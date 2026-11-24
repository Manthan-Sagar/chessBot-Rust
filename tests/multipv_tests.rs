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

#[test]
fn test_multipv_deep_lines_and_board_state_integrity() {
    use chess_engine::movegen::{make_move, MoveFlag};
    use std::collections::HashSet;

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
    assert_eq!(board.ply, 0, "board.ply must return to 0 after MultiPV search");
    assert_eq!(board.repetition_index, 0, "board.repetition_index must return to 0 after MultiPV search");
    assert_eq!(searcher.last_multipv_lines.len(), 3, "MultiPV=3 must produce 3 lines");

    let mut distinct_roots = HashSet::new();
    for (pv_num, _score, pv) in &searcher.last_multipv_lines {
        assert!(
            pv.len() >= 4,
            "MultiPV {} at depth 6 must have >= 4 moves, got {} ({:?})",
            pv_num,
            pv.len(),
            pv.iter().map(|m| m.to_uci()).collect::<Vec<_>>()
        );
        distinct_roots.insert(pv[0].to_uci());

        // Verify all moves in the PV are legal when replayed
        let mut test_board = board.clone();
        for &m in pv {
            assert!(
                make_move(&mut test_board, m, MoveFlag::AllMoves, &attack_tables, &zobrist),
                "PV move {} in MultiPV {} must be legal",
                m.to_uci(),
                pv_num
            );
        }
    }
    assert_eq!(distinct_roots.len(), 3, "All 3 MultiPV lines must start with distinct root moves");
}
