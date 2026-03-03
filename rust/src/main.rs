mod graph;
mod traversal;
mod shortest_path;
mod utils;

use std::time::Instant;

use crate::utils::AttrValue;
use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;

use traversal::dfs::dfs_edges;
use traversal::dfs::dfs_edges_ref;
use traversal::dfs_v2::dfs_edges_v2;
use traversal::dfs_v2::dfs_edges_digraph;

use traversal::bfs::bfs_edges;
use traversal::bfs::bfs_edges_ref;
use traversal::bfs_v2::bfs_edges_v2;
use traversal::bfs_v2::bfs_edges_digraph;

use shortest_path::mst::prim_mst_edges;
use shortest_path::mst_ref::prim_mst_edges_ref;
use shortest_path::mst_ref::prim_mst_edges_v2;
use shortest_path::dijkstra::dijkstra_path;
use shortest_path::floyd::floyd_warshall;

use utils::print_all::print_all;


fn main() {
    let mut g = Graph::<String>::new([
        (
            "graph".to_string(),
            AttrValue::Text("nxGraph".to_string()),
        )
    ]);

    let mut h = Graph::<String>::new([
        (
            "digraph".to_string(),
            AttrValue::Text("nxDiGraph".to_string()),
        )
    ]);
    g.add_node("A".to_string(), [("color".to_string(), AttrValue::Text("amber".to_string()))]);
    g.add_node("B".to_string(), [("color".to_string(), AttrValue::Bool(true))]);

    g.add_edge(
        "A".to_string(),
        "B".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(5)),
            ("relation".to_string(), AttrValue::Text("friend".to_string())),
        ],
    );

    g.add_edge(
        "A".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(2)),
            ("relation".to_string(), AttrValue::Text("colleague".to_string())),
        ],
    );

    g.add_edge(
        "B".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(7)),
            ("cost".to_string(), AttrValue::Float(1.5)),
        ],
    );

    g.add_edge(
        "B".to_string(),
        "E".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(1)),
            ("enabled".to_string(), AttrValue::Bool(false)),
        ],
    );

    g.add_edge(
        "C".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(9)),
            ("relation".to_string(), AttrValue::Text("family".to_string())),
        ],
    );

    g.add_edge(
        "E".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(3)),
            ("relation".to_string(), AttrValue::Text("connected".to_string())),
        ],
    );

    g.add_node("A".to_string(), [("color".to_string(), AttrValue::Text("amber".to_string()))]);
    g.add_node("B".to_string(), [("color".to_string(), AttrValue::Bool(true))]);

    g.add_edge(
        "A".to_string(),
        "B".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(5)),
            ("relation".to_string(), AttrValue::Text("friend".to_string())),
        ],
    );

    g.add_edge(
        "A".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(2)),
            ("relation".to_string(), AttrValue::Text("colleague".to_string())),
        ],
    );

    g.add_edge(
        "B".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(7)),
            ("cost".to_string(), AttrValue::Float(1.5)),
        ],
    );

    g.add_edge(
        "B".to_string(),
        "E".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(1)),
            ("enabled".to_string(), AttrValue::Bool(false)),
        ],
    );

    g.add_edge(
        "C".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(9)),
            ("relation".to_string(), AttrValue::Text("family".to_string())),
        ],
    );

    g.add_edge(
        "E".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(3)),
            ("relation".to_string(), AttrValue::Text("connected".to_string())),
        ],
    );

    let source: &String = match g.node.get_key_value("A") {
        Some((k, _)) => k,
        None => {
            eprintln!("source not found");
            return;
        }
    };

    println!("Analysing each algorithm in NX structure with 100,000 iterations");

    // <-- Depth first search --> //
    let start = Instant::now();
    for _ in 0..100000 {
        dfs_edges(&g, Some("A".to_string()), None);
    }
    println!("DFS with Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = dfs_edges_ref(&g, source, None);
    }
    println!("DFS No Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = dfs_edges_v2(&g, source, None);
    }
    println!("DFS vectors -> Avg: {:?}", start.elapsed() / 100000);
    // <--- END OF DFS --->

    // <-- Breath First Search --> //
    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges(&g, Some("A".to_string()), None);
    }
    println!("BFS with Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges_ref(&g, source, None);
    }
    println!("BFS No Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges_v2(&g, source, None);
    }
    println!("BFS Vector -> Avg: {:?}", start.elapsed() / 100000);

    // Prims Algorithm
    let start = Instant::now();
    for _ in 0..100000 {
        let _ = prim_mst_edges(&g, "weight", false);
    }
    println!("Prims w/ Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = prim_mst_edges_ref(&g, "weight", false);
    }
    println!("Prims w/o Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = prim_mst_edges_v2(&g, "weight", false);
    }
    println!("Prims w/ Vectors -> Avg: {:?}", start.elapsed() / 100000);
    
    let source = "A".to_string();
    let dst  = "D".to_string();

    match dijkstra_path(&g, &source, &dst) {
        Ok(Some((cost, path))) => {
            println!("Shortest cost: {}", cost);
            println!("Path: {:?}", path);
        }
        Ok(None) => println!("No path found."),
        Err(e) => println!("Error: {}", e),
    }

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = dijkstra_path(&g, &source, &dst);
    }
    println!("Avg: {:?}", start.elapsed() / 100000);

    // let (_, dist) = floyd_warshall(&g, "weight").unwrap();
    // println!("Distances:");
    // for (u, row) in &dist {
    //     for (v, d) in row {
    //         println!("{} -> {} = {}", u, v, d);
    //     }
    // }

    // 10 times less data
    let start = Instant::now();
    for _ in 0..10000 {
        let (_, _) = floyd_warshall(&g, "weight").unwrap();
    }
    println!("Avg: {:?}, with 10 times less data", start.elapsed() / 100000);

    //print_all(&g, &source);

    let mut h = DiGraph::<String>::new([
        (
            "digraph".to_string(),
            AttrValue::Text("SampleABCGraph".to_string()),
        )
    ]);

    // ---- Nodes ----
    h.add_node("A".to_string(), [
        ("type".to_string(), AttrValue::Text("start".to_string()))
    ]);

    h.add_node("B".to_string(), [
        ("level".to_string(), AttrValue::Int(1))
    ]);

    h.add_node("C".to_string(), [
        ("level".to_string(), AttrValue::Int(2))
    ]);

    h.add_node("D".to_string(), [
        ("active".to_string(), AttrValue::Bool(true))
    ]);

    h.add_node("E".to_string(), [
        ("priority".to_string(), AttrValue::Float(1.5))
    ]);

    h.add_node("F".to_string(), [
        ("end".to_string(), AttrValue::Bool(true))
    ]);

