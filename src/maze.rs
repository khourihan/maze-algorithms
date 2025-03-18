use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use glam::UVec2;

use crate::direction::{Direction, Directions};

#[derive(Debug, Clone)]
pub struct MazeState {
    pub size: UVec2,
    pub neighbors: Neighbors,
    pub head: UVec2,
    pub wall_head: u32,
    pub finished: bool,
    visited: HashSet<UVec2>,
    finalized: HashSet<UVec2>,
}

impl MazeState {
    #[inline]
    pub fn new(size: UVec2) -> MazeState {
        MazeState {
            head: size,
            finished: false,
            visited: HashSet::new(),
            wall_head: (size.x - 1) * size.y + size.x * (size.y - 1),
            finalized: HashSet::new(),
            neighbors: Neighbors::new(size),
            size,
        }
    }

    #[inline]
    pub fn set_visited(&mut self, cell: UVec2) {
        self.visited.insert(cell);
    }

    #[inline]
    pub fn unset_visited(&mut self, cell: UVec2) {
        self.visited.remove(&cell);
    }

    #[inline]
    pub fn set_finalized(&mut self, cell: UVec2) {
        self.finalized.insert(cell);
    }

    #[inline]
    pub fn unset_finalized(&mut self, cell: UVec2) {
        self.finalized.remove(&cell);
    }

    #[inline]
    pub fn visited(&self, cell: UVec2) -> bool {
        self.visited.contains(&cell)
    }

    #[inline]
    pub fn finalized(&self, cell: UVec2) -> bool {
        self.finalized.contains(&cell)
    }

    #[inline]
    pub fn finish(&mut self) {
        self.finished = true;
        self.head = self.size;
        self.wall_head = self.walls();
    }

    /// Computes the [`Directions`] where the given cell touches the edge of the maze.
    #[inline]
    pub fn edges(&self, cell: UVec2) -> Directions {
        let mut d = Directions::NONE;

        if cell.x == 0 {
            d |= Directions::WEST;
        } else if cell.x == self.size.x - 1 {
            d |= Directions::EAST;
        }

        if cell.y == 0 {
            d |= Directions::SOUTH;
        } else if cell.y == self.size.y - 1 {
            d |= Directions::NORTH;
        }

        d
    }

    #[inline]
    pub fn walls(&self) -> u32 {
        (self.size.x - 1) * self.size.y + self.size.x * (self.size.y - 1)
    }
}

#[derive(Debug, Clone)]
pub struct Neighbors {
    v: Vec<Directions>,
    width: u32,
    height: u32,
}

impl Neighbors {
    pub fn new(size: UVec2) -> Neighbors {
        Neighbors {
            v: vec![Directions::NONE; (size.x * size.y) as usize],
            width: size.x,
            height: size.y,
        }
    }

    pub fn open(&mut self, cell: UVec2, dir: Direction) {
        self[cell] |= dir.into();

        if let Some(other) = dir.checked_offset(cell) {
            if other.x < self.width && other.y < self.height {
                self[other] |= (-dir).into();
            }
        }
    }

    pub fn close(&mut self, cell: UVec2, dir: Direction) {
        self[cell] &= !Directions::from(dir);

        if let Some(other) = dir.checked_offset(cell) {
            if other.x < self.width && other.y < self.height {
                self[other] &= !Directions::from(-dir);
            }
        }
    }
}

impl Index<UVec2> for Neighbors {
    type Output = Directions;

    fn index(&self, index: UVec2) -> &Self::Output {
        &self.v[(index.y * self.width + index.x) as usize]
    }
}

impl IndexMut<UVec2> for Neighbors {
    fn index_mut(&mut self, index: UVec2) -> &mut Self::Output {
        &mut self.v[(index.y * self.width + index.x) as usize]
    }
}
