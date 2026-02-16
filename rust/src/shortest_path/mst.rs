use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

struct HeapItem<N>
where   
    N: Eq + Hash + Clone
{
    w: f64,
    tie: u64,
    u: N,
    v: N,
}

impl<N> Eq for HeapItem<N> where N: Eq + Hash + Clone {}

impl<N> PartialEq for HeapItem<N>
where
    N: Eq + Hash + Clone,
{
    fn eq(&self, other: &Self) -> bool {
        self.w == other.w && self.tie == other.tie
    }
}

impl<N> Ord for HeapItem<N>
where
    N: Eq + Hash + Clone,
{
    fn cmp(&self, other: &Self) -> Ordering {
        // Reverse ordering for min-heap behavior
        match self.w.partial_cmp(&other.w) {
            Some(ord) => ord.reverse().then_with(|| self.tie.cmp(&other.tie).reverse()),
            None => Ordering::Equal,
        }
    }
}

impl<N> PartialOrd for HeapItem<N>
where
    N: Eq + Hash + Clone,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn prim_mst_edges<N>(
    g: &Graph<N>,
    weight_key: &str,
    ignore_nan: bool,
) -> Result<Vec<(N,N)>, String>
where
    N: Eq + Hash + Clone + Debug
{
    
    let mut nodes: HashSet<N> = g.node.keys().cloned().collect();

    let mut visited: HashSet<N> = HashSet::new();
    let mut mst_edges: Vec<(N, N)> = Vec::new();

    let mut tie: u64 = 0;

    while let Some(start) = nodes.iter().next().cloned() {
        visited.insert(start.clone());
        nodes.remove(&start);

    
        let mut heap: BinaryHeap<HeapItem<N>> = BinaryHeap::new();

        if let Some(nbr) = g.adj_outer.get(&start) {
            for (v, attr) in nbr {
                let w = g.get_weight(attr, weight_key).unwrap_or(1.0);
                
                if w.is_nan() {
                    if ignore_nan {
                        continue;
                    }
                    return Err(format!("NaN edge weight found: {:?} -> {:?}", start, v));
                }
                tie += 1;
                heap.push(HeapItem {
                    w,
                    tie,
                    u: start.clone(),
                    v: v.clone(),
                });
            }
        }

        while let Some(edge) = heap.pop() {
            if visited.contains(&edge.v) {
                continue;
            }

            mst_edges.push((edge.u.clone(), edge.v.clone()));
            visited.insert(edge.v.clone());
            nodes.remove(&edge.v);

            if let Some(nbr2) = g.adj_outer.get(&edge.v) {
                for (dst, attr2) in nbr2 {
                    if visited.contains(dst) {
                        continue;
                    }
                    let new_weight = g.get_weight(attr2, weight_key).unwrap_or(1.0);
                    if new_weight.is_nan() {
                        if ignore_nan {
                            continue;
                        }
                        return Err(format!("NaN edge weight found: {:?} -> {:?}", edge.v, dst));
                    }
                    tie += 1;
                    heap.push(HeapItem {
                        w: new_weight,
                        tie,
                        u: edge.v.clone(),
                        v: dst.clone(),
                    });
                }
            }
        }
    }
    
    Ok(mst_edges)

}