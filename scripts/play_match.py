import os
import sys
import chess
import chess.engine
import time
import json

RUST_ENGINE_PATH = os.path.abspath(r"C:\Users\manth\.t3\projects\chess-in-rust\target\release\chess-engine.exe")
STOCKFISH_PATH = os.path.abspath(r"C:\Users\manth\AppData\Local\Microsoft\WinGet\Packages\Stockfish.Stockfish_Microsoft.Winget.Source_8wekyb3d8bbwe\stockfish\stockfish-windows-x86-64-universal.exe")
BBC_PATH = os.path.abspath(r"C:\Users\manth\.t3\projects\chess-in-rust\reference\bbc\src\bbc_nnue\bbc.exe")

def play_game(white_path, black_path, white_name="White", black_name="Black", time_per_move=0.5, max_moves=100):
    print(f"Starting Game: {white_name} (White) vs {black_name} (Black)...")
    board = chess.Board()
    
    white_engine = chess.engine.SimpleEngine.popen_uci(white_path)
    black_engine = chess.engine.SimpleEngine.popen_uci(black_path)
    
    # Configure evaluator Stockfish to judge every move
    evaluator = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
    
    moves_history = []
    eval_history = []
    
    move_count = 0
    try:
        while not board.is_game_over() and move_count < max_moves:
            current_engine = white_engine if board.turn == chess.WHITE else black_engine
            engine_name = white_name if board.turn == chess.WHITE else black_name
            
            # Stockfish evaluation before move
            eval_before = evaluator.analyse(board, chess.engine.Limit(depth=10))
            score_before = eval_before["score"].white().score(mate_score=10000)
            
            # Request move from current engine
            limit = chess.engine.Limit(time=time_per_move)
            result = current_engine.play(board, limit)
            move = result.move
            
            if move is None:
                print(f"{engine_name} failed to return a move!")
                break
                
            san_move = board.san(move)
            board.push(move)
            move_count += 1
            
            # Stockfish evaluation after move
            eval_after = evaluator.analyse(board, chess.engine.Limit(depth=10))
            score_after = eval_after["score"].white().score(mate_score=10000)
            
            # Centipawn loss for current side
            if board.turn == chess.BLACK: # White just moved
                cp_diff = (score_before - score_after) if score_before is not None and score_after is not None else 0
            else: # Black just moved
                cp_diff = (score_after - score_before) if score_before is not None and score_after is not None else 0
            
            moves_history.append({
                "ply": move_count,
                "side": "White" if board.turn == chess.BLACK else "Black",
                "engine": engine_name,
                "san": san_move,
                "uci": move.uci(),
                "eval_after": score_after,
                "cp_loss": max(0, cp_diff),
                "fen": board.fen()
            })
            
            print(f"Move {move_count}: {engine_name} played {san_move} (Eval: {score_after} cp, CP loss: {max(0, cp_diff)})")
            
    finally:
        white_engine.quit()
        black_engine.quit()
        evaluator.quit()
        
    result_str = board.result() if board.is_game_over() else "1/2-1/2 (Move limit reached)"
    print(f"\nGame Over! Result: {result_str}")
    
    return {
        "white": white_name,
        "black": black_name,
        "result": result_str,
        "total_plies": move_count,
        "moves": moves_history,
        "final_fen": board.fen()
    }

def run_tactics_suite():
    print("\n--- Running Tactical Diagnostic Suite ---")
    tactical_positions = [
        # (FEN, Expected best move UCI, Description)
        ("r1bqkb1r/pppp1ppp/2n5/4p2Q/2B1n3/8/PPPP1PPP/RNB1K1NR w KQkq - 0 5", "h5f7", "Scholar's Mate in 1"),
        ("6k1/5ppp/8/8/8/8/5PPP/4R1K1 w - - 0 1", "e1e8", "Back-rank Mate in 1"),
        ("r1b1kb1r/pppp1ppp/8/4q3/8/8/PPPP1PPP/RNBQKB1R w KQkq - 0 1", "d1e2", "Block check or pin queen"),
        ("r2qkb1r/pp2pppp/2n2n2/3p4/3P2b1/2PB1N2/PP3PPP/RNBQK2R w KQkq - 1 7", "e1g1", "Standard castling development"),
        ("r1bqk2r/pppp1ppp/2n2n2/2b1p3/2B1P3/3P1N2/PPP2PPP/RNBQK2R w KQkq - 1 5", "b1c3", "Quiet opening development"),
        ("r1b2rk1/pp3ppp/2n5/2qp4/8/1P1B1N2/P1P2PPP/R2Q1RK1 w - - 0 14", "d3h7", "Greek gift sacrifice / tactical strike"),
    ]
    
    engine = chess.engine.SimpleEngine.popen_uci(RUST_ENGINE_PATH)
    results = []
    
    try:
        for idx, (fen, expected_uci, desc) in enumerate(tactical_positions, 1):
            board = chess.Board(fen)
            res = engine.play(board, chess.engine.Limit(depth=6))
            played_uci = res.move.uci() if res.move else "none"
            is_match = played_uci == expected_uci
            print(f"Test {idx} [{desc}]: Expected {expected_uci}, Played {played_uci} -> {'PASS' if is_match else 'FAIL'}")
            results.append({
                "test": idx,
                "desc": desc,
                "fen": fen,
                "expected": expected_uci,
                "played": played_uci,
                "pass": is_match
            })
    finally:
        engine.quit()
        
    return results

if __name__ == "__main__":
    mode = sys.argv[1] if len(sys.argv) > 1 else "all"
    
    if mode in ("tactics", "all"):
        tactics_results = run_tactics_suite()
        with open("tactics_results.json", "w") as f:
            json.dump(tactics_results, f, indent=2)
            
    if mode in ("game", "all"):
        # Match: Rust Engine (White) vs Stockfish limited (Black)
        game_result = play_game(
            white_path=RUST_ENGINE_PATH,
            black_path=STOCKFISH_PATH,
            white_name="Rust-BBC-Engine",
            black_name="Stockfish-19",
            time_per_move=0.3,
            max_moves=60
        )
        with open("game_analysis.json", "w") as f:
            json.dump(game_result, f, indent=2)