// ---- Directed Edges ----

    // A -> B
    h.add_edge(
        "A".to_string(),
        "B".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(4)),
            ("relation".to_string(), AttrValue::Text("path1".to_string())),
        ],
    );

    // A -> C
    h.add_edge(
        "A".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(2)),
            ("relation".to_string(), AttrValue::Text("shortcut".to_string())),
        ],
    );

    // B -> D
    h.add_edge(
        "B".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(7)),
            ("enabled".to_string(), AttrValue::Bool(true)),
        ],
    );

    // C -> D
    h.add_edge(
        "C".to_string(),
        "D".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(3)),
            ("enabled".to_string(), AttrValue::Bool(true)),
        ],
    );

    // D -> E
    h.add_edge(
        "D".to_string(),
        "E".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(6)),
            ("cost".to_string(), AttrValue::Float(2.3)),
        ],
    );

    // E -> F
    h.add_edge(
        "E".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(1)),
            ("relation".to_string(), AttrValue::Text("final".to_string())),
        ],
    );

    // C -> F (alternate path)
    h.add_edge(
        "C".to_string(),
        "F".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(9)),
            ("relation".to_string(), AttrValue::Text("long_route".to_string())),
        ],
    );

    // F -> C (creates a directed cycle)
    h.add_edge(
        "F".to_string(),
        "C".to_string(),
        [
            ("weight".to_string(), AttrValue::Int(5)),
            ("relation".to_string(), AttrValue::Text("feedback".to_string())),
        ],
    );    

// Test
    println!("Starting Digraph testing");
    let source: &String = match g.node.get_key_value("A") {
        Some((k, _)) => k,
        None => {
            eprintln!("source not found");
            return;
        }
    };

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = dfs_edges_digraph(&h, source, None);
    }
    println!("DFS vectors -> Avg: {:?}", start.elapsed() / 100000);

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges_digraph(&h, source, None);
    }
    println!("BFS vectors -> Avg: {:?}", start.elapsed() / 100000);
}
