def floyd_warshall(G, weight="weight"):
    
    # Initializing our matrix
    dist = {}
    nodes = list(G.adj.keys())
    for u in nodes:
        dist[u] = {}
        for v in nodes:
            dist[u][v] = float("inf")
    
    for v in G:
        dist[v][v] = 0
        
    pred = {}
    for u in nodes:
        pred[u] = {}
    
    # For each Vertex in Graph
    undirected = True
    for u, v, d in G.edges(data=True):
        edge_weight = d.get(weight, 1.0)
        dist[u][v] = min(edge_weight, dist[u][v])
        pred[u][v] = u
        if undirected:
            dist[v][u] = min(edge_weight, dist[v][u])
            pred[v][u] = v
            
    # Triple Loop
    # W = intermediate node
    # u = source, v = target
    for w in G:
        distW = dist[w] 
        for u in G:
            distU = dist[u] 
            for v in G:
                d = distU[w] + distW[v]
                if distU[v] > d:
                    distU[v] = d
                    pred[u][v] = pred[w][v]
    
    return dict(pred), dict(dist)