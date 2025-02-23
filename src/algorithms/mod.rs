use crate::maze::MazeState;

mod dfs;

pub use dfs::DepthFirstSearch;

pub trait Algorithm {
    fn initialize(&mut self, maze: &mut MazeState);

    fn step(&mut self, maze: &mut MazeState);
}

#[derive(Debug, Clone)]
pub enum MazeAlgorithm {
    DepthFirstSearch(DepthFirstSearch),
}

impl Algorithm for MazeAlgorithm {
    fn initialize(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.initialize(maze),
        }
    }

    fn step(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.step(maze),
        }
    }
}
