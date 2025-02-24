use std::collections::HashSet;

use glam::UVec2;
use rand::{rngs::SmallRng, seq::SliceRandom, SeedableRng};

use crate::{direction::Direction, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct Kruskal {
    rng: SmallRng,
    walls: Vec<u32>,
    wall_set: HashSet<u32>,
    parent: Vec<usize>,
    rank: Vec<usize>,
}

impl Kruskal {
    pub fn new() -> Kruskal {
        Kruskal {
            rng: SmallRng::seed_from_u64(0),
            walls: Vec::new(),
            wall_set: HashSet::new(),
            parent: Vec::new(),
            rank: Vec::new(),
        }
    }

    fn find_parent(&mut self, i: usize) -> usize {
        if self.parent[i] != i {
            self.parent[i] = self.find_parent(self.parent[i]);
        }

        self.parent[i]
    }

    fn union(&mut self, x: usize, y: usize) {
        match self.rank[x].cmp(&self.rank[y]) {
            std::cmp::Ordering::Less => self.parent[x] = y,
            std::cmp::Ordering::Greater => self.parent[y] = x,
            std::cmp::Ordering::Equal => {
                self.parent[y] = x;
                self.rank[x] += 1;
            },
        }
    }
}

impl Algorithm for Kruskal {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());

        self.wall_set = (0..maze.walls()).collect();
        self.walls = (0..maze.walls()).collect();
        self.walls.shuffle(&mut self.rng);

        self.parent.clear();
        self.rank.clear();
        self.parent.reserve((maze.size.x * maze.size.y) as usize);
        self.rank.reserve((maze.size.x * maze.size.y) as usize);

        for y in 0..maze.size.y {
            for x in 0..maze.size.x {
                self.parent.push((y * maze.size.x + x) as usize);
                self.rank.push(0);
            }
        }

        maze.wall_head = self.walls.pop().unwrap();
        self.wall_set.remove(&maze.wall_head);
    }

    fn step(&mut self, maze: &mut MazeState) {
        let (a, b) = if maze.wall_head >= (maze.size.x - 1) * maze.size.y {
            // Vertical wall
            let pos = maze.wall_head - (maze.size.x - 1) * maze.size.y;
            let pos = UVec2::new(pos % maze.size.x, pos / maze.size.x);
            (pos, pos + UVec2::Y)
        } else {
            // Horizontal wall
            let pos = UVec2::new(maze.wall_head % (maze.size.x - 1), maze.wall_head / (maze.size.x - 1));
            (pos, pos + UVec2::X)
        };

        let u = self.find_parent((a.y * maze.size.x + a.x) as usize) as u32;
        let v = self.find_parent((b.y * maze.size.x + b.x) as usize) as u32;

        let (mut changed_a, mut changed_b) = (false, false);

        if u != v {
            maze.neighbors.open(a, Direction::from_offset(a, b));
            self.union(u as usize, v as usize);

            if !maze.visited(a) {
                changed_a = true;
            }

            if !maze.visited(b) {
                changed_b = true;
            }

            maze.set_visited(a);
            maze.set_visited(b);
        }

        let mut update_finalized = vec![a, b];
        if changed_a {
            if let Some(c) = a.checked_sub(UVec2::X) {
                update_finalized.push(c);
            }

            if let Some(c) = a.checked_sub(UVec2::Y) {
                update_finalized.push(c);
            }

            if a.x < maze.size.x - 1 {
                update_finalized.push(a + UVec2::X);
            }

            if a.y < maze.size.y - 1 {
                update_finalized.push(a + UVec2::Y);
            }
        }

        if changed_b {
            if let Some(c) = b.checked_sub(UVec2::X) {
                update_finalized.push(c);
            }

            if let Some(c) = b.checked_sub(UVec2::Y) {
                update_finalized.push(c);
            }

            if b.x < maze.size.x - 1 {
                update_finalized.push(b + UVec2::X);
            }

            if b.y < maze.size.y - 1 {
                update_finalized.push(b + UVec2::Y);
            }
        }

        for c in update_finalized {
            if maze.visited(c) && !maze.finalized(c) {
                let edges = !maze.edges(c);
                let east = c.y * (maze.size.x - 1) + c.x;
                let north = c.y * maze.size.x + c.x + (maze.size.x - 1) * maze.size.y;
                let west = east.max(1) - 1;
                let south = north.max(maze.size.x) - maze.size.x;

                if !(edges.east && self.wall_set.contains(&east)
                    || edges.north && self.wall_set.contains(&north)
                    || edges.west && self.wall_set.contains(&west)
                    || edges.south && self.wall_set.contains(&south))
                {
                    maze.set_finalized(c);
                }
            }
        }

        if let Some(next) = self.walls.pop() {
            maze.wall_head = next;
            self.wall_set.remove(&next);
        } else {
            maze.finish();
        }
    }
}
