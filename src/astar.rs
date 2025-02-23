use std::collections::{BinaryHeap, HashSet};

use glam::UVec2;
use indexmap::map::Entry;

use crate::maze::MazeState;

pub fn astar(start: UVec2, goal: UVec2, maze: &MazeState) -> Option<(HashSet<UVec2>, i32)> {
    let mut open_set = BinaryHeap::new();
    open_set.push(SmallestCostHolder {
        estimated_cost: 0,
        cost: 0,
        index: 0,
    });

    let mut parents: FxIndexMap<UVec2, (usize, i32)> = FxIndexMap::default();
    parents.insert(start, (usize::MAX, 0));

    while let Some(SmallestCostHolder { cost, index, .. }) = open_set.pop() {
        let successors = {
            let (&node, &(_, c)) = parents.get_index(index).unwrap();

            if node == goal {
                let path = reconstruct_path(&parents, index);
                return Some((path, cost));
            }

            if cost > c {
                continue;
            }

            maze.neighbors[node].into_iter().map(move |dir| dir.offset(node))
        };

        for successor in successors {
            let new_cost = cost + 1;
            let h;
            let n;

            match parents.entry(successor) {
                Entry::Vacant(e) => {
                    h = heuristic(*e.key(), goal);
                    n = e.index();
                    e.insert((index, new_cost));
                },
                Entry::Occupied(mut e) => {
                    if e.get().1 > new_cost {
                        h = heuristic(*e.key(), goal);
                        n = e.index();
                        e.insert((index, new_cost));
                    } else {
                        continue;
                    }
                },
            }

            open_set.push(SmallestCostHolder {
                estimated_cost: new_cost + h,
                cost: new_cost,
                index: n,
            });
        }
    }

    None
}

fn reconstruct_path(parents: &FxIndexMap<UVec2, (usize, i32)>, mut i: usize) -> HashSet<UVec2> {
    std::iter::from_fn(|| {
        parents.get_index(i).map(|(&node, value)| {
            i = value.0;
            node
        })
    })
    .collect()
}

struct SmallestCostHolder<C> {
    estimated_cost: C,
    cost: C,
    index: usize,
}

impl<C: PartialEq> PartialEq for SmallestCostHolder<C> {
    fn eq(&self, other: &Self) -> bool {
        self.estimated_cost.eq(&other.estimated_cost) && self.cost.eq(&other.cost)
    }
}

impl<C: PartialEq> Eq for SmallestCostHolder<C> {}

impl<C: Ord> Ord for SmallestCostHolder<C> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match other.estimated_cost.cmp(&self.estimated_cost) {
            std::cmp::Ordering::Equal => self.cost.cmp(&other.cost),
            s => s,
        }
    }
}

impl<C: Ord> PartialOrd for SmallestCostHolder<C> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

type FxIndexMap<K, V> = indexmap::IndexMap<K, V, std::hash::BuildHasherDefault<rustc_hash::FxHasher>>;

fn heuristic(pos: UVec2, goal: UVec2) -> i32 {
    (goal.x as i32 - pos.x as i32).abs() + (goal.y as i32 - pos.y as i32).abs()
}
