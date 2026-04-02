use std::collections::{HashSet, VecDeque, HashMap};
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

pub fn bfs_edges_v2<'a, N>(
    g: &'a Graph<N>,
    source: &'a N,
    depth_limit: Option<usize>,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone,
{
    if !g.node.contains_key(source) {
        return Err("Source node is not in the graph.".to_string());
    }

    let limit = depth_limit.unwrap_or(g.node.len());

    let mut node_to_idx: HashMap<&'a N, usize> = HashMap::with_capacity(g.node.len());
    for (i, node) in g.node.keys().enumerate() {
        node_to_idx.insert(node, i);
    }

    let mut seen = vec![false; g.node.len()];
    let mut bfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut queue: VecDeque<(&'a N, usize)> = VecDeque::with_capacity(g.node.len());

    let source_idx = node_to_idx[source];
    seen[source_idx] = true;
    queue.push_back((source, 0));

    while let Some((parent, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        }

        if let Some(children) = g.neighbors.get(parent) {
            for child in children {
                let child_idx = node_to_idx[child];
                if !seen[child_idx] {
                    seen[child_idx] = true;
                    bfs_path.push((parent, child));
                    queue.push_back((child, depth + 1));
                }
            }
        }
    }

    Ok(bfs_path)
}

pub fn bfs_edges_digraph<'a, N>(
    g: &'a DiGraph<N>,
    source: &'a N,
    depth_limit: Option<usize>,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone,
{
    if !g.node.contains_key(source) {
        return Err("Source node is not in the graph.".to_string());
    }

    let limit = depth_limit.unwrap_or(g.node.len());

    let mut bfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut seen: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut queue: VecDeque<(&'a N, usize)> = VecDeque::new();

    let get_children = |n: &'a N| -> Vec<&'a N> {
        match g.neighbors.get(n) {
            Some(v) => v.iter().collect(),
            None => Vec::new(),
        }
    };

    seen.insert(source);
    queue.push_back((source, 0));

    while let Some((parent, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        }

        for child in get_children(parent) {
            if seen.insert(child) {
                bfs_path.push((parent, child));
                queue.push_back((child, depth + 1));
            }
        }
    }

    Ok(bfs_path)
}