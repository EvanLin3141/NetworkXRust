import networkx as nx
import time

graph_files = [
    "sparse_graph.txt",
    "medium_graph.txt",
    "dense_graph.txt",
    "directed_sparse_graph.txt",
    "directed_medium_graph.txt",
    "directed_dense_graph.txt",
]

for file in graph_files:
    print("\n--- Testing", file, "---")

    is_directed = file.startswith("directed_")

    # Load graph
    if is_directed:
        G_loaded = nx.read_weighted_edgelist(
            file,
            nodetype=int,
            create_using=nx.DiGraph(),
        )
    else:
        G_loaded = nx.read_weighted_edgelist(file, nodetype=int)

    if is_directed:
        print("Weakly connected:", nx.is_weakly_connected(G_loaded))
    else:
        print("Connected:", nx.is_connected(G_loaded))

    print("Nodes:", G_loaded.number_of_nodes())
    print("Edges:", G_loaded.number_of_edges())

    source = 0
    target = 500

    # BFS
    start = time.perf_counter()
    bfs_edges = list(nx.bfs_edges(G_loaded, source=source))
    end = time.perf_counter()
    print("BFS time:", (end - start) * 1000, "ms")

    # DFS
    start = time.perf_counter()
    dfs_edges = list(nx.dfs_edges(G_loaded, source=source))
    end = time.perf_counter()
    print("DFS time:", (end - start) * 1000, "ms")

    # Prim only for undirected graphs
    if not is_directed:
        start = time.perf_counter()
        mst = nx.minimum_spanning_tree(G_loaded, algorithm="prim", weight="weight")
        end = time.perf_counter()

        print("Prim's time:", (end - start) * 1000, "ms")
        print("MST edges:", mst.number_of_edges())

    # Dijkstra
    start = time.perf_counter()
    dijkstra_path = nx.dijkstra_path(G_loaded, source=source, target=target, weight="weight")
    dijkstra_cost = nx.dijkstra_path_length(G_loaded, source=source, target=target, weight="weight")
    end = time.perf_counter()

    print("Dijkstra time:", (end - start) * 1000, "ms")
    print("Dijkstra cost:", dijkstra_cost)

    # Floyd-Warshall
    start = time.perf_counter()
    fw = dict(nx.floyd_warshall(G_loaded, weight="weight"))
    end = time.perf_counter()

    print("Floyd-Warshall time:", (end - start) * 1000, "ms")
    print(f"Shortest distance from {source} to {target} using Floyd-Warshall:", fw[source][target])