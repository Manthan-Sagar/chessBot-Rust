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
import sys
import threading
import time
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

class EngineBridge:
    def __init__(self, binary_path):
        self.binary_path = binary_path
        self.lock = threading.Lock()
        self.proc = None
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
            bufsize=1
        )
        self._send("uci")
        while True:
            line = self.proc.stdout.readline()
            if not line:
                break
            line = line.strip()
            if line == "uciok":
                break
        self._send("setoption name MultiPV value 3")
        self._send("isready")
        while True:
            line = self.proc.stdout.readline()
            if not line:
                break
            line = line.strip()
            if line == "readyok":
                break

    def _send(self, cmd):
        try:
            self.proc.stdin.write(cmd + "\n")
            self.proc.stdin.flush()
        except Exception:
            self._start_engine()
            self.proc.stdin.write(cmd + "\n")
            self.proc.stdin.flush()

    def analyze(self, fen: str, depth: int = 6):
        with self.lock:
            # Ensure engine is alive
            if self.proc.poll() is not None:
                self._start_engine()

            depth = max(1, min(depth, 16))
            self._send("ucinewgame")
            self._send(f"position fen {fen}")
            self._send("setoption name MultiPV value 3")
            self._send("isready")
            while True:
                line = self.proc.stdout.readline().strip()
                if line == "readyok":
                    break

            self._send(f"go depth {depth}")

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
                        score_type = tokens[tokens.index("score") + 1]
                        score_val = int(tokens[tokens.index("score") + 2])
                        pv_idx = tokens.index("pv") + 1
                        pv_str = " ".join(tokens[pv_idx:])

                        entry = {
                            "depth": d,
                            "multipv": mp,
                            "score_type": score_type, # "cp" or "mate"
                            "score": score_val,
                            "pv": pv_str,
                        }
                        multipv_lines[mp] = entry
                        if d > max_depth:
                            max_depth = d
                    except Exception:
                        pass

            # Parse side to move from FEN for eval display
            side = "w"
            fen_parts = fen.split()
            if len(fen_parts) >= 2:
                side = fen_parts[1]

            lines = [multipv_lines[k] for k in sorted(multipv_lines.keys())]
            
            top_eval = 0
            eval_str = "0.0"
            if lines:
                top = lines[0]
                raw_score = top["score"]
                # Convert to White perspective
                white_score = raw_score if side == "w" else -raw_score
                top_eval = white_score
                if top["score_type"] == "mate":
                    eval_str = f"M{white_score}"
                else:
                    eval_str = f"{'+' if white_score >= 0 else ''}{white_score / 100.0:.1f}"

            return {
                "eval": eval_str,
                "score": top_eval,
                "depth": max_depth,
                "lines": lines,
                "bestmove": best_move,
            }

    def bot_move(self, fen: str, depth: int = 4):
        with self.lock:
            if self.proc.poll() is not None:
                self._start_engine()

            depth = max(1, min(depth, 16))
            self._send(f"position fen {fen}")
            self._send("setoption name MultiPV value 1")
            self._send("isready")
            while True:
                line = self.proc.stdout.readline().strip()
                if line == "readyok":
                    break

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

            return {"move": best_move}

# Global engine bridge singleton
BRIDGE = None

class ChessRequestHandler(http.server.BaseHTTPRequestHandler):
    def end_headers(self):
        self.send_header("Access-Control-Allow-Origin", "*")
        self.send_header("Access-Control-Allow-Methods", "GET, POST, OPTIONS")
        self.send_header("Access-Control-Allow-Headers", "Content-Type")
        super().end_headers()

    def do_OPTIONS(self):
        self.send_response(200)
        self.end_headers()

    def do_GET(self):
        url_path = urllib.parse.urlparse(self.path).path
        if url_path == "/" or url_path == "/index.html":
            file_path = os.path.join(WEB_DIR, "index.html")
            content_type = "text/html; charset=utf-8"
        else:
            rel_path = url_path.lstrip("/")
            file_path = os.path.join(WEB_DIR, rel_path)
            if rel_path.endswith(".css"):
                content_type = "text/css"
            elif rel_path.endswith(".js"):
                content_type = "application/javascript"
            elif rel_path.endswith(".svg"):
                content_type = "image/svg+xml"
            elif rel_path.endswith(".json"):
                content_type = "application/json"
            else:
                content_type = "text/plain"

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
            req_data = json.loads(body.decode("utf-8"))
        except Exception:
            req_data = {}

        if url_path == "/api/analyze":
            fen = req_data.get("fen", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            depth = req_data.get("depth", 6)
            res = BRIDGE.analyze(fen, depth)
            
            payload = json.dumps(res).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)

        elif url_path == "/api/bot_move":
            fen = req_data.get("fen", "rnbqkbnr/pppppppp/8/8/8/8/PPPPPPPP/RNBQKBNR w KQkq - 0 1")
            depth = req_data.get("depth", 4)
            res = BRIDGE.bot_move(fen, depth)

            payload = json.dumps(res).encode("utf-8")
            self.send_response(200)
            self.send_header("Content-Type", "application/json")
            self.send_header("Content-Length", str(len(payload)))
            self.end_headers()
            self.wfile.write(payload)
        else:
            self.send_response(404)
            self.end_headers()
            self.wfile.write(b"404 API Endpoint Not Found")

    def log_message(self, format, *args):
        # Quiet standard server logging to keep output clean
        return

def run_server():
    global BRIDGE
    print(f"Connecting to engine at: {ENGINE_PATH}")
    BRIDGE = EngineBridge(ENGINE_PATH)
    server_address = ("", PORT)
    httpd = http.server.HTTPServer(server_address, ChessRequestHandler)
    print(f"Server started on http://127.0.0.1:{PORT}")
    try:
        httpd.serve_forever()
    except KeyboardInterrupt:
        print("\nStopping server...")
    finally:
        httpd.server_close()

if __name__ == "__main__":
    run_server()
