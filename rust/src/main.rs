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

// use traversal::dfs::dfs_edges;
// use traversal::dfs::dfs_edges_ref;
use traversal::dfs_v2::dfs_edges_v2;
use traversal::dfs_v2::dfs_edges_digraph;

// use traversal::bfs::bfs_edges;
// use traversal::bfs::bfs_edges_ref;
use traversal::bfs_v2::bfs_edges_v2;
use traversal::bfs_v2::bfs_edges_digraph;

// use shortest_path::mst::prim_mst_edges;
// use shortest_path::mst_ref::prim_mst_edges_ref;
use shortest_path::mst_ref::prim_mst_edges_v2;
use shortest_path::dijkstra::dijkstra_path;
use shortest_path::dijkstra::dijkstra_path_digraph;
// use shortest_path::floyd::floyd_warshall;
use shortest_path::floyd::floyd_warshall_v3;
use shortest_path::floyd::floyd_warshall_digraph;

use utils::print_all::print_all;
#[allow(unused)]
fn graph_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("graphs")
        .join(filename)
}
#[allow(unused)]
fn run_all_algorithms(graph_name: &str, g: &crate::graph::nx_graph::Graph<String>) {
    println!("\n==============================");
    println!("Running algorithms on {graph_name}");
    println!("==============================");

    let source = "0".to_string();
    let dst = "500".to_string();

    let start = Instant::now();
    let _ = dfs_edges_v2(g, &source, None);
    println!("DFS vectors -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = bfs_edges_v2(g, &source, None);
    println!("BFS Vector -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = prim_mst_edges_v2(g, "weight", false);
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
    let _ = floyd_warshall_v3(g, "weight").unwrap();
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

#[test]
fn test_floyd_warshall_v3_sample() {
    let mut g = Graph::<&str>::new(vec![]);

    g.add_node("A", vec![]);
    g.add_node("B", vec![]);
    g.add_node("C", vec![]);
    g.add_node("D", vec![]);

    g.add_edge("A", "B", [("weight".to_string(), AttrValue::Int(3))]);
    g.add_edge("A", "C", [("weight".to_string(), AttrValue::Int(10))]);
    g.add_edge("B", "C", [("weight".to_string(), AttrValue::Int(1))]);
    g.add_edge("B", "D", [("weight".to_string(), AttrValue::Int(2))]);
    g.add_edge("C", "D", [("weight".to_string(), AttrValue::Int(4))]);

    let (pred, dist, nodes) = floyd_warshall_v3(&g, "weight").unwrap();

    println!("nodes = {:?}", nodes);
    println!("dist = {:?}", dist);
    println!("pred = {:?}", pred);

    // Since HashMap order is not stable, map node names to indices first
    let mut idx = std::collections::HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        idx.insert(**node, i);
    }

    let a = idx["A"];
    let b = idx["B"];
    let c = idx["C"];
    let d = idx["D"];

    // Expected shortest distances
    assert_eq!(dist[a][a], 0.0);
    assert_eq!(dist[a][b], 3.0);
    assert_eq!(dist[a][c], 4.0); // A -> B -> C
    assert_eq!(dist[a][d], 5.0); // A -> B -> D

    assert_eq!(dist[b][a], 3.0);
    assert_eq!(dist[b][b], 0.0);
    assert_eq!(dist[b][c], 1.0);
    assert_eq!(dist[b][d], 2.0);

    assert_eq!(dist[c][a], 4.0); // C -> B -> A
    assert_eq!(dist[c][b], 1.0);
    assert_eq!(dist[c][c], 0.0);
    assert_eq!(dist[c][d], 3.0); // C -> B -> D

    assert_eq!(dist[d][a], 5.0); // D -> B -> A
    assert_eq!(dist[d][b], 2.0);
    assert_eq!(dist[d][c], 3.0); // D -> B -> C
    assert_eq!(dist[d][d], 0.0);

    // Optional predecessor checks
    assert_eq!(pred[a][b], Some(a));
    assert_eq!(pred[a][c], Some(b));
    assert_eq!(pred[a][d], Some(b));

    assert_eq!(pred[c][a], Some(b));
    assert_eq!(pred[c][d], Some(b));
}