use std::collections::HashSet;

use glam::UVec2;
use rand::{rngs::SmallRng, seq::IteratorRandom, Rng, SeedableRng};

use crate::{direction::Direction, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct Sidewinder {
    rng: SmallRng,
    run: HashSet<UVec2>,
    finalizing: bool,
}

impl Sidewinder {
    pub fn new() -> Sidewinder {
        Sidewinder {
            rng: SmallRng::seed_from_u64(0),
            run: HashSet::new(),
            finalizing: false,
        }
    }
}

impl Algorithm for Sidewinder {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());
        self.run.clear();
        self.finalizing = false;
        maze.head = UVec2::ZERO;
    }

    fn step(&mut self, maze: &mut MazeState) {
        if maze.head.x < maze.size.x {
            maze.set_visited(maze.head);

            if self.finalizing {
                maze.set_finalized(maze.head);
                maze.head.x += 1;
                return;
            }

            let edges = maze.edges(maze.head);

            if edges.south {
                if !edges.east {
                    maze.neighbors.open(maze.head, Direction::East);
                }
            } else {
                self.run.insert(maze.head);

                if self.rng.random_bool(2.0 / 3.0) && !edges.east {
                    maze.neighbors.open(maze.head, Direction::East);
                } else {
                    let cell = self.run.iter().choose(&mut self.rng).unwrap();
                    maze.neighbors.open(*cell, Direction::South);
                    self.run.clear();
                }

                maze.set_finalized(maze.head - UVec2::Y);
            }

            maze.head.x += 1;
        } else {
            maze.head.x = 0;
            maze.head.y += 1;

            if maze.head.y == maze.size.y {
                if !self.finalizing {
                    maze.head.y -= 1;
                    self.finalizing = true;
                } else {
                    maze.finish();
                }
            }
        }
    }
}
