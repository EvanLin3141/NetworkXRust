use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

pub fn bfs_edges<N> (
    g: &Graph<N>,
    source: Option<N>,
    depth_limit: Option<usize>,
) -> Result<Vec<(N, N)>, String>
where
    N: Eq + Hash + Clone
{
    let source = match source {
        Some(s) => s,
        None => return Err("bfs_edges error: source is None".to_string()),
    };

    let limit = depth_limit.unwrap_or(g.node.len());

    let mut bfs_path: Vec<(N, N)> = Vec::new();
    let mut seen: HashSet<N> = HashSet::new();
    let mut queue: VecDeque<(N, usize)> = VecDeque::new();

    seen.insert(source.clone());
    queue.push_back((source,0));

    while let Some((parent, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        };

        let neighbors = g.neighbors(&parent)?;
        for child in neighbors {
            if seen.insert(child.clone()) {
                bfs_path.push((parent.clone(), child.clone()));
                queue.push_back((child.clone(), depth + 1));
            }
        }
    }
    Ok(bfs_path)
}

pub fn bfs_edges_ref<'a, N>(
    g: &'a Graph<N>,
    source: &'a N,
    depth_limit: Option<usize>,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone,
{
    let limit = depth_limit.unwrap_or(g.node.len());

    let mut bfs_path: Vec<(&'a N, &'a N)> = Vec::new();
    let mut seen: HashSet<&'a N> = HashSet::new();
    let mut queue: VecDeque<(&'a N, usize)> = VecDeque::new();

    seen.insert(source);
    queue.push_back((source, 0));

    while let Some((parent, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        }

        let nbrs = g
            .adj_outer
            .get(parent)
            .ok_or_else(|| "bfs_edges_ref error: parent not in graph".to_string())?;

        for child in nbrs.keys() {
            if seen.insert(child) {
                bfs_path.push((parent, child));
                queue.push_back((child, depth + 1));
            }
        }
    }

    Ok(bfs_path)
}