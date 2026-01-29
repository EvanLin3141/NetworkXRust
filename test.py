from graph import Graph
import networkx as nx
from traversal import *

graph = Graph()
mgraph = nx.MultiGraph()
dfs = Graph()

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


#Testing DFS
# Add nodes individually
dfs.add_node(1)
dfs.add_node(2)
dfs.add_node(3)
dfs.add_node(4)
dfs.add_node(5)

# Add edges (this automatically adds nodes if missing)
dfs.add_edge(1, 2)
dfs.add_edge(1, 3)
dfs.add_edge(2, 4)
dfs.add_edge(2, 5)

edges = list(dfs_edges(dfs, source=1))
print("DFS edges:", edges)


#DFS with depth of 7# Create graph
G = Graph()

# Level 1
G.add_edge(1, 2)
G.add_edge(1, 3)

# Level 2
G.add_edge(2, 4)
G.add_edge(2, 5)
G.add_edge(2, 6)
G.add_edge(3, 7)

# Level 3
G.add_edge(4, 8)
G.add_edge(7, 9)
G.add_edge(7, 10)
G.add_edge(7, 11)

# Level 4
G.add_edge(8, 12)

# Level 5
G.add_edge(12, 13)

# DFS with depth limit 7 (or None for full depth)
edges = list(dfs_edges(G, source=1, depth_limit=7))

print("DFS edges:", edges)
print("Number of nodes:", len(G))
# [(1, 2), (2, 4), (4, 8), (8, 12), (12, 13), (2, 5), (2, 6), (1, 3), (3, 7), (7, 9), (7, 10), (7, 11)]


