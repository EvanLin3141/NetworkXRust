use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::utils::{AttrMap, AttrValue};

#[derive(Debug, Clone)]
pub struct DiGraph<N>
where
    N: Eq + Hash + Clone,
{
    pub graph: AttrMap,
    pub node: HashMap<N, AttrMap>,

    // outgoing adjacency: src -> (dst -> edge attrs)
    pub adj_outer: HashMap<N, HashMap<N, AttrMap>>,

    // incoming adjacency: dst -> (src -> edge attrs)
    pub pred: HashMap<N, HashMap<N, AttrMap>>,

    // traversal caches, matching Graph style
    pub neighbors: HashMap<N, HashSet<N>>,      // out-neighbors
    pub predecessors: HashMap<N, HashSet<N>>,   // in-neighbors
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
            neighbors: HashMap::new(),
            predecessors: HashMap::new(),
        };

        g.graph.extend(graph_attr);
        g
    }

    pub fn add_node<I>(&mut self, new_node: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        if !self.node.contains_key(&new_node) {
            self.adj_outer.insert(new_node.clone(), HashMap::new());
            self.pred.insert(new_node.clone(), HashMap::new());
            self.neighbors.entry(new_node.clone()).or_default();
            self.predecessors.entry(new_node.clone()).or_default();

            let mut attr_dict = AttrMap::new();
            attr_dict.extend(attr);
            self.node.insert(new_node, attr_dict);
        } else if let Some(existing) = self.node.get_mut(&new_node) {
            existing.extend(attr);
        }
    }

    pub fn add_edge<I>(&mut self, src: N, dst: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)> + Clone,
    {
        self.node.entry(src.clone()).or_default();
        self.node.entry(dst.clone()).or_default();

        self.adj_outer.entry(src.clone()).or_default();
        self.adj_outer.entry(dst.clone()).or_default();

        self.pred.entry(src.clone()).or_default();
        self.pred.entry(dst.clone()).or_default();

        {
            let nbrs = self.adj_outer.get_mut(&src).unwrap();
            let nbr_dict = nbrs.entry(dst.clone()).or_default();
            nbr_dict.extend(attr.clone());
        }

        {
            let preds = self.pred.get_mut(&dst).unwrap();
            let pred_dict = preds.entry(src.clone()).or_default();
            pred_dict.extend(attr);
        }

        self.neighbors
            .entry(src.clone())
            .or_default()
            .insert(dst.clone());

        self.predecessors
            .entry(dst.clone())
            .or_default()
            .insert(src.clone());
    }

    pub fn neighbors(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.neighbors
            .get(node)
            .map(|s| s.iter())
            .ok_or_else(|| "Node is not in the graph.".to_string())
    }

    pub fn predecessors(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.predecessors
            .get(node)
            .map(|s| s.iter())
            .ok_or_else(|| "Node is not in the graph.".to_string())
    }

    pub fn get_weight(&self, attr: &AttrMap, weight_key: &str) -> Option<f64> {
        match attr.get(weight_key) {
            Some(AttrValue::Float(w)) => Some(*w),
            Some(AttrValue::Int(w)) => Some(*w as f64),
            _ => None,
        }
    }

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
}