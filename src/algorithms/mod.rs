use crate::maze::MazeState;

mod dfs;
mod eller;
mod growing_tree;
mod kruskal;
mod prim;
mod recursive_division;
mod sidewinder;

pub use dfs::DepthFirstSearch;
pub use eller::Eller;
pub use growing_tree::GrowingTree;
pub use kruskal::Kruskal;
pub use prim::Prim;
pub use recursive_division::RecursiveDivision;
pub use sidewinder::Sidewinder;

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
    Sidewinder,
    RecursiveDivision,
}

#[derive(Debug, Clone)]
pub enum MazeAlgorithm {
    DepthFirstSearch(DepthFirstSearch),
    Prim(Prim),
    GrowingTree(GrowingTree),
    Kruskal(Kruskal),
    Eller(Eller),
    Sidewinder(Sidewinder),
    RecursiveDivision(RecursiveDivision),
}

impl MazeAlgorithm {
    pub fn from_label(label: AlgorithmLabel) -> MazeAlgorithm {
        match label {
            AlgorithmLabel::DepthFirstSearch => MazeAlgorithm::DepthFirstSearch(DepthFirstSearch::new()),
            AlgorithmLabel::Prim => MazeAlgorithm::Prim(Prim::new()),
            AlgorithmLabel::GrowingTree => MazeAlgorithm::GrowingTree(GrowingTree::new()),
            AlgorithmLabel::Kruskal => MazeAlgorithm::Kruskal(Kruskal::new()),
            AlgorithmLabel::Eller => MazeAlgorithm::Eller(Eller::new()),
            AlgorithmLabel::Sidewinder => MazeAlgorithm::Sidewinder(Sidewinder::new()),
            AlgorithmLabel::RecursiveDivision => MazeAlgorithm::RecursiveDivision(RecursiveDivision::new()),
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
            MazeAlgorithm::Sidewinder(a) => a.initialize(maze),
            MazeAlgorithm::RecursiveDivision(a) => a.initialize(maze),
        }
    }

    fn step(&mut self, maze: &mut MazeState) {
        match self {
            MazeAlgorithm::DepthFirstSearch(a) => a.step(maze),
            MazeAlgorithm::Prim(a) => a.step(maze),
            MazeAlgorithm::GrowingTree(a) => a.step(maze),
            MazeAlgorithm::Kruskal(a) => a.step(maze),
            MazeAlgorithm::Eller(a) => a.step(maze),
            MazeAlgorithm::Sidewinder(a) => a.step(maze),
            MazeAlgorithm::RecursiveDivision(a) => a.step(maze),
        }
    }
}
