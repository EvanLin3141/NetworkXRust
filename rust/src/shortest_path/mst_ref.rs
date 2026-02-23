use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet};
use std::fmt::Debug;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

#[derive(Debug)]
struct HeapItem<'a, N> {
    w: f64,
    tie: u64,
    u: &'a N,
    v: &'a N,
}

// BinaryHeap is a max-heap; we invert ordering to behave like a min-heap.
impl<'a, N> PartialEq for HeapItem<'a, N> {
    fn eq(&self, other: &Self) -> bool {
        self.w.to_bits() == other.w.to_bits() && self.tie == other.tie
    }
}
impl<'a, N> Eq for HeapItem<'a, N> {}

impl<'a, N> PartialOrd for HeapItem<'a, N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl<'a, N> Ord for HeapItem<'a, N> {
    fn cmp(&self, other: &Self) -> Ordering {
        // min-heap: smallest weight first
        other
            .w
            .partial_cmp(&self.w)
            .unwrap()
            .then_with(|| other.tie.cmp(&self.tie))
    }
}

pub fn prim_mst_edges_ref<'a, N>(
    g: &'a Graph<N>,
    weight_key: &str,
    ignore_nan: bool,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Debug + Clone,
{
    
    // let mut nodes: HashSet<&'a N> = g.node.keys().collect();
    let mut unvisited: HashSet<&'a N> = g.node.keys().collect();    
    let mut visited: HashSet<&'a N> = HashSet::new();

    let mut mst_edges: Vec<(&'a N, &'a N)> = Vec::new();
    let mut tie: u64 = 0;

    while let Some(start) = unvisited.iter().next().copied() {
        visited.insert(start);
        unvisited.remove(&start);

        let mut heap: BinaryHeap<HeapItem<'a, N>> = BinaryHeap::new();

        if let Some(nbr) = g.adj_outer.get(start) {
            for (v, attr) in nbr {
                let w = g.get_weight(attr, weight_key).unwrap_or(1.0);
                
                if w.is_nan() {
                    if ignore_nan {
                        continue;
                    }
                    return Err(format!("NaN edge weight found: {:?} -> {:?}", start, v));
                }
                tie += 1;
                heap.push(HeapItem {w, tie, u: start, v: v,});
            }
        }

        while let Some(edge) = heap.pop() {
            if visited.contains(&edge.v) {
                continue;
            }
            mst_edges.push((edge.u, edge.v));
            visited.insert(edge.v);
            unvisited.remove(&edge.v);

            if let Some(nbr2) = g.adj_outer.get(edge.v) {
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
                        u: edge.v,
                        v: dst,
                    });
                }
            }
        }
    }
    
    Ok(mst_edges)
}