use std::{
    mem,
    sync::{
        atomic::{AtomicBool, Ordering},
        Mutex,
    },
};

use egui::Context;
use glam::{UVec2, Vec2};
use once_cell::sync::Lazy;
use winit::{event::MouseButton, keyboard::KeyCode};

use crate::{
    input::InputManager,
    maze::MazeState,
    render::{RenderContext, Renderer},
};

pub static PAUSED: Lazy<AtomicBool> = Lazy::new(|| true.into());
pub static UPDATE_LOCK: Lazy<Mutex<bool>> = Lazy::new(|| Mutex::new(false));
pub static MAZE_STATE: Lazy<Mutex<MazeState>> = Lazy::new(|| Mutex::new(MazeState::new(UVec2::splat(16))));

const WALL_COLOR: [f32; 4] = [0.101_960_786, 0.098_039_227, 0.098_039_227, 1.0];
const CELL_COLOR: [u8; 4] = [77, 77, 78, 255];
const VISITED_COLOR: [u8; 4] = [0, 116, 117, 255];
const FINALIZED_COLOR: [u8; 4] = [3, 147, 158, 255];
const HEAD_COLOR: [u8; 4] = [236, 129, 2, 255];

pub struct MazeRenderer {
    pos: Vec2,
    scale: f32,
    wall_width: f32,
    maze_state: MazeState,
    info_window_open: bool,
}

impl Renderer for MazeRenderer {
    fn input(&mut self, input: &InputManager, width: u16, height: u16) {
        self.info_window_open ^= input.key_pressed(KeyCode::KeyT);

        if input.key_pressed(KeyCode::Space) {
            PAUSED.fetch_not(Ordering::Relaxed);
        }

        if let Some((mx, my)) = input.cursor() {
            let steps = 5.0;
            let zoom = (-input.scroll_diff().1 / steps).exp2();

            let target = Vec2::new(mx * 2.0 - width as f32, height as f32 - my * 2.0) / height as f32;

            self.pos += target * self.scale * (1.0 - zoom);
            self.scale *= zoom;
        }

        if input.mouse_held(MouseButton::Left) {
            let (mdx, mdy) = input.mouse_diff();
            self.pos.x -= mdx / height as f32 * self.scale * 2.0;
            self.pos.y += mdy / height as f32 * self.scale * 2.0;
        }
    }

    fn render(&mut self, ctx: &mut RenderContext) {
        {
            let mut lock = UPDATE_LOCK.lock().unwrap();
            if *lock {
                mem::swap(&mut self.maze_state, &mut MAZE_STATE.lock().unwrap());
            }
            *lock = false;
        }

        ctx.set_view_pos(self.pos);
        ctx.set_view_scale(self.scale);
        ctx.set_clear_color(WALL_COLOR);
        ctx.clear_rects();

        let cell_size = (1.0 - self.wall_width) / self.maze_state.size.as_vec2();
        let cell_offset = self.wall_width / (2.0 * self.maze_state.size.as_vec2());

        for y in 0..self.maze_state.size.y {
            for x in 0..self.maze_state.size.x {
                let cell = UVec2::new(x, y);

                let min = cell.as_vec2() / self.maze_state.size.as_vec2() + cell_offset;
                let max = min + cell_size;

                ctx.draw_rect(min, max, CELL_COLOR);
            }
        }
    }

    fn gui(&mut self, ctx: &Context) {}
}

impl Default for MazeRenderer {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            scale: 16.0,
            maze_state: MazeState::new(UVec2::splat(16)),
            info_window_open: false,
            wall_width: 0.2,
        }
    }
}
