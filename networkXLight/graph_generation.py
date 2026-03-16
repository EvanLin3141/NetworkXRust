import networkx as nx
import matplotlib.pyplot as plt
import random
import shutil
from pathlib import Path

# -------------------------------
# 0. Paths
# -------------------------------
NX_DIR = Path(__file__).resolve().parent
ROOT = NX_DIR.parent
RUST_GRAPHS_DIR = ROOT / "rust" / "graphs"

RUST_GRAPHS_DIR.mkdir(parents=True, exist_ok=True)

# -------------------------------
# 1. Settings
# -------------------------------
n = 1000

density_levels = {
    "sparse": 0.01,
    "medium": 0.25,
    "dense": 0.7
}

# -------------------------------
# 2. Generate graphs
# -------------------------------
for graph_type, density in density_levels.items():

    print("\nGenerating", graph_type, "graph")

    min_edges = n - 1
    max_edges = n * (n - 1) // 2

    m = int(min_edges + density * (max_edges - min_edges))

    G = nx.gnm_random_graph(n, m, seed=42)

    # ensure connectivity
    components = list(nx.connected_components(G))
    for i in range(len(components) - 1):
        u = next(iter(components[i]))
        v = next(iter(components[i + 1]))
        G.add_edge(u, v)

    # assign random weights
    for u, v in G.edges():
        G[u][v]['weight'] = random.randint(1, 10)

    print("Connected:", nx.is_connected(G))
    print("Nodes:", G.number_of_nodes())
    print("Edges:", G.number_of_edges())

    # -------------------------------
    # Save in networkXLight
    # -------------------------------
    nx_filename = NX_DIR / f"{graph_type}_graph.txt"
    nx.write_weighted_edgelist(G, nx_filename)

    print("Saved to", nx_filename)

    # -------------------------------
    # Copy to rust/graphs
    # -------------------------------
    rust_filename = RUST_GRAPHS_DIR / f"{graph_type}_graph.txt"
    shutil.copy2(nx_filename, rust_filename)

    print("Copied to", rust_filename)

    # -------------------------------
    # 3. Draw graph
    # -------------------------------
    plt.figure(figsize=(8, 8))
    pos = nx.spring_layout(G, seed=42)

    nx.draw(
        G,
        pos,
        node_size=20,
        with_labels=False
    )

    plt.title(f"{graph_type.capitalize()} Graph")
    plt.show()