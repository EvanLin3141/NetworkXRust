class Graph:
    node_dict = dict
    node_attr = dict
    adj_outer = dict    #creates adj
    adj_inner = dict    #creates adj[node]
    edge_attr = dict         #creates adj[from_node][to_node]
    graph_attr = dict

    edge = {}

    def __init__(self, incoming_graph=None, **graph_attributes):

        self.graph = self.graph_attr()
        self.node = self.node_dict()
        self.adj_outer = self.adj_outer()

        #if incoming_graph is not None:
            # TODO:

        self.graph.update(graph_attributes)
    
    def add_node(self, new_node, **attr):
        if new_node is None:
            raise ValueError("This is not a valid node")

        if new_node not in self.node:
            self.adj_outer[new_node] = self.adj_inner()
            attribute_dict = self.node[new_node] = self.node_attr()
            attribute_dict.update(attr)
        else:
            self.node[new_node].update(attribute_dict)

    def remove_node(self, node):

        adj = self.adj_outer
        try:
            neighbour = list(adj[node])
            del self.node[node]
        except:
            raise ValueError("This node does not exist in the graph")
        
        for u in neighbour:
            del adj[u][node]
        
        del adj[node]


        """
        adj = self._adj
        try:
            nbrs = list(adj[n])  # list handles self-loops (allows mutation)
            del self._node[n]    # delete node after getting the neighbours
        except KeyError as err:  # NetworkXError if n not in self
            raise NetworkXError(f"The node {n} is not in the graph.") from err
        for u in nbrs:
            del adj[u][n]  # remove all edges n-u in graph
        del adj[n]  # now remove node
        nx._clear_cache(self)"""

    def add_edge(self, u_of_edge, v_of_edge, **edge_attr):
        u, v = u_of_edge, v_of_edge

        # If any node not exist; create
        if u not in self.node:
            if u is None:
                raise ValueError("None cannot be a node")
            self.adj_outer[u] = self.adj_inner()
            self.node[u] = self.node_attr()
        
        if v not in self.node:
            if v is None:
                raise ValueError("None cannot be a node")
            self.adj_outer[v] = self.adj_inner()
            self.node[v] = self.node_attr()

        neighbourDict = self.adj_outer[u].get(v, self.edge_attr())  # Gets all neighbor of U
        neighbourDict.update(edge_attr)                             # Update the additional edge attribute
        self.adj_outer[u][v] = neighbourDict                        # Update on "U"
        self.adj_outer[v][u] = neighbourDict                        # Update on "V"

    def remove_edge(self, u, v):

        try:
            del self.adj_outer[u][v]
            if u != v:
                del self.adj_outer[v][u]
        except KeyError as err:
            raise ValueError(f"The edge {u}-{v} is not in the graph") from err

        


    #def __str__ (self):
    #    return
        



