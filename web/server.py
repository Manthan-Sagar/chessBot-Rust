#!/usr/bin/env python3
"""
High-Performance UCI Web Server Bridge for chess-in-rust.
Zero-dependency Python standard library implementation.
Serves static frontend and provides MultiPV analysis & bot move APIs.
"""

import http.server
import json
import os
import subprocess
import threading
import urllib.parse

PORT = 8000
WEB_DIR = os.path.dirname(os.path.abspath(__file__))
PROJECT_ROOT = os.path.dirname(WEB_DIR)

# Locate compiled release binary
CANDIDATE_PATHS = [
    os.path.join(PROJECT_ROOT, "target", "release", "chess-engine.exe"),
    os.path.join(PROJECT_ROOT, "target", "release", "chess-engine"),
    r"C:\Users\manth\Documents\chess-in-rust\target\release\chess-engine.exe",
    r"C:\Users\manth\.t3\projects\chess-in-rust\target\release\chess-engine.exe",
]

ENGINE_PATH = None
for p in CANDIDATE_PATHS:
    if os.path.isfile(p):
        ENGINE_PATH = p
        break

if not ENGINE_PATH:
    ENGINE_PATH = os.path.join(PROJECT_ROOT, "target", "release", "chess-engine.exe")

START_FEN = "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1"


class EngineBridge:
    def __init__(self, binary_path):
        self.binary_path = binary_path
        self.lock = threading.Lock()
        self.proc = None
        self.current_multipv = None
        self._start_engine()

    def _start_engine(self):
        if self.proc and self.proc.poll() is None:
            try:
                self.proc.kill()
            except Exception:
                pass

        self.proc = subprocess.Popen(
            [self.binary_path],
            stdin=subprocess.PIPE,
            stdout=subprocess.PIPE,
            stderr=subprocess.PIPE,
            text=True,
            bufsize=1,
        )
        self._send("uci")
        while True:
            line = self.proc.stdout.readline()
            if not line:
                break
            if line.strip() == "uciok":
                break
        self._set_multipv(3)
        self._wait_ready()

    def _send(self, cmd):
        try:
            self.proc.stdin.write(cmd + "\n")
            self.proc.stdin.flush()
        except Exception:
            self._start_engine()
            self.proc.stdin.write(cmd + "\n")
            self.proc.stdin.flush()

    def _wait_ready(self):
        self._send("isready")
        while True:
            line = self.proc.stdout.readline()
            if not line:
                break
            if line.strip() == "readyok":
                break

    def _set_multipv(self, value: int):
        if self.current_multipv != value:
            self._send(f"setoption name MultiPV value {value}")
            self.current_multipv = value

    def new_game(self):
        with self.lock:
            if self.proc.poll() is not None:
                self._start_engine()
            self._send("ucinewgame")
            self._wait_ready()
            return {"ok": True}

    def analyze(self, fen: str, depth: int = 18, movetime: int = 2000):
        with self.lock:
            if self.proc.poll() is not None:
                self._start_engine()

            depth = max(1, min(int(depth), 30))
            movetime = max(50, min(int(movetime), 30000))

            self._set_multipv(3)
            self._send(f"position fen {fen}")
            self._wait_ready()
            self._send(f"go depth {depth} movetime {movetime}")

            # Parse side to move from FEN for White-perspective normalization
            side = "w"
            fen_parts = fen.split()
            if len(fen_parts) >= 2:
                side = fen_parts[1]

            multipv_lines = {}
            best_move = None
            max_depth = 0

            while True:
                line = self.proc.stdout.readline()
                if not line:
                    break
                line = line.strip()
                if line.startswith("bestmove"):
                    parts = line.split()
                    if len(parts) >= 2:
                        best_move = parts[1]
                    break

                if line.startswith("info") and "multipv" in line and "pv" in line:
                    tokens = line.split()
                    try:
                        d = int(tokens[tokens.index("depth") + 1])
                        mp = int(tokens[tokens.index("multipv") + 1])
                        score_idx = tokens.index("score")
                        score_type = tokens[score_idx + 1]
                        raw_score = int(tokens[score_idx + 2])
                        pv_idx = tokens.index("pv") + 1
                        pv_str = " ".join(tokens[pv_idx:])

                        white_val = raw_score if side == "w" else -raw_score

                        if score_type == "mate":
                            if white_val == 0:
                                mate_in = -1 if side == "w" else 1
                            else:
                                mate_in = white_val
                            sign = 1 if mate_in > 0 else -1
                            cp_equiv = sign * max(10000, 49000 - (abs(mate_in) - 1) * 100)
                            eval_text = f"+M{mate_in}" if mate_in > 0 else f"-M{abs(mate_in)}"
                            entry = {
                                "depth": d,
                                "multipv": mp,
                                "score_type": "mate",
                                "score": cp_equiv,
                                "white_score": cp_equiv,
                                "mate": mate_in,
                                "eval": eval_text,
                                "pv": pv_str,
                            }
                        else:
                            eval_text = f"{'+' if white_val >= 0 else ''}{white_val / 100.0:.1f}"
                            entry = {
                                "depth": d,
                                "multipv": mp,
                                "score_type": "cp",
                                "score": white_val,
                                "white_score": white_val,
                                "mate": None,
                                "eval": eval_text,
                                "pv": pv_str,
                            }

                        multipv_lines[mp] = entry
                        if d > max_depth:
                            max_depth = d
                    except Exception:
                        pass

            lines = [multipv_lines[k] for k in sorted(multipv_lines.keys())]

            game_over = best_move in (None, "(none)", "0000")
            clean_best = None if game_over else best_move

            if lines:
                top = lines[0]
                top_score = top["score"]
                eval_str = top["eval"]
                top_score_type = top["score_type"]
                top_mate = top["mate"]
                if top_score_type == "mate":
                    win_prob = 1.0 if (top_mate or top_score) > 0 else 0.0
                else:
                    win_prob = 1.0 / (1.0 + 10.0 ** (-top_score / 400.0))
            else:
                top_score = 0
                eval_str = "0.0"
                top_score_type = "cp"
                top_mate = None
                win_prob = 0.5

            return {
                "eval": eval_str,
                "score": top_score,
                "white_score": top_score,
                "score_type": top_score_type,
                "mate": top_mate,
                "win_prob": round(win_prob, 4),
                "depth": max_depth,
                "lines": lines,
                "bestmove": clean_best,
                "game_over": game_over,
            }

    def bot_move(self, fen: str, depth: int = 6, movetime=None):
        with self.lock:
            if self.proc.poll() is not None:
                self._start_engine()

            depth = max(1, min(int(depth), 30))
            self._set_multipv(1)
            self._send(f"position fen {fen}")
            self._wait_ready()

            if movetime is not None:
                mt = max(50, min(int(movetime), 30000))
                self._send(f"go depth {depth} movetime {mt}")
            else:
                self._send(f"go depth {depth}")

            best_move = None
            while True:
                line = self.proc.stdout.readline()
                if not line:
                    break
                line = line.strip()
                if line.startswith("bestmove"):
                    parts = line.split()
                    if len(parts) >= 2:
                        best_move = parts[1]
                    break

            if best_move in (None, "(none)", "0000"):
                return {"move": None, "game_over": True}
            return {"move": best_move, "game_over": False}


