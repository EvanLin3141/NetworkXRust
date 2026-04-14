use std::collections::{HashSet, VecDeque};
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

// Original BFS NX Architecture - No Improvements
pub fn bfs_edges_nx<N> (
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

// pub fn bfs_edges_ref<'a, N>(
//     g: &'a Graph<N>,
//     source: &'a N,
//     depth_limit: Option<usize>,
// ) -> Result<Vec<(&'a N, &'a N)>, String>
// where
//     N: Eq + Hash + Clone,
// {
//     let limit = depth_limit.unwrap_or(g.node.len());

//     let mut bfs_path: Vec<(&'a N, &'a N)> = Vec::new();
//     let mut seen: HashSet<&'a N> = HashSet::new();
//     let mut queue: VecDeque<(&'a N, usize)> = VecDeque::new();

//     seen.insert(source);
//     queue.push_back((source, 0));

//     while let Some((parent, depth)) = queue.pop_front() {
//         if depth >= limit {
//             continue;
//         }

//         let nbrs = g
//             .adj_outer
//             .get(parent)
//             .ok_or_else(|| "bfs_edges_ref error: parent not in graph".to_string())?;

//         for child in nbrs.keys() {
//             if seen.insert(child) {
//                 bfs_path.push((parent, child));
//                 queue.push_back((child, depth + 1));
//             }
//         }
//     }

//     Ok(bfs_path)
// }

// pub fn bfs_edges_v3<'a, N>(
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

//     let mut node_to_idx: HashMap<&'a N, usize> = HashMap::with_capacity(g.node.len());
//     for (i, node) in g.node.keys().enumerate() {
//         node_to_idx.insert(node, i);
//     }

//     let mut seen = vec![false; g.node.len()];
//     let mut bfs_path: Vec<(&'a N, &'a N)> =
//         Vec::with_capacity(g.node.len().saturating_sub(1));
//     let mut queue: VecDeque<(&'a N, usize)> = VecDeque::with_capacity(g.adj_idx.len());

//     let source_idx = node_to_idx[source];
//     seen[source_idx] = true;
//     queue.push_back((source, 0));

//     while let Some((parent, depth)) = queue.pop_front() {
//         if depth >= limit {
//             continue;
//         }

//         if let Some(children) = g.neighbors.get(parent) {
//             for child in children {
//                 let child_idx = node_to_idx[child];
//                 if !seen[child_idx] {
//                     seen[child_idx] = true;
//                     bfs_path.push((parent, child));
//                     queue.push_back((child, depth + 1));
//                 }
//             }
//         }
//     }

//     Ok(bfs_path)
// }

// pub fn bfs_edges_v4<'a, N>(
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

//     let mut node_to_idx: HashMap<&'a N, usize> = HashMap::with_capacity(g.node.len());
//     for (i, node) in g.node.keys().enumerate() {
//         node_to_idx.insert(node, i);
//     }

//     let mut seen = vec![false; g.node.len()];
//     let mut bfs_path: Vec<(&'a N, &'a N)> =
//         Vec::with_capacity(g.node.len().saturating_sub(1));
//     let mut queue: VecDeque<(&'a N, usize)> = VecDeque::with_capacity(g.adj_idx.len());

//     let source_idx = node_to_idx[source];
//     seen[source_idx] = true;
//     queue.push_back((source, 0));

//     while let Some((parent, depth)) = queue.pop_front() {
//         if depth >= limit {
//             continue;
//         }

//         if let Some(children) = g.neighbors.get(parent) {
//             for child in children {
//                 let child_idx = node_to_idx[child];
//                 if !seen[child_idx] {
//                     seen[child_idx] = true;
//                     bfs_path.push((parent, child));
//                     queue.push_back((child, depth + 1));
//                 }
//             }
//         }
//     }

//     Ok(bfs_path)
// }