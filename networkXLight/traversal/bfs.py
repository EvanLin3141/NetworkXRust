from collections import deque



def bfs_edges(G, source, neighbors=None, depth_limit=None):

    if neighbors is None:
        neighbors = G.neighbour
    if depth_limit is None:
        depth_limit = len(G)
        
    seen = {source}
    n = len(G)
    depth = 0
    
    next = [(source, neighbors(source))]   
    while next and depth < depth_limit:
        this = next
        next = []
        for parent, children in this:
            for child in children:
                if child not in seen:
                    seen.add(child)
                    next.append((child, neighbors(child)))
                    yield parent, child
            if len(seen) == n: 
                return
        
        depth += 1






