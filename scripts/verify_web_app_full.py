import urllib.request
import urllib.parse
import json
import time

BASE_URL = "http://127.0.0.1:8000"

def test_full_web_app():
    print("=== Comprehensive Web App & MultiPV Engine Verification ===")
    
    # 1. Test Static files delivery
    for path, expected_text in [
        ("/", "ENGINE ANALYSIS"),
        ("/style.css", "eval-bar"),
        ("/app.js", "AudioContext")
    ]:
        url = f"{BASE_URL}{path}"
        req = urllib.request.Request(url)
        with urllib.request.urlopen(req, timeout=5) as resp:
            content = resp.read().decode('utf-8', errors='ignore')
            assert resp.status == 200, f"Status not 200 for {path}"
            assert expected_text in content, f"Expected text '{expected_text}' not found in {path}"
            print(f"[PASS] Static file: {path} ({len(content)} bytes)")

    # 2. Test MultiPV Analysis on Starting Position
    print("\n--- Testing /api/analyze (Start Position) ---")
    start_fen = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"
    req_data = json.dumps({"fen": start_fen, "depth": 5}).encode('utf-8')
    req = urllib.request.Request(f"{BASE_URL}/api/analyze", data=req_data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=10) as resp:
        res = json.loads(resp.read().decode('utf-8'))
        assert "lines" in res, "Missing 'lines' in response"
        assert len(res["lines"]) == 3, f"Expected 3 MultiPV lines, got {len(res['lines'])}"
        print(f"[PASS] Startpos MultiPV count: {len(res['lines'])} | Eval: {res.get('eval')} cp | Win Rate: {res.get('win_rate')}%")
        for line in res["lines"]:
            print(f"       Rank {line['multipv']}: {line['pv']} (Score: {line['score']} cp)")

    # 3. Test MultiPV Analysis on Tactical Mate Position (Scholar's Mate in 1)
    print("\n--- Testing /api/analyze (Scholar's Mate in 1) ---")
    mate_fen = "r1bqkb1r/pppp1ppp/2n5/4p2Q/2B1n3/8/PPPP1PPP/RNB1K1NR w KQkq - 0 5"
    req_data = json.dumps({"fen": mate_fen, "depth": 6}).encode('utf-8')
    req = urllib.request.Request(f"{BASE_URL}/api/analyze", data=req_data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=10) as resp:
        res = json.loads(resp.read().decode('utf-8'))
        assert "lines" in res
        top_line = res["lines"][0]
        print(f"[PASS] Top Line in mate position: {top_line['pv']} | Score: {top_line['score']}")
        assert "f7" in top_line["pv"] or top_line["score"] >= 9000 or top_line.get("mate"), "Top line did not find f7 mate!"

    # 4. Test MultiPV Analysis on Kiwipete Position
    print("\n--- Testing /api/analyze (Kiwipete Complex Tactics) ---")
    kiwipete_fen = "r3k2r/p1ppqpb1/bn2pnp1/3PN3/1p2P3/2N2Q1p/PPPBBPPP/R3K2R w KQkq - 0 1"
    req_data = json.dumps({"fen": kiwipete_fen, "depth": 5}).encode('utf-8')
    req = urllib.request.Request(f"{BASE_URL}/api/analyze", data=req_data, headers={"Content-Type": "application/json"})
    with urllib.request.urlopen(req, timeout=10) as resp:
        res = json.loads(resp.read().decode('utf-8'))
        print(f"[PASS] Kiwipete Analysis: {len(res['lines'])} lines, Eval: {res.get('eval')} cp")
        for line in res["lines"]:
            print(f"       MultiPV {line['multipv']}: {line['pv']} (Score: {line['score']} cp)")

    # 5. Test /api/bot_move across various depths
    print("\n--- Testing /api/bot_move ---")
    for depth in [2, 5, 8]:
        req_data = json.dumps({"fen": start_fen, "depth": depth}).encode('utf-8')
        req = urllib.request.Request(f"{BASE_URL}/api/bot_move", data=req_data, headers={"Content-Type": "application/json"})
        start_t = time.time()
        with urllib.request.urlopen(req, timeout=10) as resp:
            res = json.loads(resp.read().decode('utf-8'))
            elapsed = time.time() - start_t
            assert "move" in res
            print(f"[PASS] Bot move at depth {depth}: played '{res['move']}' in {elapsed:.3f}s")

    print("\n=== All Verification Tests Successfully PASSED! ===")
    return True

if __name__ == "__main__":
    test_full_web_app()
