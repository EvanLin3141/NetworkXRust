use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

pub fn dfs_edges<N>(
    g: &Graph<N>,
    source: Option<N>,
    depth_limit: Option<usize>,
) -> Vec<(N, N)>
where
    N: Eq + Hash + Clone + Ord,
{
    let limit = depth_limit.unwrap_or(g.node.len());

    let start: N = match source {
        Some(s) => s,
        None => g.node.keys().next().expect("Graph is empty").clone(),
    };

    let get_children = |n: &N| -> std::vec::IntoIter<N> {
        let mut kids: Vec<N> = g.adj_outer
            .get(n)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
        kids.sort(); 
        kids.into_iter()
    };

    let mut visited: HashSet<N> = HashSet::new();
    let mut dfs_path: Vec<(N, N)> = Vec::new();

    visited.insert(start.clone());

    let mut stack: Vec<(N, std::vec::IntoIter<N>, usize)> = Vec::new();
    stack.push((start.clone(), get_children(&start), 0)); // depth=0

    while let Some((parent, mut children, depth)) = stack.pop() {
        if let Some(child) = children.next() {
            stack.push((parent.clone(), children, depth));

            if visited.contains(&child) {
                continue;
            }

            dfs_path.push((parent.clone(), child.clone()));
            visited.insert(child.clone());

            if depth < limit {
                stack.push((child.clone(), get_children(&child), depth + 1));
            }
        }
    }
    dfs_path
}
