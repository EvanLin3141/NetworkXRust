use std::collections::{HashMap, HashSet};
use std::hash::Hash;

use crate::utils::{AttrMap, AttrValue};

#[derive(Debug, Clone)]
pub struct Graph<N>
where
    N: Eq + Hash + Clone,
{
    pub graph: AttrMap,
    pub node: HashMap<N, AttrMap>,
    pub adj_outer: HashMap<N, HashMap<N, AttrMap>>,
    pub neighbors: HashMap<N, HashSet<N>>, // cache for traversals

    // BFS / DFS Traversal Speed Ups
    pub node_to_idx: HashMap<N, usize>,
    pub idx_to_node: Vec<N>,
    pub adj_idx: Vec<Vec<usize>>,
}

impl<N> Graph<N>
where
    N: Eq + Hash + Clone,
{
    pub fn new<I>(graph_attr: I) -> Self
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        let mut g = Graph {
            graph: HashMap::new(),
            node: HashMap::new(),
            adj_outer: HashMap::new(),
            neighbors: HashMap::new(),

            node_to_idx: HashMap::new(),
            idx_to_node: Vec::new(),
            adj_idx: Vec::new(),
        };

        g.graph.extend(graph_attr);
        g
    }

    pub fn ensure_node_index(&mut self, node: &N) -> usize {
        if let Some(&idx) = self.node_to_idx.get(node) {
            return idx;
        }

        let idx = self.idx_to_node.len();
        self.node_to_idx.insert(node.clone(), idx);
        self.idx_to_node.push(node.clone());
        self.adj_idx.push(Vec::new());
        idx
    }

    pub fn add_node<I>(&mut self, new_node: N, attr: I)
    where
        I: IntoIterator<Item = (String, AttrValue)>,
    {
        self.ensure_node_index(&new_node);

        if !self.node.contains_key(&new_node) {
            self.adj_outer.insert(new_node.clone(), HashMap::new());
            self.neighbors.entry(new_node.clone()).or_default();

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
        let src_idx = self.ensure_node_index(&src);
        let dst_idx = self.ensure_node_index(&dst);

        self.node.entry(src.clone()).or_default();
        self.node.entry(dst.clone()).or_default();

        self.adj_outer.entry(src.clone()).or_default();
        self.adj_outer.entry(dst.clone()).or_default();

        {
            let nbrs = self.adj_outer.get_mut(&src).unwrap();
            let nbr_dict = nbrs.entry(dst.clone()).or_default();
            nbr_dict.extend(attr.clone());
        }

        {
            let nbrs = self.adj_outer.get_mut(&dst).unwrap();
            let nbr_dict = nbrs.entry(src.clone()).or_default();
            nbr_dict.extend(attr);
        }

        self.neighbors
            .entry(src.clone())
            .or_default()
            .insert(dst.clone());

        self.neighbors
            .entry(dst.clone())
            .or_default()
            .insert(src.clone());

        // keep indexed adjacency in sync
        if !self.adj_idx[src_idx].contains(&dst_idx) {
            self.adj_idx[src_idx].push(dst_idx);
        }

        if !self.adj_idx[dst_idx].contains(&src_idx) {
            self.adj_idx[dst_idx].push(src_idx);
        }
    }

    pub fn neighbors(&self, node: &N) -> Result<impl Iterator<Item = &N>, String> {
        self.neighbors
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
        let mut seen: HashSet<(N, N)> = HashSet::new();

        for (u, nbrs) in &self.adj_outer {
            for (v, attr) in nbrs {
                if seen.contains(&(v.clone(), u.clone())) {
                    continue;
                }

                seen.insert((u.clone(), v.clone()));

                if data {
                    result.push((u.clone(), v.clone(), Some(attr.clone())));
                } else {
                    result.push((u.clone(), v.clone(), None));
                }
            }
        }

        result
    }

    // BFS / DFS Helper Functions
    pub fn get_index(&self, node: &N) -> Option<usize> {
        self.node_to_idx.get(node).copied()
    }

    pub fn get_node_by_index(&self, idx: usize) -> Option<&N> {
        self.idx_to_node.get(idx)
    }

    pub fn node_count(&self) -> usize {
        self.idx_to_node.len()
    }
}
