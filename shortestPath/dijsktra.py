from collections import deque
from heapq import heappop, heappush
from itertools import count   

def _weight_function(G, weight):
    if callable(weight):
            return weight
    
    # if G.is_multigraph():
    #     return lambda u, v, d: min(attr.get(weight, 1) for attr in d.values())
    
    return lambda u, v, data: data.get(weight, 1)

def dijkstra_path(G, source, target, weight="weight"):
    """
          weight : string or function
        If this is a string, then edge weights will be accessed via the
        edge attribute with this key (that is, the weight of the edge
        joining `u` to `v` will be ``G.edges[u, v][weight]``). If no
        such edge attribute exists, the weight of the edge is assumed to
        be one.

        If this is a function, the weight of an edge is the value
        returned by the function. The function must accept exactly three
        positional arguments: the two endpoints of an edge and the
        dictionary of edge attributes for that edge. The function must
        return a number or None to indicate a hidden edge.

    Returns
    -------
    path : list
        List of nodes in a shortest path.
    """

    (length, path) =  single_source_dijkstra(G, source, target=target, weight=weight)
    return path

def single_source_dijkstra(G, source, target=None, cutoff=None, weight="weight"):
     
    """Find shortest weighted paths and lengths from a source node.

    Compute the shortest path length between source and all other
    reachable nodes for a weighted graph.

    Uses Dijkstra's algorithm to compute shortest paths and lengths
    between a source and all other reachable nodes in a weighted graph.
    
    
    Returns
    -------
    distance, path : pair of dictionaries, or numeric and list.
        If target is None, paths and lengths to all nodes are computed.
        The return value is a tuple of two dictionaries keyed by target nodes.
        The first dictionary stores distance to each target node.
        The second stores the path to each target node.
        If target is not None, returns a tuple (distance, path), where
        distance is the distance from source to target and path is a list
        representing the path from source to target.

            Notes
    -----
    Edge weight attributes must be numerical.
    Distances are calculated as sums of weighted edges traversed.

    The weight function can be used to hide edges by returning None.
    So ``weight = lambda u, v, d: 1 if d['color']=="red" else None``
    will find the shortest red path.
    """

    return multi_source_dijkstra(G, {source}, cutoff=cutoff, target=target, weight=weight)

def multi_source_dijkstra(G, sources, target=None, cutoff=None, weight="weight"):
    
    # Input validation
    if not sources:
        raise ValueError("sources must not be empty")
    for s in sources:
        if s not in G:
            raise ValueError(f"Node {s} not found in graph")

    if target in sources:
        return (0, [target])

    weight = _weight_function(G, weight)
    # Source points to itself
    paths = {source: [source] for source in sources} # Dictionary of paths

    dist = _dijkstra_multisource(G, sources, weight, paths=paths, cutoff=cutoff, target=target)

    if target is None:
        return (dist, paths)
    try: 
        return (dist[target], paths[target])
    except KeyError as err:
        raise ValueError(f"No path to {target}.") 
    
        
# Uses Dijkstra's algorithm to find shortest weighted paths
def _dijkstra_multisource(G, sources, weight, pred=None, paths=None, cutoff=None, target=None):
    
    G_succ = G.adj # Speed-up (Directed and undirected)

    dist = {}   # dictionary of final distances
    seen = {}   

    # fringe is heapq with 3-tuples (distance,c,node)
    # use the count c to avoid comparing nodes (may not be able to)
    c = count()
    fringe = []

    for source in sources:
        seen[source] = 0
        heappush(fringe, (0, next(c), source))
    while fringe:   #while fringe is not none:
        (distance, _, node) = heappop(fringe)
        if node in dist:
            continue    #already searched this node. 
                        #Skips the while loop
        dist[node] = distance  #distance[node] = distance
        if node == target:           # If found, then stop
            break 

        # For each v, adj[u] do RELAX(u,v,w)
        for nbr, e in G_succ[node].items():
            cost = weight(node, nbr, e)    #weight -> function _weight_function
            if cost is None:
                continue    #skip

            node_to_nbr_dist = dist[node] + cost
            if cutoff is not None:
                if node_to_nbr_dist > cutoff:
                    continue    #if distance exceeds cutoff, skip

            # If nbr in dist == finalized, then check if negative weights. Dijsktra doesnt allow neg weights
            if nbr in dist:
                nbr_dist = dist[nbr] 
                if node_to_nbr_dist < nbr_dist:
                    raise ValueError("Contradictory paths found:", "negative weights?")
                # Another path of same length, i.e another shortest path
                elif pred is not None and node_to_nbr_dist == nbr_dist:  # Confirming the weight is correct
                    pred[nbr].append(node)     #pred[u] = v / pred{u : v}
            
            # Update, push onto heap. Path reconstruction
            elif nbr not in seen or node_to_nbr_dist < seen[nbr]: 
                seen[nbr]= node_to_nbr_dist
                heappush(fringe, (node_to_nbr_dist, next(c), nbr))
                if paths is not None:
                    paths[nbr] = paths[node]+[nbr]
                if pred is not None:
                    pred[nbr] = [node]
            # Multipath, if same then more than 1 path
            elif node_to_nbr_dist == seen[nbr]:
                if pred is not None:
                    pred[nbr].append(node)    
    return dist