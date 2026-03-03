use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::utils::{AttrMap, AttrValue};
// A beginner-friendly DiGraph mirroring NetworkX DiGraph layout:
//
// NetworkX-ish mental model:
// - graph: graph-level attributes
// - node: node -> attributes
// - succ: u -> (v -> edge attrs)   // outgoing adjacency
// - pred: v -> (u -> edge attrs)   // incoming adjacency
//
// We also keep small caches for quick traversal lists:
// - adj_outer_cache: u -> Vec<v>
// - predecessors_cache: v -> Vec<u>
#[derive(Debug, Clone)]
pub struct DiGraph<N>
where
    N: Eq + Hash + Clone,
{
    pub graph: AttrMap,
    pub node: HashMap<N, AttrMap>,
    pub adj_outer: HashMap<N, HashMap<N, AttrMap>>,
    pub pred: HashMap<N, HashMap<N, AttrMap>>,

    pub adj_outer_cache: HashMap<N, Vec<N>>,
    pub predecessors_cache: HashMap<N, Vec<N>>,
}

impl<N> DiGraph<N>
where
    N: Eq + Hash + Clone,
{
    pub fn new<I>(graph_attr: I) -> Self
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        let mut g = DiGraph {
            graph: HashMap::new(),
            node: HashMap::new(),
            adj_outer: HashMap::new(),
            pred: HashMap::new(),

            adj_outer_cache: HashMap::new(),
            predecessors_cache: HashMap::new(),
        };

        g.graph.extend(graph_attr);
        g
    }

    pub fn add_node<I>(&mut self, new_node: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        // Ensure the node exists in all core maps (like NetworkX does)
        self.adj_outer.entry(new_node.clone()).or_default();
        self.pred.entry(new_node.clone()).or_default();
        self.adj_outer_cache.entry(new_node.clone()).or_default();
        self.predecessors_cache.entry(new_node.clone()).or_default();

        // Node attributes
        let entry = self.node.entry(new_node).or_insert_with(HashMap::new);
        entry.extend(attr);
    }

    /// Add a *directed* edge src -> dst.
    /// (Unlike your undirected Graph version, we DO NOT add dst -> src.)
    pub fn add_edge<I>(&mut self, src: N, dst: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)> + Clone,
    {
        // NetworkX behavior: adding an edge implicitly adds the nodes
        self.add_node(src.clone(), std::iter::empty());
        self.add_node(dst.clone(), std::iter::empty());

        // Outgoing: adj_outer[src][dst] = attrs
        {
            let nbrs = self.adj_outer.get_mut(&src).unwrap();
            let edge_attr = nbrs.entry(dst.clone()).or_default();
            edge_attr.extend(attr.clone());
        }

        // Incoming: pred[dst][src] = attrs
        {
            let nbrs = self.pred.get_mut(&dst).unwrap();
            let edge_attr = nbrs.entry(src.clone()).or_default();
            edge_attr.extend(attr);
        }

        // Maintain caches
        let out_list = self.adj_outer_cache.entry(src.clone()).or_default();
        if !out_list.contains(&dst) {
            out_list.push(dst.clone());
        }

        let in_list = self.predecessors_cache.entry(dst.clone()).or_default();
        if !in_list.contains(&src) {
            in_list.push(src.clone());
        }
    }

    /// adj_outer (out-neighbors), similar to `G.adj_outer(n)` in NetworkX.
    pub fn adj_outer(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.adj_outer_cache
            .get(node)
            .map(|v| v.iter())
            .ok_or_else(|| "Node is not in the graph.".to_string())
    }

    /// Predecessors (in-neighbors), similar to `G.predecessors(n)` in NetworkX.
    pub fn predecessors(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.predecessors_cache
            .get(node)
            .map(|v| v.iter())
            .ok_or_else(|| "Node is not in the graph.".to_string())
    }

    /// NetworkX `neighbors()` on a DiGraph is effectively "adj_outer" (out-neighbors).
    pub fn neighbors(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.adj_outer_cache
            .get(node)
            .map(|m| m.iter())
            .ok_or_else(|| "Node is not in the graph.".to_string())
    }

    pub fn get_weight(&self, attr: &AttrMap, weight_key: &str) -> Option<f64> {
        match attr.get(weight_key) {
            Some(AttrValue::Float(w)) => Some(*w),
            Some(AttrValue::Int(w)) => Some(*w as f64),
            _ => None, // missing or wrong type
        }
    }

    /// Return directed edges. No `seen` set needed because direction matters.
    ///
    /// - data=false: (u, v, None)
    /// - data=true:  (u, v, Some(edge_attr))
    pub fn edges(&self, data: bool) -> Vec<(N, N, Option<AttrMap>)> {
        let mut result = Vec::new();
        for (u, nbrs) in &self.adj_outer {
            for (v, attr) in nbrs {
                if data {
                    result.push((u.clone(), v.clone(), Some(attr.clone())));
                } else {
                    result.push((u.clone(), v.clone(), None));
                }
            }
        }
        result
    }

    /// Optional helper: return all nodes (like `G.nodes()` conceptually)
    pub fn nodes(&self) -> impl Iterator<Item = &N> {
        self.node.keys()
    }

    /// Optional helper: out-degree
    pub fn out_degree(&self, node: &N) -> Result<usize, String> {
        Ok(self
            .adj_outer_cache
            .get(node)
            .ok_or_else(|| "Node is not in the graph.".to_string())?
            .len())
    }

    /// Optional helper: in-degree
    pub fn in_degree(&self, node: &N) -> Result<usize, String> {
        Ok(self
            .predecessors_cache
            .get(node)
            .ok_or_else(|| "Node is not in the graph.".to_string())?
            .len())
    }
}