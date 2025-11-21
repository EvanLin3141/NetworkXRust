1. **`dfs_edges(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Yields edges in DFS order.
- Supports custom neighbor sorting.
- Can limit search depth.
- If `source=None`, DFS covers all components.

---

2. **`dfs_tree(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Returns a directed graph (DiGraph) representing the DFS tree.
- Uses `dfs_edges()` internally.

---

3. **`dfs_predecessors(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Returns a dictionary mapping each node → its DFS predecessor.
- Useful for reconstructing DFS paths and the DFS tree.

---

4. **`dfs_successors(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Returns a dictionary mapping each node → list of its DFS successors.
- Opposite of `dfs_predecessors`.

---

5. **`dfs_preorder_nodes(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Generates nodes in preorder (visit node **before** exploring children).
- Useful for traversal-based algorithms.

---

6. **`dfs_postorder_nodes(G, source=None, depth_limit=None, sort_neighbors=None)`**
- Generates nodes in postorder (visit node **after** exploring children).
- Useful for algorithms like topological sorting.

---

## 7. `dfs_labeled_edges(G, source=None, depth_limit=None, sort_neighbors=None)`
- Yields triples `(u, v, label)` where `label` is one of:
  - `"forward"` — visiting an unvisited child  
  - `"nontree"` — visiting an already visited node  
  - `"reverse"` — backtracking  
  - `"reverse-depth_limit"` — backtracking due to hitting depth limit  
- Shows the full DFS trace including backtracking.

---

## General Notes
- If `source` is omitted, DFS runs across all connected components.
- `depth_limit` defaults to `len(G)`, effectively unlimited.
- `sort_neighbors` allows custom ordering, e.g. `sorted`.

