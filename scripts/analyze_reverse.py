import json

def analyze_reverse():
    with open("reverse_game_analysis.json") as f:
        game = json.load(f)
        
    black_losses = []
    white_losses = []
    
    blunders = []
    mistakes = []
    inaccuracies = []
    
    for i, m in enumerate(game["moves"]):
        ply = m["ply"]
        side = m["side"]
        loss = m["cp_loss"]
        san = m["san"]
        fen = m["fen"]
        
        capped_loss = min(loss, 1000)
        if side == "Black":
            black_losses.append(capped_loss)
            if loss >= 300:
                blunders.append((ply, san, loss, fen))
            elif loss >= 90:
                mistakes.append((ply, san, loss, fen))
            elif loss >= 30:
                inaccuracies.append((ply, san, loss, fen))
        else:
            white_losses.append(capped_loss)
            
    black_capped_acpl = sum(black_losses) / len(black_losses)
    black_raw_acpl = sum(m['cp_loss'] for m in game['moves'] if m['side'] == 'Black') / len(black_losses)
    white_acpl = sum(white_losses) / len(white_losses)
    
    opening = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'Black' and m['ply'] <= 20]
    mid = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'Black' and 20 < m['ply'] <= 50]
    end = [min(m['cp_loss'], 1000) for m in game['moves'] if m['side'] == 'Black' and m['ply'] > 50]
    
    print("=== GAME 2 OVERVIEW ===")
    print(f"White: {game['white']} vs Black: {game['black']}")
    print(f"Result: {game['result']}")
    print(f"Total plies: {game['total_plies']}")
    print(f"Rust (Black) Capped ACPL: {black_capped_acpl:.1f}")
    print(f"Rust (Black) Raw ACPL: {black_raw_acpl:.1f}")
    print(f"Stockfish (White) ACPL: {white_acpl:.1f}")
    if opening:
        print(f"Rust Opening ACPL (plies 1-20): {sum(opening)/len(opening):.1f}")
    if mid:
        print(f"Rust Middlegame ACPL (plies 21-50): {sum(mid)/len(mid):.1f}")
    if end:
        print(f"Rust Endgame ACPL (plies 51+): {sum(end)/len(end):.1f}")
        
    print("\nBlack Blunders (>=300):", blunders)
    print("Black Mistakes (90-300):", mistakes)
    print("Black Inaccuracies (30-90):", inaccuracies)

if __name__ == "__main__":
    analyze_reverse()
