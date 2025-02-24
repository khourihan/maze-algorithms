use crate::maze::MazeState;

mod dfs;
mod eller;
mod growing_tree;
mod kruskal;
mod prim;

pub use dfs::DepthFirstSearch;
pub use eller::Eller;
pub use growing_tree::GrowingTree;
pub use kruskal::Kruskal;
pub use prim::Prim;

pub trait Algorithm {
    fn initialize(&mut self, maze: &mut MazeState);

    fn step(&mut self, maze: &mut MazeState);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AlgorithmLabel {
    DepthFirstSearch,
    Prim,
    GrowingTree,
    Kruskal,
    Eller,
}

#[derive(Debug, Clone)]
pub enum MazeAlgorithm {
    DepthFirstSearch(DepthFirstSearch),
    Prim(Prim),
    GrowingTree(GrowingTree),
    Kruskal(Kruskal),
    Eller(Eller),
}

impl MazeAlgorithm {
    pub fn from_label(label: AlgorithmLabel) -> MazeAlgorithm {
        match label {
            AlgorithmLabel::DepthFirstSearch => MazeAlgorithm::DepthFirstSearch(DepthFirstSearch::new()),
            AlgorithmLabel::Prim => MazeAlgorithm::Prim(Prim::new()),
            AlgorithmLabel::GrowingTree => MazeAlgorithm::GrowingTree(GrowingTree::new()),
            AlgorithmLabel::Kruskal => MazeAlgorithm::Kruskal(Kruskal::new()),
            AlgorithmLabel::Eller => MazeAlgorithm::Eller(Eller::new()),
        }
    }
}

impl Algorithm for MazeAlgorithm {
    fn initialize(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.initialize(maze),
            MazeAlgorithm::Prim(a) => a.initialize(maze),
            MazeAlgorithm::GrowingTree(a) => a.initialize(maze),
            MazeAlgorithm::Kruskal(a) => a.initialize(maze),
            MazeAlgorithm::Eller(a) => a.initialize(maze),
        }
    }

    fn step(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.step(maze),
            MazeAlgorithm::Prim(a) => a.step(maze),
            MazeAlgorithm::GrowingTree(a) => a.step(maze),
            MazeAlgorithm::Kruskal(a) => a.step(maze),
            MazeAlgorithm::Eller(a) => a.step(maze),
        }
    }
}
