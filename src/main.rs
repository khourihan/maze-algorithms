use std::{
    sync::atomic::Ordering,
    thread,
    time::{Duration, Instant},
};

use glam::UVec2;
use maze::{Directions, MazeState};
use renderer::MazeRenderer;
use winit::event_loop::{ControlFlow, EventLoop};

mod input;
mod maze;
mod render;
mod renderer;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut maze = MazeState::new(UVec2::splat(16));

    maze.neighbors[UVec2::new(4, 4)].insert(Directions::EAST);
    maze.neighbors[UVec2::new(5, 4)].insert(Directions::WEST);
    maze.neighbors[UVec2::new(5, 4)].insert(Directions::NORTH);
    maze.neighbors[UVec2::new(5, 5)].insert(Directions::SOUTH);
    maze.neighbors[UVec2::new(5, 5)].insert(Directions::NORTH);
    maze.neighbors[UVec2::new(5, 6)].insert(Directions::SOUTH);

    let frame_time = Duration::from_micros(16667);

    thread::spawn(move || loop {
        let start = Instant::now();

        let mut lock = renderer::UPDATE_LOCK.lock().unwrap();
        {
            let mut lock = renderer::MAZE_STATE.lock().unwrap();
            *lock = maze.clone();
        }
        *lock |= true;

        let runtime = start.elapsed();

        if let Some(remaining) = frame_time.checked_sub(runtime) {
            thread::sleep(remaining);
        }
    });

    let renderer = MazeRenderer::default();
    let mut app = render::App::new(renderer);

    event_loop.run_app(&mut app).unwrap();
}
