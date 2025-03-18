use glam::{UVec2, UVec4};
use rand::{rngs::SmallRng, Rng, SeedableRng};

use crate::{direction::Direction, maze::MazeState};

use super::Algorithm;

#[derive(Debug, Clone)]
pub struct RecursiveDivision {
    rng: SmallRng,
    first: Vec<UVec4>,
    second: Vec<UVec4>,
}

impl RecursiveDivision {
    pub fn new() -> RecursiveDivision {
        RecursiveDivision {
            rng: SmallRng::seed_from_u64(0),
            first: Vec::new(),
            second: Vec::new(),
        }
    }
}

impl Algorithm for RecursiveDivision {
    fn initialize(&mut self, maze: &mut MazeState) {
        self.rng = SmallRng::from_rng(&mut rand::rng());
        self.first.clear();
        self.first.push(UVec4::new(0, 0, maze.size.x, maze.size.y));

        for x in 0..maze.size.x {
            for y in 0..maze.size.y {
                let p = UVec2::new(x, y);
                let e = maze.edges(p);

                maze.neighbors[p] = !e;
            }
        }
    }

    fn step(&mut self, maze: &mut MazeState) {
        let region = if let Some(r) = self.first.pop() {
            r
        } else if let Some(r) = self.second.pop() {
            r
        } else {
            maze.finish();
            return;
        };

        let (x, y, width, height) = (region.x, region.y, region.z, region.w);

        for i in x..x + width {
            for j in y..y + height {
                maze.set_visited(UVec2::new(i, j));
            }
        }

        if width < 2 || height < 2 {
            for i in x..x + width {
                for j in y..y + height {
                    maze.set_finalized(UVec2::new(i, j));
                }
            }

            return;
        }

        let horizontal = match width.cmp(&height) {
            std::cmp::Ordering::Less => true,
            std::cmp::Ordering::Equal => self.rng.random_bool(0.5),
            std::cmp::Ordering::Greater => false,
        };

        let length = if horizontal { width } else { height };
        let delta = if horizontal { UVec2::new(1, 0) } else { UVec2::new(0, 1) };
        let dir = if horizontal { Direction::North } else { Direction::East };

        let mut w = UVec2::new(
            x + if horizontal || width == 2 {
                0
            } else {
                self.rng.random_range(0..width - 2)
            },
            y + if horizontal {
                if height == 2 {
                    0
                } else {
                    self.rng.random_range(0..height - 2)
                }
            } else {
                0
            },
        );

        let p = UVec2::new(
            w.x + if horizontal { self.rng.random_range(0..width) } else { 0 },
            w.y + if horizontal {
                0
            } else {
                self.rng.random_range(0..height)
            },
        );

        if horizontal {
            maze.wall_head = p.y * maze.size.x + p.x + (maze.size.x - 1) * maze.size.y;
        } else {
            maze.wall_head = p.y * (maze.size.x - 1) + p.x;
        }

        for _ in 0..length {
            if w != p {
                maze.neighbors.close(w, dir);
            }

            w += delta;
        }

        let (nw, nh) = if horizontal {
            (width, w.y - y + 1)
        } else {
            (w.x - x + 1, height)
        };
        self.first.push(UVec4::new(x, y, nw, nh));

        let (nx, ny) = if horizontal { (x, w.y + 1) } else { (w.x + 1, y) };
        let (nw, nh) = if horizontal {
            (width, y + height - w.y - 1)
        } else {
            (x + width - w.x - 1, height)
        };
        self.second.push(UVec4::new(nx, ny, nw, nh));
    }
}
