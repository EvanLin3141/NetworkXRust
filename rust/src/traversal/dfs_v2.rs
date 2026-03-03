use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

struct Frame<'a, N> {
    parent: &'a N,
    children: &'a [N],
    i: usize,
    depth: usize,
}

pub fn dfs_edges_v2<'a, N>(
    g: &'a Graph<N>,
    source: &'a N,
    depth_limit: Option<usize>
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone + Ord,
{
    let limit = depth_limit.unwrap_or(g.node.len());
    let mut visited: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut dfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut stack: Vec<Frame<'a, N>> = Vec::with_capacity(g.node.len());

    let get_children = |n: &'a N| -> &'a [N] {
        match g.neighbors.get(n) {
            Some(v) => v.as_slice(),
            None => &[],
        }
    };

    visited.insert(source);

    stack.push(
        Frame {
            parent: source,
            children: get_children(source),
            i: 0,
            depth: 0,
        }
    );

    loop {
        let (parent, child, next_depth) = match stack.last_mut() {
            Some(frame) => {
                if frame.i >= frame.children.len() {
                    stack.pop();
                    continue;
                }

                let child: &'a N = &frame.children[frame.i];
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

pub fn dfs_edges_digraph<'a, N>(
    g: &'a DiGraph<N>,
    source: &'a N,
    depth_limit: Option<usize>
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Clone + Ord,
{
    let limit = depth_limit.unwrap_or(g.node.len());
    let mut visited: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
    let mut dfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
    let mut stack: Vec<Frame<'a, N>> = Vec::with_capacity(g.node.len());

    let get_children = |n: &'a N| -> &'a [N] {
        match g.adj_outer_cache.get(n) {
            Some(v) => v.as_slice(),
            None => &[],
        }
    };

    visited.insert(source);

    stack.push(
        Frame {
            parent: source,
            children: get_children(source),
            i: 0,
            depth: 0,
        }
    );

    loop {
        let (parent, child, next_depth) = match stack.last_mut() {
            Some(frame) => {
                if frame.i >= frame.children.len() {
                    stack.pop();
                    continue;
                }

                let child: &'a N = &frame.children[frame.i];
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