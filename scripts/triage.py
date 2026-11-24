import json

for p in ['game_analysis.json', 'reverse_game_analysis.json']:
    data = json.load(open(p))
    rust = [m for m in data['moves'] if 'Rust' in m.get('engine', '')]
    loss = sum(m.get('cp_loss', 0) for m in rust) / len(rust) if rust else 0
    blunders = [m for m in rust if m.get('cp_loss', 0) >= 300]
    print(f"{p}: Result={data.get('result')}, Plies={data.get('total_plies')}, ACPL={loss:.1f}, Blunders={len(blunders)}")
    for b in blunders:
        print(f"  Blunder ply {b['ply']}: {b['san']} ({b['cp_loss']} cp loss)")
