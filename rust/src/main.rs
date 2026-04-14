#![allow(dead_code, unused_imports)]
mod graph;
mod traversal;
mod shortest_path;
mod utils;
mod graph_loader;
mod digraph_loader;



use std::path::PathBuf;
use std::time::Instant;

use graph_loader::load_graph_from_file;
use crate::digraph_loader::load_digraph_from_file;

use crate::utils::AttrValue;
use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

use traversal::dfs::dfs_edges;
use traversal::dfs_nx::dfs_edges_nx;
use traversal::dfs::dfs_edges_digraph;

use traversal::bfs::bfs_edges;
use traversal::bfs_nx::bfs_edges_nx;
use traversal::bfs::bfs_edges_digraph;

use shortest_path::mst::prim_mst_edges;
use shortest_path::mst_nx::prim_mst_edges_nx;

use shortest_path::dijkstra::dijkstra_path;
use shortest_path::dijkstra::dijkstra_path_digraph;

use shortest_path::floyd::floyd_warshall;
use shortest_path::floyd::floyd_warshall_nx;
use shortest_path::floyd::floyd_warshall_digraph;

#[cfg(test)]
mod tests;

#[allow(unused)]
fn graph_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("graphs")
        .join(filename)
}

#[allow(unused)]
fn run_all_algorithms_nx(graph_name: &str, g: &crate::graph::nx_graph::Graph<String>) {
    println!("\n==============================");
    println!("Running algorithms on NX Architecture with {graph_name}");
    println!("==============================");

    let source = "0".to_string();
    let dst = "500".to_string();

    let start = Instant::now();
    let _ = dfs_edges_nx(g, Some(source.clone()), None);
    println!("DFS vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = bfs_edges_nx(g, Some(source.clone()), None);
    println!("BFS Vector -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = prim_mst_edges_nx(g, "weight", false);
    println!("Prims w/ Vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = floyd_warshall_nx(g, "weight").unwrap();
    println!("Floyd-Warshall -> {:?}", start.elapsed());
}

#[allow(unused)]
fn run_all_algorithms(graph_name: &str, g: &crate::graph::nx_graph::Graph<String>) {
    println!("\n==============================");
    println!("Running algorithms on {graph_name}");
    println!("==============================");

    let source = "0".to_string();
    let dst = "500".to_string();

    let start = Instant::now();
    let _ = dfs_edges(g, &source, None);
    println!("DFS vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = bfs_edges(g, &source, None);
    println!("BFS Vector -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = prim_mst_edges(g, "weight", false);
    println!("Prims w/ Vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    match dijkstra_path(g, &source, &dst) {
        Ok(Some((cost, path))) => {
            println!("Shortest cost: {}", cost);

            let preview: Vec<_> = path.iter().take(20).collect();
            println!("Path (first 20): {:?}", preview);
        }
        Ok(None) => println!("No path found."),
        Err(e) => println!("Error: {}", e),
    }
    println!("Dijkstra -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = floyd_warshall(g, "weight").unwrap();
    println!("Floyd-Warshall -> {:?}", start.elapsed());
}
#[allow(unused)]
fn run_all_algorithms_directed(
    graph_name: &str,
    g: &crate::graph::nx_digraph::DiGraph<String>,
) {
    println!("\n==============================");
    println!("Running directed algorithms on {graph_name}");
    println!("==============================");

    let source = "0".to_string();
    let dst = "500".to_string();

    let start = Instant::now();
    let _ = dfs_edges_digraph(g, &source, None);
    println!("DFS vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = bfs_edges_digraph(g, &source, None);
    println!("BFS Vector -> {:?}", start.elapsed());

    let start = Instant::now();
    match dijkstra_path_digraph(g, &source, &dst) {
        Ok(Some((cost, path))) => {
            println!("Shortest cost: {}", cost);

            let preview: Vec<_> = path.iter().take(20).collect();
            println!("Path (first 20): {:?}", preview);
        }
        Ok(None) => println!("No path found."),
        Err(e) => println!("Error: {}", e),
    }
    println!("Dijkstra -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = floyd_warshall_digraph(g, "weight").unwrap();
    println!("Floyd-Warshall -> {:?}", start.elapsed());
}
fn main() {
    println!("Graph Loading");

    let sparse = load_graph_from_file(&graph_path("sparse_graph.txt"))
        .expect("failed to load sparse graph");

    let medium = load_graph_from_file(&graph_path("medium_graph.txt"))
        .expect("failed to load medium graph");

    let dense = load_graph_from_file(&graph_path("dense_graph.txt"))
        .expect("failed to load dense graph");

    println!("Graphs loaded.");

    println!("Directed Graph Loading");

    run_all_algorithms_nx("Sparse Graph", &sparse);
    run_all_algorithms_nx("Medium Graph", &medium);
    run_all_algorithms_nx("Dense Graph", &dense); 

    run_all_algorithms("Sparse Graph", &sparse);
    run_all_algorithms("Medium Graph", &medium);
    run_all_algorithms("Dense Graph", &dense);

    let d_sparse = load_digraph_from_file(&graph_path("directed_sparse_graph.txt"))
        .expect("failed to load sparse graph");

    let d_medium = load_digraph_from_file(&graph_path("directed_medium_graph.txt"))
        .expect("failed to load medium graph");

    let d_dense = load_digraph_from_file(&graph_path("directed_dense_graph.txt"))
        .expect("failed to load dense graph");

    println!("Graphs loaded.");

    run_all_algorithms_directed("Directed Sparse Graph", &d_sparse);
    run_all_algorithms_directed("Directed Medium Graph", &d_medium);
    run_all_algorithms_directed("Directed Dense Graph", &d_dense);
}

