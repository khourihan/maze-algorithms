use std::collections::{HashMap, HashSet};

use glam::UVec2;
use rand::{rngs::SmallRng, seq::SliceRandom, Rng, SeedableRng};

use crate::{direction::Direction, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct Eller {
    rng: SmallRng,
    row: RowState,
    next_row: RowState,
    cells: Vec<(usize, UVec2)>,
    finalizing: bool,
}

impl Eller {
    pub fn new() -> Eller {
        Eller {
            rng: SmallRng::seed_from_u64(0),
            row: RowState::new(0, 0),
            next_row: RowState::new(0, 0),
            cells: Vec::new(),
            finalizing: false,
        }
    }
}

impl Algorithm for Eller {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());
        self.row = RowState::new(0, maze.size.x);

        maze.head = UVec2::ZERO;
    }

    fn step(&mut self, maze: &mut MazeState) {
        if maze.head.x < maze.size.x {
            if self.finalizing {
                maze.set_finalized(maze.head);

                let (index, cell) = self.cells.pop().unwrap();

                if (index == 0 || self.rng.random_bool(1.0 / 3.0)) && !maze.edges(cell).north {
                    maze.neighbors.open(cell, Direction::North);
                    self.next_row.record(self.row.set_for(cell), cell + UVec2::Y);
                    maze.set_visited(cell + UVec2::Y);
                }

                if self.cells.is_empty() {
                    self.row = self.next_row.clone();
                    maze.head.x = maze.size.x;
                }
            } else {
                maze.set_visited(maze.head);

                let edges = maze.edges(maze.head);
                if edges.west {
                    maze.head.x += 1;
                    return;
                }

                let set = self.row.set_for(maze.head);
                let old_set = self.row.set_for(maze.head - UVec2::X);

                if set != old_set && (edges.north || self.rng.random_bool(0.5)) {
                    maze.neighbors.open(maze.head, Direction::West);
                    self.row.merge(old_set, set);
                }
            }

            maze.head.x += 1;
        } else {
            self.finalizing ^= true;

            maze.head.x = 0;
            if self.finalizing {
                self.next_row = self.row.next();
                self.cells.clear();

                for (_set, cells) in self.row.cells_in_set.iter() {
                    let mut c: Vec<_> = cells.iter().copied().collect();
                    c.shuffle(&mut self.rng);

                    self.cells.extend(c.into_iter().enumerate());
                }
            } else {
                maze.head.y += 1;

                if maze.head.y == maze.size.y {
                    maze.finish();
                }
            }
        }
    }
}

#[derive(Debug, Clone)]
struct RowState {
    width: u32,
    cells_in_set: HashMap<usize, HashSet<UVec2>>,
    set_for_cell: Vec<Option<usize>>,
    next_set: usize,
}

impl RowState {
    pub fn new(start_set: usize, width: u32) -> RowState {
        RowState {
            width,
            cells_in_set: HashMap::new(),
            set_for_cell: vec![None; width as usize],
            next_set: start_set,
        }
    }

    pub fn record(&mut self, set: usize, cell: UVec2) {
        self.set_for_cell[cell.x as usize] = Some(set);
        self.cells_in_set.entry(set).or_default();
        self.cells_in_set.get_mut(&set).unwrap().insert(cell);
    }

    pub fn set_for(&mut self, cell: UVec2) -> usize {
        if self.set_for_cell[cell.x as usize].is_none() {
            self.record(self.next_set, cell);
            self.next_set += 1;
        }

        self.set_for_cell[cell.x as usize].unwrap()
    }

    pub fn merge(&mut self, winner: usize, loser: usize) {
        let cells: Vec<_> = self.cells_in_set.get(&loser).unwrap().iter().copied().collect();

        for cell in cells {
            self.set_for_cell[cell.x as usize] = Some(winner);
            self.cells_in_set.get_mut(&winner).unwrap().insert(cell);
        }

        self.cells_in_set.remove(&loser);
    }

    pub fn next(&self) -> RowState {
        RowState::new(self.next_set, self.width)
    }
}
