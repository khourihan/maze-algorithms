use std::collections::VecDeque;

use glam::UVec2;
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{direction::Directions, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct DepthFirstSearch {
    rng: SmallRng,
    visited: VecDeque<UVec2>,
}

impl DepthFirstSearch {
    pub fn new() -> DepthFirstSearch {
        DepthFirstSearch {
            rng: SmallRng::seed_from_u64(0),
            visited: VecDeque::new(),
        }
    }
}

impl Algorithm for DepthFirstSearch {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());
        self.visited.clear();

        let x = self.rng.random_range(0..maze.size.x);
        let y = self.rng.random_range(0..maze.size.y);

        maze.head = UVec2::new(x, y);

        self.visited.push_back(maze.head);
        maze.set_visited(maze.head);
    }

    fn step(&mut self, maze: &mut MazeState) {
        let mut dirs = !Directions::from_fn(|d| d.checked_offset(maze.head).is_some_and(|c| maze.visited(c)));
        dirs &= !maze.edges(maze.head);

        if let Some(dir) = dirs.choose(&mut self.rng) {
            maze.neighbors.open(maze.head, dir);
            maze.head = dir.offset(maze.head);

            self.visited.push_back(maze.head);
            maze.set_visited(maze.head);
        } else if let Some(previous) = self.visited.pop_back() {
            maze.head = previous;
            maze.set_finalized(maze.head);
        } else {
            maze.finish();
        }
    }
}
