use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

// Dijkstra adapted from
// https://doc.rust-lang.org/std/collections/binary_heap/index.html
#[derive(Copy, Clone, Eq, PartialEq)]
pub struct Edge<T>
where
    T: Ord,
{
    pub cost: usize,
    pub node: T,
}

impl<T> Ord for Edge<T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.node.cmp(&other.node))
    }
}

impl<T> PartialOrd for Edge<T>
where
    T: Ord,
{
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

pub fn shortest_path<T>(edges: &dyn Fn(T) -> Vec<Edge<T>>, start: T, goal: T) -> Option<usize>
where
    T: Copy + Ord + std::hash::Hash,
{
    let mut dist: HashMap<T, usize> = HashMap::new();
    let mut heap = BinaryHeap::new();

    heap.push(Edge {
        cost: 0,
        node: start,
    });

    while let Some(Edge { cost, node }) = heap.pop() {
        if node == goal {
            return Some(cost);
        }

        if let Some(&d) = dist.get(&node) {
            if cost > d {
                continue;
            }
        }

        for next_state in &edges(node) {
            let next = Edge {
                cost: cost + next_state.cost,
                node: next_state.node,
            };

            if let Some(&d) = dist.get(&next.node) {
                if next.cost < d {
                    heap.push(next);
                    dist.insert(next.node, next.cost);
                }
            } else {
                heap.push(next);
                dist.insert(next.node, next.cost);
            }
        }
    }

    None
}
