from graph import Graph


graph = Graph()

graph.add_node("1")
print(dict(graph.adj_outer))

graph.add_node("2")
print(dict(graph.adj_outer))
graph.add_node(8)
print(dict(graph.adj_outer))
graph.add_node(2)
print(dict(graph.adj_outer))

