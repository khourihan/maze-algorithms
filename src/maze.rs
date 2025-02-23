use std::collections::HashSet;

use glam::UVec2;

bitflags::bitflags! {
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub struct Directions: u8 {
        const NORTH = 1 << 0;
        const EAST = 1 << 1;
        const SOUTH = 1 << 2;
        const WEST = 1 << 3;
    }
}

pub struct MazeState {
    pub size: UVec2,
    head: Option<u32>,
    visited: HashSet<u32>,
    finalized: HashSet<u32>,
    neighbors: Vec<Directions>,
}

impl MazeState {
    #[inline]
    pub fn new(size: UVec2) -> MazeState {
        MazeState {
            head: None,
            visited: HashSet::new(),
            finalized: HashSet::new(),
            neighbors: vec![Directions::empty(); (size.x * size.y) as usize],
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
}
