use std::thread;

use glam::UVec2;
use maze::MazeState;
use renderer::MazeRenderer;
use winit::event_loop::{ControlFlow, EventLoop};

mod input;
mod maze;
mod render;
mod renderer;

fn main() {
    let event_loop = EventLoop::new().unwrap();

    event_loop.set_control_flow(ControlFlow::Poll);

    let mut maze_state = MazeState::new(UVec2::splat(16));

    thread::spawn(move || {
        let mut lock = renderer::UPDATE_LOCK.lock().unwrap();
        {
            let mut lock = renderer::MAZE_STATE.lock().unwrap();
            *lock = maze_state
        }
        *lock |= true;
    });

    let renderer = MazeRenderer::default();
    let mut app = render::App::new(renderer);

    event_loop.run_app(&mut app).unwrap();
}
