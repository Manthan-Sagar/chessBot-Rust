import os
import sys
import json
import chess
import chess.engine
import time

RUST_ENGINE_PATH = os.path.abspath(r"C:\Users\manth\Documents\chess-in-rust\target\release\chess-engine.exe")
STOCKFISH_PATH = os.path.abspath(r"C:\Users\manth\AppData\Local\Microsoft\WinGet\Packages\Stockfish.Stockfish_Microsoft.Winget.Source_8wekyb3d8bbwe\stockfish\stockfish-windows-x86-64-universal.exe")

def play_elo_game(white_is_rust, target_elo, time_per_move=0.3, max_moves=60):
    white_name = "RustEngine" if white_is_rust else f"Stockfish-{target_elo}"
    black_name = f"Stockfish-{target_elo}" if white_is_rust else "RustEngine"
    print(f"\n--- Match: {white_name} (White) vs {black_name} (Black) [SF Elo: {target_elo}] ---")
    
    board = chess.Board()
    
    rust_engine = chess.engine.SimpleEngine.popen_uci(RUST_ENGINE_PATH)
    sf_engine = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
    evaluator = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
    
    # Configure calibrated Stockfish Elo
    sf_engine.configure({"UCI_LimitStrength": True, "UCI_Elo": target_elo})
    
    moves_history = []
    move_count = 0
    rust_losses = []
    
    try:
        while not board.is_game_over() and move_count < max_moves:
            turn_is_rust = (board.turn == chess.WHITE) if white_is_rust else (board.turn == chess.BLACK)
            curr_engine = rust_engine if turn_is_rust else sf_engine
            curr_name = "RustEngine" if turn_is_rust else f"Stockfish-{target_elo}"
            
            # Stockfish evaluation before move (neutral evaluator at depth 10)
            eval_before = evaluator.analyse(board, chess.engine.Limit(depth=10))
            score_before = eval_before["score"].white().score(mate_score=10000)
            
            res = curr_engine.play(board, chess.engine.Limit(time=time_per_move))
            move = res.move
            if move is None:
                print(f"{curr_name} forfeited (no move returned)")
                break
                
            san_move = board.san(move)
            board.push(move)
            move_count += 1
            
            eval_after = evaluator.analyse(board, chess.engine.Limit(depth=10))
            score_after = eval_after["score"].white().score(mate_score=10000)
            
            if board.turn == chess.BLACK: # White moved
                cp_diff = (score_before - score_after) if score_before is not None and score_after is not None else 0
            else: # Black moved
                cp_diff = (score_after - score_before) if score_before is not None and score_after is not None else 0
                
            loss = max(0, cp_diff)
            if turn_is_rust:
                rust_losses.append(min(loss, 1000))
                
            moves_history.append({
                "ply": move_count,
                "player": curr_name,
                "san": san_move,
                "eval": score_after,
                "cp_loss": loss
            })
            
            if move_count % 10 == 0 or loss > 100:
                print(f"  Ply {move_count}: {curr_name} played {san_move} (Eval: {score_after} cp, Loss: {loss} cp)")
                
    finally:
        rust_engine.quit()
        sf_engine.quit()
        evaluator.quit()
        
    result_str = board.result() if board.is_game_over() else "1/2-1/2 (Move limit reached)"
    rust_score = 0.5
    if result_str == "1-0":
        rust_score = 1.0 if white_is_rust else 0.0
    elif result_str == "0-1":
        rust_score = 0.0 if white_is_rust else 1.0
        
    acpl = sum(rust_losses) / len(rust_losses) if rust_losses else 0.0
    print(f"Result: {result_str} | Rust Score: {rust_score} | Rust ACPL: {acpl:.1f} cp")
    
    return {
        "target_elo": target_elo,
        "white_is_rust": white_is_rust,
        "result": result_str,
        "rust_score": rust_score,
        "rust_acpl": acpl,
        "total_plies": move_count,
        "moves": moves_history
    }

def run_elo_benchmark(elos=[2400, 2500, 2600]):
    print(f"=== Starting Calibrated Stockfish Elo Benchmark: {elos} ===")
    results = {}
    
    for elo in elos:
        print(f"\n==========================================")
        print(f"      TESTING ELO TIER: {elo}")
        print(f"==========================================")
        # Game 1: Rust as White
        g1 = play_elo_game(white_is_rust=True, target_elo=elo)
        # Game 2: Rust as Black
        g2 = play_elo_game(white_is_rust=False, target_elo=elo)
        
        total_score = g1["rust_score"] + g2["rust_score"]
        avg_acpl = (g1["rust_acpl"] + g2["rust_acpl"]) / 2.0
        
        results[str(elo)] = {
            "score": total_score,
            "max_score": 2.0,
            "pct": (total_score / 2.0) * 100,
            "avg_acpl": avg_acpl,
            "g1": g1,
            "g2": g2
        }
        print(f"\n>>> Elo {elo} Series Result: {total_score}/2.0 ({(total_score/2.0)*100:.1f}%) | Avg ACPL: {avg_acpl:.1f} cp <<<")
        
    with open("elo_benchmark_results.json", "w") as f:
        json.dump(results, f, indent=2)
        
    print("\n=== Benchmark Complete! Results saved to elo_benchmark_results.json ===")
    return results

if __name__ == "__main__":
    run_elo_benchmark()
