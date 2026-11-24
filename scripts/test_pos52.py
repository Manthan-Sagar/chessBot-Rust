import subprocess
import time

RUST_ENGINE_PATH = r"C:\Users\manth\Projects\chess-in-rust\target\release\chess-engine.exe"
import os
RUST_ENGINE_PATH = os.path.abspath(r"target\release\chess-engine.exe")

def test_engine_pos():
    p = subprocess.Popen([RUST_ENGINE_PATH], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    
    # Position before ply 53 (after 52... Qg4)
    # FEN: N1b2r1k/b1P2ppB/3n4/p7/6q1/2P2pP1/P1Q2P1P/R4RK1 w - - 1 27
    fen = "N1b2r1k/b1P2ppB/3n4/p7/6q1/2P2pP1/P1Q2P1P/R4RK1 w - - 1 27"
    
    cmds = [
        "uci",
        "isready",
        f"position fen {fen}",
        "go movetime 300",
    ]
    p.stdin.write("\n".join(cmds) + "\n")
    p.stdin.flush()
    
    time.sleep(1.0)
    p.stdin.write("quit\n")
    p.stdin.flush()
    
    out, _ = p.communicate()
    print("Output for movetime 300:")
    print(out)

    p2 = subprocess.Popen([RUST_ENGINE_PATH], stdin=subprocess.PIPE, stdout=subprocess.PIPE, stderr=subprocess.PIPE, text=True)
    cmds2 = [
        "uci",
        "isready",
        f"position fen {fen}",
        "go depth 8",
    ]
    p2.stdin.write("\n".join(cmds2) + "\n")
    p2.stdin.flush()
    time.sleep(1.5)
    p2.stdin.write("quit\n")
    p2.stdin.flush()
    out2, _ = p2.communicate()
    print("Output for depth 8:")
    print(out2)

test_engine_pos()
