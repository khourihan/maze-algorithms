use std::{
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use algorithms::{Algorithm, DepthFirstSearch, MazeAlgorithm};
use glam::{UVec2, Vec2};
use maze::MazeState;
use renderer::MazeRenderer;
use winit::event_loop::{ControlFlow, EventLoop};

mod algorithms;
mod direction;
mod input;
mod maze;
mod render;
mod renderer;

const START_FRAME_TIME_US: u64 = 65536;
const START_MAZE_SIZE: UVec2 = UVec2::splat(16);

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut maze = MazeState::new(START_MAZE_SIZE);
    let mut algorithm = MazeAlgorithm::DepthFirstSearch(DepthFirstSearch::new());

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

            if let Some(micros) = renderer::FRAME_TIME.lock().unwrap().take() {
                frame_time = Duration::from_micros(micros);
            }

            if let Some(size) = renderer::MAZE_SIZE.lock().unwrap().take() {
                maze = MazeState::new(size);
                algorithm.initialize(&mut maze);
            }
        }
    });

    let renderer = MazeRenderer {
        pos: Vec2::ZERO,
        scale: 0.5,
        maze: MazeState::new(START_MAZE_SIZE),
        maze_size: START_MAZE_SIZE,
        frame_time_us: START_FRAME_TIME_US,
        info_window_open: false,
        wall_width: 0.3,
    };

    let mut app = render::App::new(renderer);

    event_loop.run_app(&mut app).unwrap();
}
