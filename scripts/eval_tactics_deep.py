import chess
import chess.engine

STOCKFISH_PATH = r"C:\Users\manth\AppData\Local\Microsoft\WinGet\Packages\Stockfish.Stockfish_Microsoft.Winget.Source_8wekyb3d8bbwe\stockfish\stockfish-windows-x86-64-universal.exe"
RUST_ENGINE_PATH = r"C:\Users\manth\.t3\projects\chess-in-rust\target\release\chess-engine.exe"

sf = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
rust = chess.engine.SimpleEngine.popen_uci(RUST_ENGINE_PATH)

print("=== Analyzing Test 3 ===")
fen3 = "r1b1kb1r/pppp1ppp/8/4q3/8/8/PPPP1PPP/RNBQKB1R w KQkq - 0 1"
board3 = chess.Board(fen3)
sf_info3 = sf.analyse(board3, chess.engine.Limit(depth=14), multipv=3)
for i, item in enumerate(sf_info3):
    move = board3.san(item["pv"][0])
    score = item["score"].white().score(mate_score=10000)
    print(f"Stockfish rank {i+1}: {move} ({score} cp)")

print("\n=== Analyzing Test 6 (Greek Gift) ===")
fen6 = "r1b2rk1/pp3ppp/2n5/2qp4/8/1P1B1N2/P1P2PPP/R2Q1RK1 w - - 0 14"
board6 = chess.Board(fen6)
sf_info6 = sf.analyse(board6, chess.engine.Limit(depth=14), multipv=3)
for i, item in enumerate(sf_info6):
    move = board6.san(item["pv"][0])
    score = item["score"].white().score(mate_score=10000)
    print(f"Stockfish rank {i+1}: {move} ({score} cp)")

print("\nTesting Rust Engine at deeper depths on Test 6:")
for d in [6, 7, 8, 9, 10]:
    res = rust.play(board6, chess.engine.Limit(depth=d))
    played_san = board6.san(res.move)
    print(f"Rust Engine Depth {d}: played {played_san}")

sf.quit()
rust.quit()
