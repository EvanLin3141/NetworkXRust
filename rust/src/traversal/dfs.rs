#[warn(unused_imports)]
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

struct Frame {
    parent_idx: usize,
    child_pos: usize,
    depth: usize,
}

pub fn dfs_edges<'a, N>(
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

    let mut visited = vec![false; g.node_count()];
    let mut dfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node_count().saturating_sub(1));
    let mut stack: Vec<Frame> = Vec::with_capacity(g.node_count());

    visited[source_idx] = true;

    stack.push(Frame {
        parent_idx: source_idx,
        child_pos: 0,
        depth: 0,
    });

    while let Some(frame) = stack.last_mut() {
        let parent_idx = frame.parent_idx;

        if frame.child_pos >= g.adj_idx[parent_idx].len() {
            stack.pop();
            continue;
        }

        let child_idx = g.adj_idx[parent_idx][frame.child_pos];
        frame.child_pos += 1;

        let next_depth = frame.depth + 1;

        if !visited[child_idx] {
            visited[child_idx] = true;

            let parent = &g.idx_to_node[parent_idx];
            let child = &g.idx_to_node[child_idx];
            dfs_path.push((parent, child));

            if next_depth < limit {
                stack.push(Frame {
                    parent_idx: child_idx,
                    child_pos: 0,
                    depth: next_depth,
                });
            }
        }
    }

    Ok(dfs_path)
}

pub fn dfs_edges_digraph<'a, N>(
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

    let mut visited = vec![false; g.node_count()];
    let mut dfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node_count().saturating_sub(1));
    let mut stack: Vec<Frame> = Vec::with_capacity(g.node_count());

    visited[source_idx] = true;

    stack.push(Frame {
        parent_idx: source_idx,
        child_pos: 0,
        depth: 0,
    });

    while let Some(frame) = stack.last_mut() {
        let parent_idx = frame.parent_idx;

        if frame.child_pos >= g.adj_idx[parent_idx].len() {
            stack.pop();
            continue;
        }

        let child_idx = g.adj_idx[parent_idx][frame.child_pos];
        frame.child_pos += 1;

        let next_depth = frame.depth + 1;

        if !visited[child_idx] {
            visited[child_idx] = true;

            let parent = &g.idx_to_node[parent_idx];
            let child = &g.idx_to_node[child_idx];
            dfs_path.push((parent, child));

            if next_depth < limit {
                stack.push(Frame {
                    parent_idx: child_idx,
                    child_pos: 0,
                    depth: next_depth,
                });
            }
        }
    }

    Ok(dfs_path)
}