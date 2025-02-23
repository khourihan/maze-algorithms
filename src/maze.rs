use std::{
    collections::HashSet,
    ops::{Index, IndexMut},
};

use glam::UVec2;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Directions: u8 {
        const EAST = 1 << 0;
        const NORTH = 1 << 1;
        const WEST = 1 << 2;
        const SOUTH = 1 << 3;
    }
}

#[derive(Debug, Clone)]
pub struct MazeState {
    pub size: UVec2,
    pub neighbors: Neighbors,
    pub head: Option<u32>,
    visited: HashSet<u32>,
    finalized: HashSet<u32>,
}

impl MazeState {
    #[inline]
    pub fn new(size: UVec2) -> MazeState {
        MazeState {
            head: None,
            visited: HashSet::new(),
            finalized: HashSet::new(),
            neighbors: Neighbors::new(size),
            size,
        }
    }

    #[inline]
    pub fn set_visited(&mut self, cell: UVec2) {
        self.visited.insert(cell.y * self.size.x + cell.x);
    }

    #[inline]
    pub fn unset_visited(&mut self, cell: UVec2) {
        self.visited.remove(&(cell.y * self.size.x + cell.x));
    }

    #[inline]
    pub fn set_finalized(&mut self, cell: UVec2) {
        self.finalized.insert(cell.y * self.size.x + cell.x);
    }

    #[inline]
    pub fn unset_finalized(&mut self, cell: UVec2) {
        self.finalized.remove(&(cell.y * self.size.x + cell.x));
    }

    #[inline]
    pub fn visited(&self, cell: UVec2) -> bool {
        self.visited.contains(&(cell.y * self.size.x + cell.x))
    }

    #[inline]
    pub fn finalized(&self, cell: UVec2) -> bool {
        self.finalized.contains(&(cell.y * self.size.x + cell.x))
    }
}

#[derive(Debug, Clone)]
pub struct Neighbors {
    v: Vec<Directions>,
    width: u32,
}

impl Neighbors {
    pub fn new(size: UVec2) -> Neighbors {
        Neighbors {
            v: vec![Directions::empty(); (size.x * size.y) as usize],
            width: size.x,
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
