mod graph;
mod traversal;

use crate::graph::nx_graph::{AttrValue, Graph};

use traversal::dfs::dfs_edges;

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
    let edges = dfs_edges(&g, Some("A".to_string()), None);

    println!("DFS edges: {:?}", edges);

}
