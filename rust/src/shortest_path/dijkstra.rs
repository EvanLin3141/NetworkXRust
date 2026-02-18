use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};
use std::hash::Hash;

use crate::graph::nx_graph::Graph;

#[derive(Debug, Clone)]
struct State<N> {
    cost: f64,
    node: N,
}

impl<N: Eq> PartialEq for State<N> {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}
impl<N: Eq> Eq for State<N> {}

impl<N: Eq> PartialOrd for State<N> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        // reverse for min-heap
        other.cost.partial_cmp(&self.cost)
    }
}
impl<N: Eq> Ord for State<N> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap_or(Ordering::Equal)
    }
}


pub fn dijkstra_path<N>(
    g: &Graph<N>,
    start: &N, 
    goal: &N
) -> Result<Option<(f64, Vec<N>)>, String> 
where
    N: Eq + Hash + Clone
{
    let mut dist: HashMap<N, f64> = HashMap::new();
    let mut prev: HashMap<N, N> = HashMap::new();
    let mut heap: BinaryHeap<State<N>> = BinaryHeap::new();

    dist.insert(start.clone(), 0.0);
    heap.push(State {
        cost: 0.0,
        node: start.clone(),
    });

    while let Some(State { cost, node }) = heap.pop() {
        let best_cost = *dist.get(&node).unwrap_or(&f64::INFINITY);
        if cost > best_cost {
            continue;
        }

        if &node == goal {
            let mut path = Vec::new();
            let mut cur = goal.clone();
            path.push(cur.clone());

            while &cur != start {
                let parent = match prev.get(&cur) {
                    Some(p) => p,
                    None => return Ok(None),
                };
                cur = parent.clone();
                path.push(cur.clone());
            }

            path.reverse();
            return Ok(Some((cost, path)));
        }

        let neighbors = match g.adj_outer.get(&node) {
            Some(m) => m,
            None => continue, 
        };

        for (neighbor, edge_attr) in neighbors.iter() {
            let w = g.get_weight(edge_attr, "weight").unwrap_or(1.0);
            let next_cost = cost + w;

            let neighbor_best = *dist.get(neighbor).unwrap_or(&f64::INFINITY);
            if next_cost < neighbor_best {
                dist.insert(neighbor.clone(), next_cost);
                prev.insert(neighbor.clone(), node.clone());
                heap.push(State {
                    cost: next_cost,
                    node: neighbor.clone(),
                });
            }
        }
    }

    Ok(None)
}


