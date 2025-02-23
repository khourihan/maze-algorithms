use std::{
    collections::HashSet,
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use algorithms::{Algorithm, AlgorithmLabel, MazeAlgorithm};
use glam::{UVec2, Vec2};
use maze::MazeState;
use renderer::MazeRenderer;
use winit::event_loop::{ControlFlow, EventLoop};

mod algorithms;
mod astar;
mod direction;
mod input;
mod maze;
mod render;
mod renderer;

const START_FRAME_TIME_US: u64 = 65536;
const START_MAZE_SIZE: UVec2 = UVec2::splat(16);
const START_ALGORITHM: AlgorithmLabel = AlgorithmLabel::Kruskal;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut maze_size = START_MAZE_SIZE;
    let mut algorithm_label = START_ALGORITHM;

    let mut path = HashSet::new();
    let mut maze = MazeState::new(maze_size);
    let mut algorithm = MazeAlgorithm::from_label(algorithm_label);

    algorithm.initialize(&mut maze);

    thread::spawn(move || {
        let mut start = Instant::now();
        let mut frame_time = Duration::from_micros(START_FRAME_TIME_US);

        loop {
            if renderer::PAUSED.load(Ordering::Relaxed) || frame_time.checked_sub(start.elapsed()).is_some() {
                thread::yield_now();
            } else if !maze.finished {
                algorithm.step(&mut maze);
                start = Instant::now();
            }

            let mut lock = renderer::UPDATE_LOCK.lock().unwrap();
            {
                let mut lock = renderer::MAZE_STATE.lock().unwrap();
                *lock = maze.clone();
            }
            *lock |= true;

            let mut update_path = false;

            if let Some(micros) = renderer::FRAME_TIME.lock().unwrap().take() {
                frame_time = Duration::from_micros(micros);
            }

            if let Some(size) = renderer::MAZE_SIZE.lock().unwrap().take() {
                maze_size = size;

                maze = MazeState::new(maze_size);
                algorithm = MazeAlgorithm::from_label(algorithm_label);
                algorithm.initialize(&mut maze);
                path.clear();
                update_path |= true;
            }

            if let Some(label) = renderer::MAZE_ALGORITHM.lock().unwrap().take() {
                algorithm_label = label;

                maze = MazeState::new(maze_size);
                algorithm = MazeAlgorithm::from_label(algorithm_label);
                algorithm.initialize(&mut maze);
                path.clear();
                update_path |= true;
            }

            if let Some((start, goal)) = renderer::MAZE_START_GOAL.lock().unwrap().take() {
                if let Some((shortest_path, _)) = astar::astar(start, goal, &maze) {
                    path = shortest_path;
                } else {
                    path.clear();
                }

                update_path |= true;
            }

            if update_path {
                let mut lock = renderer::PATH_LOCK.lock().unwrap();
                {
                    let mut lock = renderer::MAZE_PATH.lock().unwrap();
                    *lock = path.clone();
                }
                *lock |= true;
            }
        }
    });

    let renderer = MazeRenderer {
        pos: Vec2::ZERO,
        scale: 0.5,
        maze: MazeState::new(START_MAZE_SIZE),
        maze_size: START_MAZE_SIZE,
        frame_time_us: START_FRAME_TIME_US,
        algorithm: START_ALGORITHM,
        info_window_open: false,
        wall_width: 0.3,
        selected_start: None,
        selected_goal: None,
        path: HashSet::new(),
    };

    let mut app = render::App::new(renderer);

    event_loop.run_app(&mut app).unwrap();
}
