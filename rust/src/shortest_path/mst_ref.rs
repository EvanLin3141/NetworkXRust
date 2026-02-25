use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashSet, HashMap};
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

// V2 with several improvements
// 1. Index Mapping instead of HashMaps
// 2. Fix &&a'n
// 3.

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


pub fn prim_mst_edges_v2<'a, N>(
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