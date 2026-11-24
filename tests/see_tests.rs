use chess_engine::board::Board;
use chess_engine::magic::AttackTables;
use chess_engine::movegen::generate_moves;
use chess_engine::moves::{Move, MoveList};
use chess_engine::see::{see, see_eval};
use chess_engine::zobrist::ZobristKeys;

fn find_move(board: &Board, attack_tables: &AttackTables, uci_str: &str) -> Move {
    let mut move_list = MoveList::new();
    generate_moves(board, attack_tables, &mut move_list);
    for i in 0..move_list.count {
        let m = move_list.get(i);
        if m.to_uci() == uci_str {
            return m;
        }
    }
    panic!("Move {} not found in position", uci_str);
}

#[test]
fn test_see_winning_capture_pawn_takes_queen() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White pawn on d4, Black queen on e5 defended by pawn on d6
    board.parse_fen("4k3/8/3p4/4q3/3P4/8/8/4K3 w - - 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "d4e5");

    let eval = see_eval(&board, m, &attack_tables);
    assert!(eval > 0, "Pawn takes Queen must have positive SEE: {}", eval);
    assert_eq!(eval, 800); // 900 (Q) - 100 (P) = 800
    assert!(see(&board, m, 0, &attack_tables));
}

#[test]
fn test_see_losing_capture_queen_takes_defended_pawn() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White queen on d1, Black pawn on d5 defended by pawn on e6
    board.parse_fen("4k3/8/4p3/3p4/8/8/8/3QK3 w - - 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "d1d5");

    let eval = see_eval(&board, m, &attack_tables);
    assert!(eval < 0, "Queen takes defended pawn must have negative SEE: {}", eval);
    assert_eq!(eval, -800); // 100 (P) - 900 (Q) = -800
    assert!(!see(&board, m, 0, &attack_tables));
}

#[test]
fn test_see_equal_trade_knights() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White knight on c3, Black knight on d5 defended by pawn on c6
    board.parse_fen("4k3/8/2p5/3n4/8/2N5/8/4K3 w - - 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "c3d5");

    let eval = see_eval(&board, m, &attack_tables);
    assert_eq!(eval, 0, "Equal knight trade should have SEE == 0: {}", eval);
    assert!(see(&board, m, 0, &attack_tables));
}

#[test]
fn test_see_undefended_hanging_piece() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White queen on a1, Black rook on a8 completely undefended
    board.parse_fen("r3k3/8/8/8/8/8/8/Q3K3 w - - 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "a1a8");

    let eval = see_eval(&board, m, &attack_tables);
    assert_eq!(eval, 500, "Capturing undefended rook should yield 500: {}", eval);
    assert!(see(&board, m, 0, &attack_tables));
}

#[test]
fn test_see_en_passant() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White pawn on e5, Black plays d7-d5 -> en passant on d6
    board.parse_fen("4k3/8/8/3Pp3/8/8/8/4K3 w - e6 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "d5e6");

    let eval = see_eval(&board, m, &attack_tables);
    assert_eq!(eval, 100, "En passant capture should yield 100: {}", eval);
    assert!(see(&board, m, 0, &attack_tables));
}

#[test]
fn test_see_xray_battery() {
    let zobrist = ZobristKeys::new();
    let attack_tables = AttackTables::new();
    let mut board = Board::new();
    // White R on d1, Q on d2 (battery on d-file).
    // Black R on d7 defended by R on d8.
    // White plays Rd2xd7. Black recaptures Rd8xd7. White recaptures Rd1xd7.
    // White nets +500 (trades one rook, wins the second).
    board.parse_fen("3rr1k1/3r4/8/8/8/8/3R4/3R2K1 w - - 0 1", &zobrist);
    let m = find_move(&board, &attack_tables, "d2d7");

    let eval = see_eval(&board, m, &attack_tables);
    assert_eq!(eval, 500, "X-ray battery winning rook trade: {}", eval);
    assert!(see(&board, m, 0, &attack_tables));
}
