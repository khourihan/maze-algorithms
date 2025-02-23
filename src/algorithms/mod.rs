use crate::maze::MazeState;

mod dfs;

pub use dfs::DepthFirstSearch;

pub trait Algorithm {
    fn initialize(&mut self, maze: &mut MazeState);

    fn step(&mut self, maze: &mut MazeState);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmLabel {
    DepthFirstSearch,
}

#[derive(Debug, Clone)]
pub enum MazeAlgorithm {
    DepthFirstSearch(DepthFirstSearch),
}

impl MazeAlgorithm {
    pub fn from_label(label: AlgorithmLabel) -> MazeAlgorithm {
        match label {
            AlgorithmLabel::DepthFirstSearch => MazeAlgorithm::DepthFirstSearch(DepthFirstSearch::new()),
        }
    }
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
