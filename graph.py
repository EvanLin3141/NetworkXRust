class Graph:
    node_dict = dict
    node_attr = dict
    adj_outer = dict    #creates adj
    adj_inner = dict    #creates adj[node]
    edge = dict         #creates adj[from_node][to_node]
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

        if node not in self.graph:
            raise ValueError("This node does not exist in the graph")
        
        adj = self.adj_outer
        for neighbour in list(node):
            del adj[neighbour][node]
        
        del adj[node]
        del self.node[node]

    #def __str__(self):
        



