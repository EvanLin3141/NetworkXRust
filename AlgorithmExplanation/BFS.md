# Overview
# The code implements various Breadth-First Search (BFS) algorithms for traversing 
# and analyzing graphs in the NetworkX library. BFS explores nodes in a graph 
# level by level, starting from a source node.

# Key Functions

1. **`generic_bfs_edges`**: 
   - Iterates over edges in a BFS traversal from a given `source` node.
   - Allows custom neighbor ordering and depth limits.

2. **`bfs_edges`**: 
   - Similar to `generic_bfs_edges`, but specifically for iterating over edges in a BFS traversal.
   - Supports reverse traversal for directed graphs and neighbor sorting.

3. **`bfs_tree`**: 
   - Constructs a **BFS tree** starting from a source node.
   - Returns a directed graph (`DiGraph`) representing the BFS tree.

4. **`bfs_predecessors`**: 
   - Yields predecessors of nodes in a BFS traversal starting from a `source`.
   - Returns a list of nodes and their corresponding predecessors.

5. **`bfs_successors`**: 
   - Yields successors of nodes in a BFS traversal starting from a `source`.
   - Returns a list of nodes and their corresponding successors.

6. **`bfs_layers`**: 
   - Returns nodes grouped by their distance (layer) from the `source` node(s).
   - Useful for finding nodes at a specific distance from the start.

7. **`bfs_labeled_edges`**: 
   - Iterates over BFS edges and labels them as one of four types: **tree**, **forward**, **level**, or **reverse**.
   - Helps differentiate edge types in BFS (e.g., tree edges vs. back edges).

8. **`descendants_at_distance`**: 
   - Returns all nodes that are exactly at a given distance from the `source`.

