use glam::UVec2;
use indexmap::IndexSet;
use rand::{rngs::SmallRng, seq::IndexedRandom, Rng, SeedableRng};

use crate::{direction::Directions, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct Prim {
    rng: SmallRng,
    visited: IndexSet<UVec2>,
}

impl Prim {
    pub fn new() -> Prim {
        Prim {
            rng: SmallRng::seed_from_u64(0),
            visited: IndexSet::new(),
        }
    }
}

impl Algorithm for Prim {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());
        self.visited.clear();

        let x = self.rng.random_range(0..maze.size.x);
        let y = self.rng.random_range(0..maze.size.y);

        maze.head = UVec2::new(x, y);

        self.visited.insert(maze.head);
        maze.set_visited(maze.head);
    }

    fn step(&mut self, maze: &mut MazeState) {
        if maze.visited(maze.head) {
            let mut dirs = !Directions::from_fn(|d| d.checked_offset(maze.head).is_some_and(|c| maze.visited(c)));
            dirs &= !maze.edges(maze.head);

            if let Some(dir) = dirs.choose(&mut self.rng) {
                maze.neighbors.open(maze.head, dir);
                maze.head = dir.offset(maze.head);
                return;
            } else {
                self.visited.shift_remove(&maze.head);
                maze.set_finalized(maze.head);
            }
        } else {
            self.visited.insert(maze.head);
            maze.set_visited(maze.head);
        }

        if self.visited.is_empty() {
            maze.finish();
        } else {
            let index = self.rng.random_range(0..self.visited.len());
            let next = self.visited.get_index(index).unwrap();
            maze.head = *next;
        }
    }
}
