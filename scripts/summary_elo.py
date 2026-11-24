import json

with open(r'C:\Users\manth\Documents\chess-in-rust\elo_benchmark_results.json') as f:
    data = json.load(f)

for tier, info in data.items():
    print(f"=== Elo Tier {tier} ===")
    print(f"Score: {info['score']} / {info['max_score']} ({info['pct']}%) | Avg ACPL: {info['avg_acpl']:.1f}")
    print(f"  Game 1 (White): Result = {info['g1']['result']}, ACPL = {info['g1']['rust_acpl']}")
    print(f"  Game 2 (Black): Result = {info['g2']['result']}, ACPL = {info['g2']['rust_acpl']}")
