use crate::Graph;

use crate::traversal::dfs::{dfs_edges, dfs_edges_ref};
use crate::traversal::dfs_v2::dfs_edges_v2;
use crate::traversal::bfs::{bfs_edges, bfs_edges_ref};
use crate::traversal::bfs_v2::bfs_edges_v2;
use crate::shortest_path::mst::prim_mst_edges;
use crate::shortest_path::mst_ref::{prim_mst_edges_ref, prim_mst_edges_v2};

use std::fmt::Debug;
use std::hash::Hash;

pub fn print_all<'a, N>(g: &'a Graph<N>, source: &'a N)
where
    N: Eq + Hash + Clone + Ord + Debug,
{
    let dfs = dfs_edges(&g, Some(source.clone()), None);    
    println!("DFS (clone) edges: {:?}", dfs);

    let dfs = dfs_edges_ref(&g, source, None);
    println!("DFS (ref) edges: {:?}", dfs);

    let dfs = dfs_edges_v2(&g, source, None);
    println!("DFS (vectors) edges: {:?}", dfs);

    let bfs = bfs_edges(&g, Some(source.clone()), None);
    println!("BFS (clone) edges: {:?}", bfs);

    let bfs = bfs_edges_ref(&g, source, None);
    println!("BFS (ref) edges: {:?}", bfs);

    let bfs = bfs_edges_v2(&g, source, None);
    println!("BFS (vec) edges: {:?}", bfs);

    let prim = prim_mst_edges(&g, "weight", false).unwrap();
    println!("Prim (clone) MST edges: {:?}", prim);

    let prim = prim_mst_edges_ref(&g, "weight", false).unwrap();
    println!("Prim (ref) MST edges: {:?}", prim);

    let prim = prim_mst_edges_v2(&g, "weight", false).unwrap();
    println!("Prim (vec) MST edges: {:?}", prim);
}