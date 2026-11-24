import os
import json
import chess
import chess.engine
from play_match import play_game, RUST_ENGINE_PATH, STOCKFISH_PATH

def run_reverse_match():
    print("Starting Reverse Match: Stockfish-19 (White) vs Rust-BBC-Engine (Black)...")
    game_result = play_game(
        white_path=STOCKFISH_PATH,
        black_path=RUST_ENGINE_PATH,
        white_name="Stockfish-19",
        black_name="Rust-BBC-Engine",
        time_per_move=0.3,
        max_moves=60
    )
    with open("reverse_game_analysis.json", "w") as f:
        json.dump(game_result, f, indent=2)
    print("Reverse match finished!")

if __name__ == "__main__":
    run_reverse_match()
