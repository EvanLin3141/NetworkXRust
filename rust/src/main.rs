mod graph;
mod traversal;
mod shortest_path;

use std::time::Instant;
use crate::graph::nx_graph::{AttrValue, Graph};

use traversal::dfs::dfs_edges;
use traversal::dfs::dfs_edges_ref;
use traversal::bfs::bfs_edges;
use traversal::bfs::bfs_edges_ref;
use shortest_path::mst::prim_mst_edges;
use shortest_path::dijkstra::dijkstra_path;
use shortest_path::floyd::floyd_warshall;


fn main() {
    let mut g = Graph::<String>::new([
        (
            "name".to_string(),
            AttrValue::Text("nxGraph".to_string()),
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

    println!("Analysing each algorithm in NX structure with 100,000 iterations");
    let start = Instant::now();
    for _ in 0..100000 {
        dfs_edges(&g, Some("A".to_string()), None);
    }
    println!("DFS with Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let source: &String = match g.node.get_key_value("A") {
        Some((k, _)) => k,
        None => {
            eprintln!("source not found");
            return;
        }
    };

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = dfs_edges_ref(&g, source, None);
    }
    println!("DFS No Cloning -> Avg: {:?}", start.elapsed() / 100000);
    
    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges(&g, Some("A".to_string()), None);
    }
    println!("BFS with Cloning -> Avg: {:?}", start.elapsed() / 100000);

    let source: &String = match g.node.get_key_value("A") {
        Some((k, _)) => k,
        None => {
            eprintln!("source not found");
            return;
        }
    };

    let start = Instant::now();
    for _ in 0..100000 {
        let _ = bfs_edges_ref(&g, source, None);
    }
    println!("BFS No Cloning -> Avg: {:?}", start.elapsed() / 100000);

    for _ in 0..100000 {
        let _ = prim_mst_edges(&g, "weight", false);
    }

    println!("Avg: {:?}", start.elapsed() / 100000);

    
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

}
