#[warn(unused_imports)]
use std::collections::{HashSet,HashMap};
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

struct Frame<'a, N> {
    parent: &'a N,
    children: Vec<&'a N>,
    i: usize,
    depth: usize,
}

struct FrameV2 {
    parent_idx: usize,
    child_pos: usize,
    depth: usize,
}

pub fn dfs_edges_v3<'a, N>(
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
    let mut visited: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut dfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut stack: Vec<Frame<'a, N>> = Vec::with_capacity(g.node.len());

    let get_children = |n: &'a N| -> Vec<&'a N> {
        match g.neighbors.get(n) {
            Some(v) => v.iter().collect(),
            None => Vec::new(),
        }
    };

    visited.insert(source);

    stack.push(Frame {
        parent: source,
        children: get_children(source),
        i: 0,
        depth: 0,
    });

    while let Some(frame) = stack.last_mut() {
        if frame.i >= frame.children.len() {
            stack.pop();
            continue;
        }

        let child = frame.children[frame.i];
        frame.i += 1;

        let parent = frame.parent;
        let next_depth = frame.depth + 1;

        if visited.insert(child) {
            dfs_path.push((parent, child));

            if next_depth < limit {
                stack.push(Frame {
                    parent: child,
                    children: get_children(child),
                    i: 0,
                    depth: next_depth,
                });
            }
        }
    }

    Ok(dfs_path)
}

pub fn dfs_edges_v2<'a, N>(
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
    let mut stack: Vec<FrameV2> = Vec::with_capacity(g.node_count());

    visited[source_idx] = true;

    stack.push(FrameV2 {
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
                stack.push(FrameV2 {
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
    if !g.node.contains_key(source) {
        return Err("Source node is not in the graph.".to_string());
    }

    let limit = depth_limit.unwrap_or(g.node.len());
    let mut visited: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut dfs_path: Vec<(&'a N, &'a N)> =
        Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut stack: Vec<Frame<'a, N>> = Vec::with_capacity(g.node.len());

    let get_children = |n: &'a N| -> Vec<&'a N> {
        match g.neighbors.get(n) {
            Some(v) => v.iter().collect(),
            None => Vec::new(),
        }
    };

    visited.insert(source);

    stack.push(Frame {
        parent: source,
        children: get_children(source),
        i: 0,
        depth: 0,
    });

    loop {
        let (parent, child, next_depth) = match stack.last_mut() {
            Some(frame) => {
                if frame.i >= frame.children.len() {
                    stack.pop();
                    continue;
                }

                let child = frame.children[frame.i];
                frame.i += 1;

                (frame.parent, child, frame.depth + 1)
            }
            None => break,
        };

        if visited.insert(child) {
            dfs_path.push((parent, child));

            if next_depth < limit {
                stack.push(Frame {
                    parent: child,
                    children: get_children(child),
                    i: 0,
                    depth: next_depth,
                });
            }
        }
    }

    Ok(dfs_path)
}