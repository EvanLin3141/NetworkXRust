use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;

use crate::graph::nx_graph::Graph;
use crate::utils::AttrValue;

use crate::graph::nx_digraph::DiGraph;

pub fn load_digraph_from_file(path: &Path) -> std::io::Result<DiGraph<String>> {
    println!("load_digraph_from_file path = {:?}", path);

    let file = File::open(path)?;
    let reader = BufReader::new(file);

    let mut g = DiGraph::<String>::new([
        (
            "graph".to_string(),
            AttrValue::Text("nxDiGraph".to_string()),
        )
    ]);

    for line_result in reader.lines() {
        let line = line_result?;
        let line = line.trim();

        if line.is_empty() {
            continue;
        }

        let parts: Vec<&str> = line.split_whitespace().collect();

        if parts.len() < 2 {
            continue;
        }

        let u = parts[0].to_string();
        let v = parts[1].to_string();

        let weight = if parts.len() >= 3 {
            parts[2].parse::<i64>().unwrap_or(1)
        } else {
            1
        };

        if !g.node.contains_key(&u) {
            g.add_node(u.clone(), []);
        }

        if !g.node.contains_key(&v) {
            g.add_node(v.clone(), []);
        }

        g.add_edge(
            u,
            v,
            [
                ("weight".to_string(), AttrValue::Int(weight)),
            ],
        );
    }

    Ok(g)
}