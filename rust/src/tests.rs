use crate::utils::AttrValue;
use crate::graph::nx_graph::Graph;

use crate::traversal::bfs::bfs_edges;
use crate::traversal::dfs::dfs_edges;

use crate::shortest_path::dijkstra::dijkstra_path;
use crate::shortest_path::mst::prim_mst_edges;
use crate::shortest_path::floyd::floyd_warshall;

#[test]
fn test_bfs_sample() {
    let mut g = Graph::<&str>::new(vec![]);

    g.add_node("A", vec![]);
    g.add_node("B", vec![]);
    g.add_node("C", vec![]);
    g.add_node("D", vec![]);

    g.add_edge("A", "B", [("weight".to_string(), AttrValue::Int(1))]);
    g.add_edge("A", "C", [("weight".to_string(), AttrValue::Int(1))]);
    g.add_edge("B", "D", [("weight".to_string(), AttrValue::Int(1))]);

    let start = "A";
    let edges = bfs_edges(&g, &start, None).unwrap();

    println!("bfs edges = {:?}", edges);

    let expected = vec![(&"A", &"B"), (&"A", &"C"), (&"B", &"D")];

    assert_eq!(edges.len(), expected.len());
    for e in expected {
        assert!(edges.contains(&e), "missing edge {:?}", e);
    }
}

#[test]
fn test_dfs_sample() {
    let mut g = Graph::<&str>::new(vec![]);

    g.add_node("A", vec![]);
    g.add_node("B", vec![]);
    g.add_node("C", vec![]);
    g.add_node("D", vec![]);

    g.add_edge("A", "B", [("weight".to_string(), AttrValue::Int(1))]);
    g.add_edge("A", "C", [("weight".to_string(), AttrValue::Int(1))]);
    g.add_edge("B", "D", [("weight".to_string(), AttrValue::Int(1))]);

    let start = "A";
    let edges = dfs_edges(&g, &start, None).unwrap();

    println!("dfs edges = {:?}", edges);

    let expected = vec![(&"A", &"B"), (&"B", &"D"), (&"A", &"C")];

    assert_eq!(edges.len(), expected.len());
    assert_eq!(edges, expected);
}

#[test]
fn test_dijkstra_sample() {
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

    let start = "A";
    let goal = "D";

    let (cost, path) = dijkstra_path(&g, &start, &goal).unwrap().unwrap();

    println!("dijkstra cost = {:?}", cost);
    println!("dijkstra path = {:?}", path);

    assert_eq!(cost, 5.0);
    assert_eq!(path, vec![&"A", &"B", &"D"]);
}

#[test]
fn test_prims_sample() {
    fn undirected_edge_eq(a: (&&str, &&str), b: (&&str, &&str)) -> bool {
        (a.0 == b.0 && a.1 == b.1) || (a.0 == b.1 && a.1 == b.0)
    }

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

    let mst_edges = prim_mst_edges(&g, "weight", false).unwrap();

    println!("mst_edges = {:?}", mst_edges);

    let expected = vec![(&"A", &"B"), (&"B", &"C"), (&"B", &"D")];

    assert_eq!(mst_edges.len(), expected.len());

    for e in expected {
        assert!(
            mst_edges.iter().any(|x| undirected_edge_eq(*x, e)),
            "missing MST edge {:?}",
            e
        );
    }
}

#[test]
fn test_floyd_warshall_sample() {
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

    let (pred, dist, nodes) = floyd_warshall(&g, "weight").unwrap();

    println!("nodes = {:?}", nodes);
    println!("dist = {:?}", dist);
    println!("pred = {:?}", pred);

    let mut idx = std::collections::HashMap::new();
    for (i, node) in nodes.iter().enumerate() {
        idx.insert(**node, i);
    }

    let a = idx["A"];
    let b = idx["B"];
    let c = idx["C"];
    let d = idx["D"];

    assert_eq!(dist[a][a], 0.0);
    assert_eq!(dist[a][b], 3.0);
    assert_eq!(dist[a][c], 4.0);
    assert_eq!(dist[a][d], 5.0);

    assert_eq!(dist[b][a], 3.0);
    assert_eq!(dist[b][b], 0.0);
    assert_eq!(dist[b][c], 1.0);
    assert_eq!(dist[b][d], 2.0);

    assert_eq!(dist[c][a], 4.0);
    assert_eq!(dist[c][b], 1.0);
    assert_eq!(dist[c][c], 0.0);
    assert_eq!(dist[c][d], 3.0);

    assert_eq!(dist[d][a], 5.0);
    assert_eq!(dist[d][b], 2.0);
    assert_eq!(dist[d][c], 3.0);
    assert_eq!(dist[d][d], 0.0);

    assert_eq!(pred[a][b], Some(a));
    assert_eq!(pred[a][c], Some(b));
    assert_eq!(pred[a][d], Some(b));

    assert_eq!(pred[c][a], Some(b));
    assert_eq!(pred[c][d], Some(b));
}
