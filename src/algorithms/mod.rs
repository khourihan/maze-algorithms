use crate::maze::MazeState;

mod dfs;
mod prim;

pub use dfs::DepthFirstSearch;
pub use prim::Prim;

pub trait Algorithm {
    fn initialize(&mut self, maze: &mut MazeState);

    fn step(&mut self, maze: &mut MazeState);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmLabel {
    DepthFirstSearch,
    Prim,
}

#[derive(Debug, Clone)]
pub enum MazeAlgorithm {
    DepthFirstSearch(DepthFirstSearch),
    Prim(Prim),
}

impl MazeAlgorithm {
    pub fn from_label(label: AlgorithmLabel) -> MazeAlgorithm {
        match label {
            AlgorithmLabel::DepthFirstSearch => MazeAlgorithm::DepthFirstSearch(DepthFirstSearch::new()),
            AlgorithmLabel::Prim => MazeAlgorithm::Prim(Prim::new()),
        }
    }
}

impl Algorithm for MazeAlgorithm {
    fn initialize(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.initialize(maze),
            MazeAlgorithm::Prim(a) => a.initialize(maze),
        }
    }

    fn step(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.step(maze),
            MazeAlgorithm::Prim(a) => a.step(maze),
        }
    }
}
