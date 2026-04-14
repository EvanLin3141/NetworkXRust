use std::collections::HashSet;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

struct Frame<'a, N> {
    parent: &'a N,
    children: Vec<&'a N>,
    i: usize,
    depth: usize,
}

pub fn dfs_edges_nx<N>(
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
        let kids: Vec<N> = g.adj_outer
            .get(n)
            .map(|m| m.keys().cloned().collect())
            .unwrap_or_default();
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

// pub fn dfs_edges_ref<'a, N>(
//     g: &'a Graph<N>,
//     source: &'a N,
//     depth_limit: Option<usize>,
// ) -> Result<Vec<(&'a N, &'a N)>, String>
// where
//     N: Eq + Hash + Clone + Ord,
// {
//     let limit = depth_limit.unwrap_or(g.node.len());

//     let start: &'a N = source;

//     let get_children = |n: &'a N| -> std::vec::IntoIter<&'a N> {
//         let kids: Vec<&'a N> = g
//             .adj_outer
//             .get(n)
//             .map(|m| m.keys().collect())
//             .unwrap_or_default();
//         // kids.sort();
//         kids.into_iter()
//     };

//     let mut visited: HashSet<&'a N> = HashSet::new();
//     let mut dfs_path: Vec<(&'a N, &'a N)> = Vec::new();

//     visited.insert(start);

//     let mut stack: Vec<(&'a N, std::vec::IntoIter<&'a N>, usize)> = Vec::new();
//     stack.push((start, get_children(start), 0));

//     while let Some((parent, mut children, depth)) = stack.pop() {
//         if let Some(child) = children.next() {
//             stack.push((parent, children, depth));

//             if visited.insert(child) {
//                 dfs_path.push((parent, child));

//                 if depth + 1 < limit {
//                     stack.push((child, get_children(child), depth + 1));
//                 }
//             }
//         }
//     }
//     Ok(dfs_path)
// }

// pub fn dfs_edges_v3<'a, N>(
//     g: &'a Graph<N>,
//     source: &'a N,
//     depth_limit: Option<usize>,
// ) -> Result<Vec<(&'a N, &'a N)>, String>
// where
//     N: Eq + Hash + Clone,
// {
//     if !g.node.contains_key(source) {
//         return Err("Source node is not in the graph.".to_string());
//     }

//     let limit = depth_limit.unwrap_or(g.node.len());
//     let mut visited: HashSet<&'a N> = HashSet::with_capacity(g.node.len());
//     let mut dfs_path: Vec<(&'a N, &'a N)> = Vec::with_capacity(g.node.len().saturating_sub(1));
//     let mut stack: Vec<Frame<'a, N>> = Vec::with_capacity(g.node.len());

//     let get_children = |n: &'a N| -> Vec<&'a N> {
//         match g.neighbors.get(n) {
//             Some(v) => v.iter().collect(),
//             None => Vec::new(),
//         }
//     };

//     visited.insert(source);

//     stack.push(Frame {
//         parent: source,
//         children: get_children(source),
//         i: 0,
//         depth: 0,
//     });

//     while let Some(frame) = stack.last_mut() {
//         if frame.i >= frame.children.len() {
//             stack.pop();
//             continue;
//         }

//         let child = frame.children[frame.i];
//         frame.i += 1;

//         let parent = frame.parent;
//         let next_depth = frame.depth + 1;

//         if visited.insert(child) {
//             dfs_path.push((parent, child));

//             if next_depth < limit {
//                 stack.push(Frame {
//                     parent: child,
//                     children: get_children(child),
//                     i: 0,
//                     depth: next_depth,
//                 });
//             }
//         }
//     }

//     Ok(dfs_path)
// }