use chess_engine::board::Board;
use chess_engine::magic::AttackTables;
use chess_engine::perft::perft_driver;
use chess_engine::zobrist::ZobristKeys;

#[test]
fn test_perft_suite_startpos_depth5() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    board.parse_fen("rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1", &zobrist);

    assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 20);
    assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 400);
    assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 8902);
    assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 197281);
    assert_eq!(perft_driver(5, &mut board, &attack_tables, &zobrist), 4865609);
}

#[test]
fn test_perft_suite_kiwipete_depth4() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    board.parse_fen("r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1", &zobrist);

    assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 48);
    assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 2039);
    assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 97862);
    assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 4085603);
}

#[test]
fn test_perft_suite_pos3_depth5() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    board.parse_fen("8/2p5/3p4/KP5r/1R3p1k/8/4P1P1/8 w - - 0 1", &zobrist);

    assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 14);
    assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 191);
    assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 2812);
    assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 43238);
    assert_eq!(perft_driver(5, &mut board, &attack_tables, &zobrist), 674624);
}

#[test]
fn test_perft_suite_pos4_depth4() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    board.parse_fen("r3k2r/Pppp1ppp/1b3nbN/nP6/BBP1P3/q4N2/Pp1P2PP/R2Q1RK1 w kq - 0 1", &zobrist);

    assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 6);
    assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 264);
    assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 9467);
    assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 422333);
    assert_eq!(perft_driver(5, &mut board, &attack_tables, &zobrist), 15833292);
}

#[test]
fn test_perft_suite_pos5_depth4() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    board.parse_fen("rnbq1k1r/pp1Pbppp/2p5/8/2B5/8/PPP1NnPP/RNBQK2R w KQ - 1 8", &zobrist);

    assert_eq!(perft_driver(1, &mut board, &attack_tables, &zobrist), 44);
    assert_eq!(perft_driver(2, &mut board, &attack_tables, &zobrist), 1486);
    assert_eq!(perft_driver(3, &mut board, &attack_tables, &zobrist), 62379);
    assert_eq!(perft_driver(4, &mut board, &attack_tables, &zobrist), 2103487);
}
