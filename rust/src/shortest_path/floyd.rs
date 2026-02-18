use std::collections::HashMap;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

pub fn floyd_warshall<N>(
    g: &Graph<N>, 
    weight_key: &str,
) -> Result<(HashMap<N, HashMap<N, N>>, HashMap<N, HashMap<N, f64>>), String>
where
    N: Eq + Hash + Clone
{
    let node: Vec<N> = g.node.keys().cloned().collect();

    let mut dist: HashMap<N, HashMap<N, f64>> = HashMap::new();
    for u in &node {
        let mut row: HashMap<N, f64> = HashMap::new();
        for v in & node {
            row.insert(v.clone(), f64::INFINITY);
        }
        dist.insert(u.clone(), row);
    }

    for v in &node {
        if let Some(row) = dist.get_mut(v) {
            row.insert(v.clone(), 0.0);
        }
    }

    let mut pred: HashMap<N, HashMap<N, N>> = HashMap::new();
    for u in &node {
        pred.insert(u.clone(), HashMap::new());
    }

    let undirected = true;
    for (u, v, opt_attr) in g.edges(true) {
        let attr = opt_attr.ok_or("edges(true) returned None")?;
        let edge_weight = g.get_weight(&attr, weight_key).unwrap_or(1.0);

        let current = *dist[&u].get(&v).unwrap_or(&f64::INFINITY);
        if edge_weight < current {
            dist.get_mut(&u).unwrap().insert(v.clone(), edge_weight);
        }
        pred.get_mut(&u).unwrap().insert(v.clone(), u.clone());

        if undirected {
            let current_rev = *dist[&v].get(&u).unwrap_or(&f64::INFINITY);

            if edge_weight < current_rev {
                dist.get_mut(&v).unwrap().insert(u.clone(), edge_weight);
            }

            pred.get_mut(&v).unwrap().insert(u.clone(), v.clone());
        }
    }

    for w in &node {
        for u in &node {
            for v in &node {
                let duw = *dist[u].get(w).unwrap_or(&f64::INFINITY);
                let duv = *dist[w].get(v).unwrap_or(&f64::INFINITY);
                let d = duv + duw;

                let duv = *dist[u].get(v).unwrap_or(&f64::INFINITY);
                if duv > d {
                    dist.get_mut(u).unwrap().insert(v.clone(), d);
                if let Some(pwv) = pred[w].get(v).cloned() {
                        pred.get_mut(u).unwrap().insert(v.clone(), pwv);
                    }
                }
            }
        }
    }
    Ok((pred, dist))
}