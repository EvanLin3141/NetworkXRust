use std::collections::{HashSet, VecDeque};
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
    let limit = depth_limit.unwrap_or(g.node.len());

    let mut bfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut seen: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut queue: VecDeque<(&'a N, usize)> = VecDeque::new();

    let get_children = |n: &'a N| -> &'a [N] {
        match g.neighbors.get(n) {
            Some(v) => v.as_slice(),
            None => &[],
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

pub fn bfs_edges_digraph<'a, N>(
    g: &'a DiGraph<N>,
    source: &'a N,
    depth_limit: Option<usize>,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone,
{
    let limit = depth_limit.unwrap_or(g.node.len());

    let mut bfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut seen: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut queue: VecDeque<(&'a N, usize)> = VecDeque::new();

    let get_children = |n: &'a N| -> &'a [N] {
        match g.adj_outer_cache.get(n) {
            Some(v) => v.as_slice(),
            None => &[],
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