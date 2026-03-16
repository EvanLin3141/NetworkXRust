import subprocess
from pathlib import Path

ROOT = Path(__file__).parent
PY_DIR = ROOT / "networkXLight"
RUST_DIR = ROOT / "rust"

print("Step 1: Generating graphs with NetworkX...")
subprocess.run(["python", "graph_generation.py"], cwd=PY_DIR, check=True)

print("Step 2: Running Python graph analysis (DFS/BFS)...")
subprocess.run(["python", "graph_analysis.py"], cwd=PY_DIR, check=True)

print("Step 3: Running Rust algorithms...")
subprocess.run(["cargo", "run", "--release"], cwd=RUST_DIR, check=True)

print("Pipeline complete.")