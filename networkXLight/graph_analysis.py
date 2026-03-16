import networkx as nx
import time

graph_files = [
    "sparse_graph.txt",
    "medium_graph.txt",
    "dense_graph.txt"
]

for file in graph_files:
    print("\n--- Testing", file, "---")

    # Load graph
    G_loaded = nx.read_weighted_edgelist(file, nodetype=int)

    print("Connected:", nx.is_connected(G_loaded))
    print("Nodes:", G_loaded.number_of_nodes())
    print("Edges:", G_loaded.number_of_edges())

    source = 0
    target = 500

    # -------------------------------
    # BFS Timing Analysis
    # -------------------------------
    start = time.perf_counter()
    bfs_edges = list(nx.bfs_edges(G_loaded, source=source))
    end = time.perf_counter()

    print("BFS time:", end - start, "seconds")

    # -------------------------------
    # DFS Timing Analysis
    # -------------------------------
    start = time.perf_counter()
    dfs_edges = list(nx.dfs_edges(G_loaded, source=source))
    end = time.perf_counter()

    print("DFS time:", end - start, "seconds")

    # -------------------------------
    # Prim's MST Timing Analysis
    # -------------------------------
    start = time.perf_counter()
    mst = nx.minimum_spanning_tree(G_loaded, algorithm="prim", weight="weight")
    end = time.perf_counter()

    print("Prim's time:", end - start, "seconds")
    print("MST edges:", mst.number_of_edges())


    # -------------------------------
    # Dijkstra Timing Analysis
    # -------------------------------
    start = time.perf_counter()
    dijkstra_path = nx.dijkstra_path(G_loaded, source=source, target=target, weight="weight")
    dijkstra_cost = nx.dijkstra_path_length(G_loaded, source=source, target=target, weight="weight")
    end = time.perf_counter()

    print("Dijkstra time:", end - start, "seconds")
    print("Dijkstra cost:", dijkstra_cost)

    # -------------------------------
    # Floyd-Warshall Timing Analysis
    # -------------------------------
    start = time.perf_counter()
    fw = dict(nx.floyd_warshall(G_loaded, weight="weight"))
    end = time.perf_counter()

    print("Floyd-Warshall time:", end - start, "seconds")
    print(f"Shortest distance from {source} to {target} using Floyd-Warshall:", fw[source][target])