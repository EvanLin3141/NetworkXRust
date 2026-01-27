from graph import Graph
import networkx as nx


graph = Graph()
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

