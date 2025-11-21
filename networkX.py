import networkx as nx

### NOTES:
#
# Prevents duplicate node and edges (Undirected)

G = nx.Graph()

G.add_node(1)
G.add_nodes_from([2, 3])
G.add_nodes_from([(4, {"color": "red"}), (5, {"color": "green"})])

print(G)

G.add_edge(1,2)
e = (2,3)
G.add_edge(*e)
G.add_edges_from([(1, 2), (1, 3)])

print(G)

G.clear()

G.add_edges_from([(1, 4), (1, 11)])
G.add_edges_from([(2, 6), (2, 9)])
print(G)
G.add_node(1)
G.add_edge(1, 2)          # Duplicate Node and Edge
print(G)
G.add_node("spam")        # adds node "spam"
G.add_nodes_from("spam")  # adds 4 nodes: 's', 'p', 'a', 'm'
G.add_edge(3, 'm')

#print(G)


#Examining elements of a graph

#list(G.nodes)      #Doesn't Work
# print(G.nodes)
# print(G.edges)
# print(G.adj[1])
print(list(G.adj[1]))
print(list(G.neighbors(1)))
print(G.degree[1])
bfsNodes = list(nx.dfs_tree(G,1))
print(bfsNodes)