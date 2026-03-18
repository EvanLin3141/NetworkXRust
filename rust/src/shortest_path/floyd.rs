use std::collections::HashMap;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;
use crate::graph::nx_digraph::DiGraph;
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

pub fn floyd_warshall_v2<'a, N>(
    g: &'a Graph<N>,
    weight_key: &str,
) -> Result<
    (
        HashMap<&'a N, HashMap<&'a N, &'a N>>,
        HashMap<&'a N, HashMap<&'a N, f64>>,
    ),
    String,
>
where
    N: Eq + Hash + Clone,
{
    let nodes: Vec<&'a N> = g.node.keys().collect();

    let mut dist: HashMap<&'a N, HashMap<&'a N, f64>> = HashMap::new();
    for &u in &nodes {
        let mut row = HashMap::new();
        for &v in &nodes {
            row.insert(v, f64::INFINITY);
        }
        dist.insert(u, row);
    }

    for &v in &nodes {
        if let Some(row) = dist.get_mut(v) {
            row.insert(v, 0.0);
        }
    }

    let mut pred: HashMap<&'a N, HashMap<&'a N, &'a N>> = HashMap::new();
    for &u in &nodes {
        pred.insert(u, HashMap::new());
    }

    for (u, neighbors) in &g.adj_outer {
        for (v, attr) in neighbors {
            let edge_weight = g.get_weight(attr, weight_key).unwrap_or(1.0);

            let current = *dist[u].get(v).unwrap_or(&f64::INFINITY);
            if edge_weight < current {
                dist.get_mut(u).unwrap().insert(v, edge_weight);
            }

            pred.get_mut(u).unwrap().insert(v, u);
        }
    }

    for &w in &nodes {
        for &u in &nodes {
            for &v in &nodes {
                let duw = *dist[u].get(w).unwrap_or(&f64::INFINITY);
                let dwv = *dist[w].get(v).unwrap_or(&f64::INFINITY);
                let d = duw + dwv;

                let duv = *dist[u].get(v).unwrap_or(&f64::INFINITY);
                if d < duv {
                    dist.get_mut(u).unwrap().insert(v, d);

                    if let Some(&pwv) = pred[w].get(v) {
                        pred.get_mut(u).unwrap().insert(v, pwv);
                    }
                }
            }
        }
    }

    Ok((pred, dist))
}

pub fn floyd_warshall_digraph<'a, N>(
    g: &'a DiGraph<N>,
    weight_key: &str,
) -> Result<(Vec<Vec<Option<usize>>>, Vec<Vec<f64>>, Vec<&'a N>), String>
where
    N: Eq + Hash + Clone,
{
    let nodes: Vec<&'a N> = g.node.keys().collect();
    let n = nodes.len();

    let mut index: HashMap<&'a N, usize> = HashMap::with_capacity(n);
    for (i, node) in nodes.iter().enumerate() {
        index.insert(*node, i);
    }

    let mut dist: Vec<Vec<f64>> = vec![vec![f64::INFINITY; n]; n];
    let mut pred: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];

    for i in 0..n {
        dist[i][i] = 0.0;
    }

    for (u, neighbors) in &g.adj_outer {
        let i = index[u];

        for (v, attr) in neighbors {
            let j = index[v];
            let w: f64 = g.get_weight(attr, weight_key).unwrap_or(1.0);

            if w < dist[i][j] {
                dist[i][j] = w;
                pred[i][j] = Some(i);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik.is_infinite() {
                continue;
            }

            for j in 0..n {
                let dkj = dist[k][j];
                if dkj.is_infinite() {
                    continue;
                }

                let alt = dik + dkj;
                if alt < dist[i][j] {
                    dist[i][j] = alt;
                    pred[i][j] = pred[k][j];
                }
            }
        }
    }

    Ok((pred, dist, nodes))
}

pub fn floyd_warshall_v3<'a, N>(
    g: &'a Graph<N>,
    weight_key: &str,
) -> Result<(Vec<Vec<Option<usize>>>, Vec<Vec<f64>>, Vec<&'a N>), String>
where
    N: Eq + Hash + Clone,
{
    let nodes: Vec<&'a N> = g.node.keys().collect();
    let n = nodes.len();

    let mut index: HashMap<&'a N, usize> = HashMap::with_capacity(n);
    for (i, node) in nodes.iter().enumerate() {
        index.insert(*node, i);
    }

    let mut dist: Vec<Vec<f64>> = vec![vec![f64::INFINITY; n]; n];
    let mut pred: Vec<Vec<Option<usize>>> = vec![vec![None; n]; n];

    for i in 0..n {
        dist[i][i] = 0.0;
    }

    for (u, neighbors) in &g.adj_outer {
        let i = index[u];

        for (v, attr) in neighbors {
            let j = index[v];
            let w: f64 = g.get_weight(attr, weight_key).unwrap_or(1.0);

            if w < dist[i][j] {
                dist[i][j] = w;
                pred[i][j] = Some(i);
            }
        }
    }

    for k in 0..n {
        for i in 0..n {
            let dik = dist[i][k];
            if dik.is_infinite() {
                continue;
            }

            for j in 0..n {
                let dkj = dist[k][j];
                if dkj.is_infinite() {
                    continue;
                }

                let alt = dik + dkj;
                if alt < dist[i][j] {
                    dist[i][j] = alt;
                    pred[i][j] = pred[k][j];
                }
            }
        }
    }

    Ok((pred, dist, nodes))
}