import urllib.request
import urllib.parse
import json
import time
import sys

BASE_URL = "http://127.0.0.1:8000"

def test_endpoints():
    print(f"=== Testing Web API Server at {BASE_URL} ===")
    
    # 1. Test Static files
    try:
        req = urllib.request.Request(f"{BASE_URL}/")
        with urllib.request.urlopen(req, timeout=5) as resp:
            content = resp.read().decode('utf-8')
            assert resp.status == 200
            assert "<!DOCTYPE html>" in content or "<html" in content
            print("[PASS] GET / serves index.html")
    except Exception as e:
        print(f"[FAIL] GET / failed: {e}")
        return False
        
    # 2. Test /api/analyze
    test_fen = "rnbqkbnr/pppppppp/8/8/4P3/8/PPPP1PPP/RNBQKBNR b KQkq e3 0 1"
    try:
        data = json.dumps({"fen": test_fen, "depth": 6}).encode('utf-8')
        req = urllib.request.Request(
            f"{BASE_URL}/api/analyze",
            data=data,
            headers={"Content-Type": "application/json"}
        )
        with urllib.request.urlopen(req, timeout=10) as resp:
            res = json.loads(resp.read().decode('utf-8'))
            assert resp.status == 200
            assert "lines" in res
            assert len(res["lines"]) > 0
            print(f"[PASS] POST /api/analyze returned {len(res['lines'])} lines, eval: {res.get('eval')}")
            for line in res["lines"]:
                print(f"       MultiPV {line.get('multipv')}: {line.get('pv')} (Score: {line.get('score')} cp)")
    except Exception as e:
        print(f"[FAIL] POST /api/analyze failed: {e}")
        return False

    # 3. Test /api/bot_move
    try:
        data = json.dumps({"fen": test_fen, "depth": 4}).encode('utf-8')
        req = urllib.request.Request(
            f"{BASE_URL}/api/bot_move",
            data=data,
            headers={"Content-Type": "application/json"}
        )
        with urllib.request.urlopen(req, timeout=10) as resp:
            res = json.loads(resp.read().decode('utf-8'))
            assert resp.status == 200
            assert "move" in res
            print(f"[PASS] POST /api/bot_move returned move: {res['move']}")
    except Exception as e:
        print(f"[FAIL] POST /api/bot_move failed: {e}")
        return False

    print("=== All API Endpoint Tests PASSED! ===")
    return True

if __name__ == "__main__":
    test_endpoints()