# Global engine bridge singleton
BRIDGE = None


class ChessHTTPServer(http.server.ThreadingHTTPServer):
    daemon_threads = True
    allow_reuse_address = True


class ChessRequestHandler(http.server.BaseHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def _send_json(self, data, status=200):
        payload = json.dumps(data).encode("utf-8")
        self.send_response(status)
        self.send_header("Content-Type", "application/json")
        self.send_header("Content-Length", str(len(payload)))
        self.end_headers()
        self.wfile.write(payload)

    def do_GET(self):
        url_path = urllib.parse.urlparse(self.path).path
        if url_path == "/" or url_path == "/index.html":
            file_path = os.path.join(WEB_DIR, "index.html")
            content_type = "text/html; charset=utf-8"
        else:
            rel_path = url_path.lstrip("/")
            file_path = os.path.abspath(os.path.join(WEB_DIR, rel_path))
            if os.path.commonpath([WEB_DIR, file_path]) != WEB_DIR:
                self.send_response(403)
                self.end_headers()
                self.wfile.write(b"403 Forbidden")
                return
            if rel_path.endswith(".css"):
                content_type = "text/css; charset=utf-8"
            elif rel_path.endswith(".js"):
                content_type = "application/javascript; charset=utf-8"
            elif rel_path.endswith(".svg"):
                content_type = "image/svg+xml"
            elif rel_path.endswith(".json"):
                content_type = "application/json"
            else:
                content_type = "text/plain; charset=utf-8"

        if os.path.isfile(file_path):
            try:
                with open(file_path, "rb") as f:
                    content = f.read()
                self.send_response(200)
                self.send_header("Content-Type", content_type)
                self.send_header("Content-Length", str(len(content)))
                self.end_headers()
                self.wfile.write(content)
            except Exception as e:
                self.send_response(500)
                self.end_headers()
                self.wfile.write(f"500 Internal Server Error: {e}".encode("utf-8"))
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"404 Not Found")

    def do_POST(self):
        url_path = urllib.parse.urlparse(self.path).path
        content_length = int(self.headers.get("Content-Length", 0))
        body = self.rfile.read(content_length)

        try:
            req_data = json.loads(body.decode("utf-8")) if body else {}
        except Exception:
            req_data = {}

        try:
            if url_path == "/api/analyze":
                fen = req_data.get("fen", START_FEN)
                depth = int(req_data.get("depth", 18))
                movetime = int(req_data.get("movetime", 2000))
                res = BRIDGE.analyze(fen, depth=depth, movetime=movetime)
                self._send_json(res)

            elif url_path == "/api/bot_move":
                fen = req_data.get("fen", START_FEN)
                depth = int(req_data.get("depth", 6))
                movetime = req_data.get("movetime")
                if movetime is not None:
                    movetime = int(movetime)
                res = BRIDGE.bot_move(fen, depth=depth, movetime=movetime)
                self._send_json(res)

            elif url_path == "/api/new_game":
                res = BRIDGE.new_game()
                self._send_json(res)

            else:
                self.send_response(404)
                self.end_headers()
                self.wfile.write(b"404 API Endpoint Not Found")
        except Exception as e:
            self._send_json({"error": str(e)}, status=500)

    def log_message(self, format, *args):
        return


def run_server():
    global BRIDGE
    print(f"Connecting to engine at: {ENGINE_PATH}")
    BRIDGE = EngineBridge(ENGINE_PATH)
    server_address = ("", PORT)
    httpd = ChessHTTPServer(server_address, ChessRequestHandler)
    print(f"Server started on http://127.0.0.1:{PORT}")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping server...")
    finally:
        httpd.server_close()


if __name__ == "__main__":
    run_server()
