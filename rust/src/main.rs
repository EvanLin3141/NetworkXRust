mod graph;
mod traversal;
mod shortest_path;
mod utils;
mod graph_loader;

use std::path::PathBuf;
use std::time::Instant;

use graph_loader::load_graph_from_file;

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
use shortest_path::floyd::floyd_warshall;

use utils::print_all::print_all;

fn graph_path(filename: &str) -> PathBuf {
    PathBuf::from(env!("CARGO_MANIFEST_DIR"))
        .join("graphs")
        .join(filename)
}

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
    let _ = floyd_warshall(g, "weight").unwrap();
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

    run_all_algorithms("Sparse Graph", &sparse);
    run_all_algorithms("Medium Graph", &medium);
    run_all_algorithms("Dense Graph", &dense);

    let mut h = DiGraph::<String>::new([(
        "digraph".to_string(),
        AttrValue::Text("SampleABCGraph".to_string()),
    )]);

    h.add_node(
        "A".to_string(),
        [("type".to_string(), AttrValue::Text("start".to_string()))],
    );

    h.add_node(
        "B".to_string(),
        [("level".to_string(), AttrValue::Int(1))],
    );

    h.add_node(
        "C".to_string(),
        [("level".to_string(), AttrValue::Int(2))],
    );

    h.add_node(
        "D".to_string(),
        [("active".to_string(), AttrValue::Bool(true))],
    );

    h.add_node(
        "E".to_string(),
        [("priority".to_string(), AttrValue::Float(1.5))],
    );

    h.add_node(
        "F".to_string(),
        [("end".to_string(), AttrValue::Bool(true))],
    );

    h.add_edge(
        "A".to_string(),
        "B".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(4)),
            ("relation".to_string(), AttrValue::Text("path1".to_string())),
        ],
    );

    h.add_edge(
        "A".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(2)),
            ("relation".to_string(), AttrValue::Text("shortcut".to_string())),
        ],
    );

    h.add_edge(
        "B".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(7)),
            ("enabled".to_string(), AttrValue::Bool(true)),
        ],
    );

    h.add_edge(
        "C".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(3)),
            ("enabled".to_string(), AttrValue::Bool(true)),
        ],
    );

    h.add_edge(
        "D".to_string(),
        "E".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(6)),
            ("cost".to_string(), AttrValue::Float(2.3)),
        ],
    );

    h.add_edge(
        "E".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(1)),
            ("relation".to_string(), AttrValue::Text("final".to_string())),
        ],
    );

    h.add_edge(
        "C".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(9)),
            ("relation".to_string(), AttrValue::Text("long_route".to_string())),
        ],
    );

    h.add_edge(
        "F".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(5)),
            ("relation".to_string(), AttrValue::Text("feedback".to_string())),
        ],
    );

    println!("Starting Digraph testing");

    let source: &String = match h.node.get_key_value("A") {
        Some((k, _)) => k,
        None => {
            eprintln!("source not found");
            return;
        }
    };

    let start = Instant::now();
    let _ = dfs_edges_digraph(&h, source, None);
    println!("Digraph DFS -> {:?}", start.elapsed());

    let start = Instant::now();
    let _ = bfs_edges_digraph(&h, source, None);
    println!("Digraph BFS -> {:?}", start.elapsed());
}