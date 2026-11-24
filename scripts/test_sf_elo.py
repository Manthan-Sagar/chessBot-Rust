import chess
import chess.engine
import os

STOCKFISH_PATH = r"C:\Users\manth\AppData\Local\Microsoft\WinGet\Packages\Stockfish.Stockfish_Microsoft.Winget.Source_8wekyb3d8bbwe\stockfish\stockfish-windows-x86-64-universal.exe"

def test_stockfish_elo():
    engine = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
    print("Stockfish options:")
    print("UCI_LimitStrength in options?", "UCI_LimitStrength" in engine.options)
    print("UCI_Elo in options?", "UCI_Elo" in engine.options)
    if "UCI_Elo" in engine.options:
        elo_opt = engine.options["UCI_Elo"]
        print(f"UCI_Elo min={elo_opt.min}, max={elo_opt.max}, default={elo_opt.default}")
        
    engine.configure({"UCI_LimitStrength": True, "UCI_Elo": 2400})
    print("Configured to UCI_Elo 2400 successfully!")
    
    board = chess.Board()
    limit = chess.engine.Limit(time=0.1)
    res = engine.play(board, limit)
    print("Stockfish move at 2400 Elo:", res.move)
    engine.quit()

if __name__ == "__main__":
    test_stockfish_elo()
