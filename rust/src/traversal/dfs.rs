use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

pub fn dfs_edges<N>(
    g: &Graph<N>,
    source: Option<N>,
    depth_limit: Option<usize>,
) -> Vec<(N, N)>
where
    N: Eq + Hash + Clone,
{
    let limit = depth_limit.unwrap_or(g.node.len());

    let start: N = match source {
        Some(s) => s,
        None => g.node.keys().next().expect("Graph is empty").clone(),
    };

    let get_children = |n: &N| -> std::vec::IntoIter<N> {
        let kids: Vec<N> = g.adj_outer
            .get(n)
            .map(|m| m.keys().cloned().collect::<Vec<N>>())
            .unwrap_or_default();

        kids.into_iter()
    };

    let mut visited: HashSet<N> = HashSet::new();
    let mut dfs_path: Vec<(N, N)> = Vec::new();

    visited.insert(start.clone());

    // stack holds (parent, iterator over its children)
    let mut stack: Vec<(N, std::vec::IntoIter<N>)> = Vec::new();
    stack.push((start.clone(), get_children(&start)));

    // Python: depth_now starts at 1; in this approach depth is stack.len()
    while let Some((parent, mut children)) = stack.pop() {
        // Try to advance parent's iterator by one
        if let Some(child) = children.next() {
            // Put parent back with iterator advanced (resume later)
            stack.push((parent.clone(), children));

            if visited.contains(&child) {
                continue;
            }

            // Instead of yield: collect edge
            dfs_path.push((parent.clone(), child.clone()));
            visited.insert(child.clone());

            // Go deeper if within limit
            if stack.len() < limit {
                stack.push((child.clone(), get_children(&child)));
            }
        }
        // else: iterator exhausted -> backtrack automatically (don’t re-push)
    }

    dfs_path
}
