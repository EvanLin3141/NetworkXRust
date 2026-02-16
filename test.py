from Graph.Graph import *
from traversal.dfs import *
from traversal.bfs import *
from shortestPath.dijsktra import *
from networkX import *
"""
mgraph = nx.MultiGraph()

graph.add_node("1")
print(dict(graph.adj_outer))

graph.add_node("2")
print(dict(graph.adj_outer))
graph.add_node(8)
print(dict(graph.adj_outer))
graph.add_node(2)
print(dict(graph.adj_outer))

graph.add_edge("1", "2", color="red")
print(dict(graph.adj_outer))
graph.add_edge("1", "2", color="green")
print(dict(graph.adj_outer))
graph.add_edge("1", "2", color="red")
print(dict(graph.adj_outer))

# Adds an additional key weight after color
graph.add_edge("1", "2", weight=3)
print(dict(graph.adj_outer))
graph.remove_edge("2","1")
print(dict(graph.adj_outer))
graph.remove_node("2")
print(dict(graph.adj_outer))
"""

# #Testing DFS
# # Add nodes individually
# dfs = Graph()

# dfs.add_node(1)
# dfs.add_node(2)
# dfs.add_node(3)
# dfs.add_node(4)
# dfs.add_node(5)

# # Add edges (this automatically adds nodes if missing)
# dfs.add_edge(1, 2)
# dfs.add_edge(1, 3)
# dfs.add_edge(2, 4)
# dfs.add_edge(2, 5)

# edges = list(dfs_edges(dfs, source=1))
# print("DFS edges:", edges)


#DFS with depth of 7# Create graph
# G = Graph()

# # Level 1
# G.add_edge(1, 2)
# G.add_edge(1, 3)

# # Level 2
# G.add_edge(2, 4)
# G.add_edge(2, 5)
# G.add_edge(2, 6)
# G.add_edge(3, 7)

# # Level 3
# G.add_edge(4, 8)
# G.add_edge(7, 9)
# G.add_edge(7, 10)
# G.add_edge(7, 11)

# # Level 4
# G.add_edge(8, 12)

# # Level 5
# G.add_edge(12, 13)

# # DFS with depth limit 7 (or None for full depth)
# edges = list(dfs_edges(G, source=1, depth_limit=7))

# # print("DFS edges:", edges)
# # print("Number of nodes:", len(G))
# # # [(1, 2), (2, 4), (4, 8), (8, 12), (12, 13), (2, 5), (2, 6), (1, 3), (3, 7), (7, 9), (7, 10), (7, 11)]

# edges = list(bfs_edges(G, source=1))
# print("BFS edges:", edges)
# print("Number of nodes:", len(G))

# # --- create a custom graph to test ---
# G = nx.Graph(name="DemoGraph")

# # Add nodes with attributes
# G.add_node("A", color="red")
# G.add_node("B", color="blue")
# G.add_node("C", color="green")
# G.add_node("D", color="yellow")

# # Add edges with attributes
# G.add_edge("A", "B", weight=5, relation="friends")
# G.add_edge("A", "C", weight=2)
# G.add_edge("B", "C", weight=7)
# G.add_edge("C", "D", weight=1, label="bridge")
# G.add_edge("A", "D")  # edge with no attributes

# # --- test the exact loop you asked about ---
# u = "A"
# print(f"Neighbors of {u}:")

# for v, d in G.adj[u].items():
#     print(f"  {u} -> {v}, edge_attr = {d}")

# # Optional: show full adjacency dict
# print("\nFull adjacency dict:")
# print(G.adj)

# G = Graph()

# # Add edges (undirected by your add_edge implementation)
# G.add_edge("A", "B", weight=1)
# G.add_edge("B", "C", weight=2)
# G.add_edge("C", "D", weight=3)
# G.add_edge("A", "C", weight=4)
# G.add_edge("B", "D", weight=5)
# mst_edges = list(prim_mst_edges(G, weight="weight", data=True))

# # for u, v, attr in mst_edges:
# #     print(f"{u} -- {v}, weight = {attr['weight']}")
    
    
# V = Graph()

# edges = [
#     ("A", "B", 4),
#     ("A", "C", 3),
#     ("B", "C", 1),
#     ("B", "D", 2),
#     ("C", "D", 4),
#     ("C", "E", 2),
#     ("D", "F", 3),
#     ("E", "F", 1),
#     ("E", "G", 5),
#     ("F", "H", 2),
#     ("G", "H", 3),
#     ("H", "I", 4),
#     ("I", "J", 1),
#     ("H", "J", 6),
# ]

# for u, v, w in edges:
#     V.add_edge(u, v, weight=w)

# mst = list(prim_mst_edges(V, weight="weight", data=True))

# for u, v, attr in mst:
#     print(f"{u} -- {v}, weight = {attr['weight']}")


# X = Graph()
# edges = [
#     ("A", "B", 1),
#     ("A", "C", 4),
#     ("B", "C", 1),
#     ("B", "D", 2),
#     ("C", "D", 1),
# ]

# for u, v, w in edges:
#     X.add_edge(u, v, weight=w)

# sources = ["A", "D"]

# dist = dijkstra_path(X, sources[0],sources[1])
# print(dist)

# dist = multi_source_dijkstra(X, "A", "D")
# print(dist)
# from floyd import *

# H = Graph()
# edges =[
#         ("s", "u", 10),
#         ("s", "x", 5),
#         ("u", "v", 1),
#         ("u", "x", 2),
#         ("v", "y", 1),
#         ("x", "u", 3),
#         ("x", "v", 5),
#         ("x", "y", 2),
#         ("y", "s", 7),
#         ("y", "v", 6),
#     ]

# for u, v, w in edges:
#     H.add_edge(u, v, weight=w)  

# predecessors, distance = floyd_warshall(H)
# print(distance)
# print("")
# print(predecessors)
import time
import networkx as nx

# --- Build graph (undirected, like your Rust add_edge adds both directions) ---
G = Graph()

# Graph-level attributes
G.graph["name"] = "nxGraph"

# Node attributes
G.add_node("A", color="amber")
G.add_node("B", color=True)  # matches your Bool(true) example

# Edges + attributes
G.add_edge("A", "B", weight=5, relation="friend")
G.add_edge("A", "C", weight=2, relation="colleague")
G.add_edge("B", "D", weight=7, cost=1.5)
G.add_edge("B", "E", weight=1, enabled=False)
G.add_edge("C", "F", weight=9, relation="family")
G.add_edge("E", "F", weight=3, relation="connected")

import timeit
time_taken = timeit.timeit(
    "list(dfs_edges(G, source='A'))",
    globals=globals(),
    number=100000
)
print("Avg:", time_taken / 100000)

