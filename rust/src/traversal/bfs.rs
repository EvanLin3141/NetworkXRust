use std::collections::VecDeque;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

pub fn bfs_edges<'a, N>(
    g: &'a Graph<N>,
    source: &'a N,
    depth_limit: Option<usize>,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone,
{
    let source_idx = g
        .get_index(source)
        .ok_or_else(|| "Source node is not in the graph.".to_string())?;

    let limit = depth_limit.unwrap_or(g.node_count());

    let mut seen = vec![false; g.node_count()];
    let mut bfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node_count().saturating_sub(1));
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(g.node_count());

    seen[source_idx] = true;
    queue.push_back((source_idx, 0));

    while let Some((parent_idx, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        }

        for &child_idx in &g.adj_idx[parent_idx] {
            if !seen[child_idx] {
                seen[child_idx] = true;

                let parent = &g.idx_to_node[parent_idx];
                let child = &g.idx_to_node[child_idx];

                bfs_path.push((parent, child));
                queue.push_back((child_idx, depth + 1));
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
    let source_idx = g
        .get_index(source)
        .ok_or_else(|| "Source node is not in the graph.".to_string())?;

    let limit = depth_limit.unwrap_or(g.node_count());

    let mut seen = vec![false; g.node_count()];
    let mut bfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node_count().saturating_sub(1));
    let mut queue: VecDeque<(usize, usize)> = VecDeque::with_capacity(g.node_count());

    seen[source_idx] = true;
    queue.push_back((source_idx, 0));

    while let Some((parent_idx, depth)) = queue.pop_front() {
        if depth >= limit {
            continue;
        }

        for &child_idx in &g.adj_idx[parent_idx] {
            if !seen[child_idx] {
                seen[child_idx] = true;

                let parent = &g.idx_to_node[parent_idx];
                let child = &g.idx_to_node[child_idx];

                bfs_path.push((parent, child));
                queue.push_back((child_idx, depth + 1));
            }
        }
    }

    Ok(bfs_path)
}