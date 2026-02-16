#Iterate over edges in a depth-first-search (DFS)
def dfs_edges(G, source=None, depth_limit=None, *, sort_neighbours=None):
    if source is None:
        source = G
    else:
        nodes = [source]
    
    if depth_limit is None:
        depth_limit = len(G)

    
    get_children = (
        G.neighbour
        if sort_neighbours is None  #if None do nothing
        else lambda n: iter(sort_neighbours(G.neighbour(n)))
    )

    visited = set()

    for node in nodes:
        if node in visited:
            continue
        visited.add(node)
        stack = [(node, get_children(node))]
        depth_now = 1
        while stack:
            parent, children = stack[-1]    # Pops top of stack
            for child in children:
                if child not in visited:

                    #all_edges.append((parent, child))  # instead of yield
                    
                    yield parent, child
                    visited.add(child)
                    if depth_now < depth_limit:
                        stack.append((child, get_children(child)))  # Add child to our stack for iterative DFS
                        depth_now += 1
                        break
            else:
                stack.pop()
                depth_now -= 1

