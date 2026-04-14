use std::collections::{BinaryHeap, HashMap};
use std::fmt::Debug;
use std::hash::Hash;

use crate::graph::nx_graph::Graph;


#[derive(Debug)]
struct HeapItemIdx {
    w: f64,
    tie: u64,
    u: usize,
    v: usize,
}

impl PartialEq for HeapItemIdx {
    fn eq(&self, other: &Self) -> bool {
        self.w.to_bits() == other.w.to_bits() && self.tie == other.tie
    }
}
impl Eq for HeapItemIdx {}

impl PartialOrd for HeapItemIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HeapItemIdx {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.w.total_cmp(&self.w).then_with(|| other.tie.cmp(&self.tie))
    }
}


pub fn prim_mst_edges<'a, N>(
    g: &'a Graph<N>,
    weight_key: &str,
    ignore_nan: bool,
) -> Result<Vec<(&'a N, &'a N)>, String>
where
    N: Eq + Hash + Debug + Clone,
{
    
    let nodes: Vec<&'a N> = g.node.keys().collect();

    let mut idx_of: HashMap<&'a N, usize> = HashMap::with_capacity(nodes.len());
    for (i, &node) in nodes.iter().enumerate() {
        idx_of.insert(node, i);
    }

    let mut visited = vec![false; nodes.len()];
    let mut mst_edges: Vec<(&'a N, &'a N)> = Vec::with_capacity(nodes.len().saturating_sub(1));
    let mut tie: u64 = 0;

    // handle disconnected graphs: start Prim from each component
    let mut start_i = 0;
    while start_i < nodes.len() {
        while start_i < nodes.len() && visited[start_i] {
            start_i += 1;
        }
        if start_i == nodes.len() {
            break;
        }

        visited[start_i] = true;
        let start = nodes[start_i];

        let mut heap: BinaryHeap<HeapItemIdx> = BinaryHeap::new();

        if let Some(nbr) = g.adj_outer.get(start) {
            for (v, attr) in nbr {
                let w = g.get_weight(attr, weight_key).unwrap_or(1.0);
                if w.is_nan() {
                    if ignore_nan {
                        continue;
                    }
                    return Err(format!("NaN edge weight found: {:?} -> {:?}", start, v));
                }

                let vi = *idx_of
                    .get(v)
                    .ok_or_else(|| format!("Neighbor not found in node index: {:?}", v))?;

                if visited[vi] {
                    continue;
                }

                tie += 1;
                heap.push(HeapItemIdx {
                    w,
                    tie,
                    u: start_i,
                    v: vi,
                });
            }
        }

        while let Some(edge) = heap.pop() {
            if visited[edge.v] {
                continue;
            }

            visited[edge.v] = true;
            mst_edges.push((nodes[edge.u], nodes[edge.v]));

            let vref = nodes[edge.v];
            if let Some(nbr2) = g.adj_outer.get(vref) {
                for (dst, attr2) in nbr2 {
                    let di = *idx_of
                        .get(dst)
                        .ok_or_else(|| format!("Neighbor not found in node index: {:?}", dst))?;

                    if visited[di] {
                        continue;
                    }

                    let new_weight = g.get_weight(attr2, weight_key).unwrap_or(1.0);
                    if new_weight.is_nan() {
                        if ignore_nan {
                            continue;
                        }
                        return Err(format!("NaN edge weight found: {:?} -> {:?}", vref, dst));
                    }

                    tie += 1;
                    heap.push(HeapItemIdx {
                        w: new_weight,
                        tie,
                        u: edge.v,
                        v: di,
                    });
                }
            }
        }
    }

    Ok(mst_edges)
}
