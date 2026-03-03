from Graph.Graph import *
from traversal.dfs import *
from traversal.bfs import *
from shortestPath.dijsktra import *
from floyd import *
from mst import *
import time

def build_graph():
    G = Graph()
    G.graph["name"] = "nxGraph"

    G.add_node("A", color="amber")
    G.add_node("B", color=True)

    G.add_edge("A", "B", weight=5, relation="friend")
    G.add_edge("A", "C", weight=2, relation="colleague")
    G.add_edge("B", "D", weight=7, cost=1.5)
    G.add_edge("B", "E", weight=1, enabled=False)
    G.add_edge("C", "F", weight=9, relation="family")
    G.add_edge("E", "F", weight=3, relation="connected")

    return G

def bench(label, iterations, func):
    start = time.perf_counter()
    for _ in range(iterations):
        func()
    elapsed = time.perf_counter() - start
    print(f"{label} -> Avg: {elapsed/iterations:.9f} s")

def main():
    G = build_graph()

    print("Analysing each algorithm in NX structure with 100,000 iterations")

    bench("DFS", 100_000, lambda: list(dfs_edges(G, source="A")))
    bench("BFS", 100_000, lambda: list(bfs_edges(G, source="A")))

    # Prim MST (fixed)
    bench("Prim MST", 100_000, lambda: list(prim_mst_edges(G, weight="weight")))

    source = "A"
    dst = "D"

    path = dijkstra_path(G, source, dst, weight="weight")
    print("Path:", path)

    bench("Dijkstra", 100_000, lambda: dijkstra_path(G, source, dst, weight="weight"))

    iterations_fw = 10_000
    start = time.perf_counter()
    for _ in range(iterations_fw):
       _, _ = floyd_warshall(G, weight="weight")
    elapsed = time.perf_counter() - start
    print(f"Floyd-Warshall -> Avg: {elapsed/iterations_fw:.9f} s (10x fewer iterations)")

if __name__ == "__main__":
    main()