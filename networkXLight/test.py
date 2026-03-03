import time
import networkx as nx

def ns() -> int:
    return time.perf_counter_ns()

def avg_ns(total_ns: int, iters: int) -> float:
    return total_ns / iters

def main():
    # ---- Build graph (closest match: directed edges like your add_edge calls) ----
    G = nx.DiGraph()
    G.graph["name"] = "nxGraph"

    # Nodes with attributes
    G.add_node("A", color="amber")
    G.add_node("B", color=True)

    # Edges with attributes
    G.add_edge("A", "B", weight=5, relation="friend")
    G.add_edge("A", "C", weight=2, relation="colleague")
    G.add_edge("B", "D", weight=7, cost=1.5)
    G.add_edge("B", "E", weight=1, enabled=False)
    G.add_edge("C", "F", weight=9, relation="family")
    G.add_edge("E", "F", weight=3, relation="connected")

    source = "A"
    dst = "D"

    print("Analysing each algorithm in NX structure with 100,000 iterations")

    # ---- DFS edges ----
    iters = 100_000
    t0 = ns()
    for _ in range(iters):
        list(nx.dfs_edges(G, source=source))
    t1 = ns()
    print(f"DFS -> Avg: {avg_ns(t1 - t0, iters):,.1f} ns")

    # ---- BFS edges ----
    t0 = ns()
    for _ in range(iters):
        list(nx.bfs_edges(G, source=source))
    t1 = ns()
    print(f"BFS -> Avg: {avg_ns(t1 - t0, iters):,.1f} ns")

    # ---- Prim's MST (NetworkX Prim is for UNDIRECTED graphs) ----
    # Convert to undirected for MST (this mirrors typical NX behavior: MST is defined on undirected graphs)
    UG = G.to_undirected()

    t0 = ns()
    for _ in range(iters):
        # Returns an iterator, consume it so work actually happens
        list(nx.minimum_spanning_edges(UG, algorithm="prim", weight="weight", data=True))
    t1 = ns()
    print(f"Prim MST (undirected) -> Avg: {avg_ns(t1 - t0, iters):,.1f} ns")

    # ---- Dijkstra path (directed, weighted) ----
    try:
        cost = nx.dijkstra_path_length(G, source, dst, weight="weight")
        path = nx.dijkstra_path(G, source, dst, weight="weight")
        print(f"Shortest cost: {cost}")
        print(f"Path: {path}")
    except nx.NetworkXNoPath:
        print("No path found.")

    t0 = ns()
    for _ in range(iters):
        try:
            nx.dijkstra_path(G, source, dst, weight="weight")
        except nx.NetworkXNoPath:
            pass
    t1 = ns()
    print(f"Dijkstra path -> Avg: {avg_ns(t1 - t0, iters):,.1f} ns")

    # ---- Floyd–Warshall (all-pairs shortest paths) ----
    # This is O(n^3), so use fewer iterations like you did.
    fw_iters = 10_000
    t0 = ns()
    for _ in range(fw_iters):
        # Returns dict-of-dict distances
        _ = nx.floyd_warshall(G, weight="weight")
    t1 = ns()
    print(f"Floyd–Warshall -> Avg: {avg_ns(t1 - t0, fw_iters):,.1f} ns (with 10x fewer iters)")

    # ---- Print all (rough equivalent) ----
    print("\nGraph metadata:", G.graph)
    print("Nodes with attrs:")
    for n, attrs in G.nodes(data=True):
        print(f"  {n}: {attrs}")

    print("Edges with attrs:")
    for u, v, attrs in G.edges(data=True):
        print(f"  {u} -> {v}: {attrs}")

    print(f"\nNeighbors/outgoing from {source}: {list(G.successors(source))}")

if __name__ == "__main__":
    main()
