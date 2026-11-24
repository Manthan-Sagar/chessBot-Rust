import json
import chess
import chess.engine

STOCKFISH_PATH = r"C:\Users\manth\AppData\Local\Microsoft\WinGet\Packages\Stockfish.Stockfish_Microsoft.Winget.Source_8wekyb3d8bbwe\stockfish\stockfish-windows-x86-64-universal.exe"

def analyze():
    with open("game_analysis.json") as f:
        game = json.load(f)
        
    engine = chess.engine.SimpleEngine.popen_uci(STOCKFISH_PATH)
    
    white_losses = []
    black_losses = []
    
    blunders = []
    mistakes = []
    inaccuracies = []
    
    for i, m in enumerate(game["moves"]):
        ply = m["ply"]
        side = m["side"]
        loss = m["cp_loss"]
        san = m["san"]
        fen = m["fen"]
        
        # We cap loss at 1000 for ACPL calculation to avoid 1 blunder skewing average to infinity
        capped_loss = min(loss, 1000)
        if side == "White":
            white_losses.append(capped_loss)
        else:
            black_losses.append(capped_loss)
            
        if side == "White":
            if loss >= 300:
                blunders.append((ply, san, loss, fen, i))
            elif loss >= 90:
                mistakes.append((ply, san, loss, fen, i))
            elif loss >= 30:
                inaccuracies.append((ply, san, loss, fen, i))

    print("=== GAME OVERVIEW ===")
    print(f"White: {game['white']} vs Black: {game['black']}")
    print(f"Result: {game['result']}")
    print(f"Total plies: {game['total_plies']}")
    print()
    
    # ACPL
    # Raw ACPL vs Capped ACPL
    white_raw_acpl = sum(m['cp_loss'] for m in game['moves'] if m['side'] == 'White') / len(white_losses)
    white_capped_acpl = sum(white_losses) / len(white_losses)
    black_acpl = sum(black_losses) / len(black_losses)
    
    # Opening (plies 1-20)
    white_opening = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'White' and m['ply'] <= 20]
    # Middlegame (plies 21-50)
    white_mid = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'White' and 20 < m['ply'] <= 50]
    # Endgame (plies > 50)
    white_end = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'White' and m['ply'] > 50]
    
    print(f"White Capped ACPL (cap 1000cp): {white_capped_acpl:.1f}")
    print(f"White Raw ACPL: {white_raw_acpl:.1f}")
    print(f"Black ACPL: {black_acpl:.1f}")
    if white_opening:
        print(f"White Opening ACPL (plies 1-20): {sum(white_opening)/len(white_opening):.1f}")
    if white_mid:
        print(f"White Middlegame ACPL (plies 21-50): {sum(white_mid)/len(white_mid):.1f}")
    if white_end:
        print(f"White Late Middlegame/End ACPL (plies 51+): {sum(white_end)/len(white_end):.1f}")
    print()
    
    print("=== WHITE BLUNDERS (Loss >= 300 cp) ===")
    for ply, san, loss, fen, idx in blunders:
        # Get board state before this move
        prev_fen = game["moves"][idx-1]["fen"] if idx > 0 else chess.STARTING_FEN
        board = chess.Board(prev_fen)
        info = engine.analyse(board, chess.engine.Limit(depth=14))
        best_move = board.san(info["pv"][0]) if "pv" in info and info["pv"] else "none"
        print(f"Ply {ply}: White played {san} (CP Loss: {loss})")
        print(f"  Position before move: {prev_fen}")
        print(f"  Stockfish best move: {best_move} (Eval: {info['score'].white().score(mate_score=10000)} cp)")
        print(f"  Position after move: {fen}")
        print()

    print("=== WHITE MISTAKES (90 <= Loss < 300 cp) ===")
    for ply, san, loss, fen, idx in mistakes:
        prev_fen = game["moves"][idx-1]["fen"] if idx > 0 else chess.STARTING_FEN
        board = chess.Board(prev_fen)
        info = engine.analyse(board, chess.engine.Limit(depth=14))
        best_move = board.san(info["pv"][0]) if "pv" in info and info["pv"] else "none"
        print(f"Ply {ply}: White played {san} (CP Loss: {loss})")
        print(f"  Position before move: {prev_fen}")
        print(f"  Stockfish best move: {best_move} (Eval: {info['score'].white().score(mate_score=10000)} cp)")
        print()

    print("=== WHITE INACCURACIES (30 <= Loss < 90 cp) ===")
    for ply, san, loss, fen, idx in inaccuracies:
        prev_fen = game["moves"][idx-1]["fen"] if idx > 0 else chess.STARTING_FEN
        board = chess.Board(prev_fen)
        info = engine.analyse(board, chess.engine.Limit(depth=14))
        best_move = board.san(info["pv"][0]) if "pv" in info and info["pv"] else "none"
        print(f"Ply {ply}: White played {san} (CP Loss: {loss}) | Best: {best_move}")
    
    engine.quit()

if __name__ == "__main__":
    analyze()
