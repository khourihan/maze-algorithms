use std::{
    collections::HashSet,
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
    maze::{Directions, MazeState},
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
    maze: MazeState,
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
                mem::swap(&mut self.maze, &mut MAZE_STATE.lock().unwrap());
            }
            *lock = false;
        }

        ctx.set_view_pos(self.pos);
        ctx.set_view_scale(self.scale);
        ctx.set_clear_color(WALL_COLOR);
        ctx.clear_rects();

        let inv_maze_size = self.maze.size.as_vec2().recip();
        let cell_size = (1.0 - self.wall_width) * inv_maze_size;
        let cell_offset = self.wall_width * 0.5 * inv_maze_size;
        let wall_size = self.wall_width * inv_maze_size;
        let wall_offset = (1.0 - 0.5 * self.wall_width) * inv_maze_size;

        let mut open_walls_x = HashSet::new();
        let mut open_walls_y = HashSet::new();

        for y in 0..self.maze.size.y {
            for x in 0..self.maze.size.x {
                let cell = UVec2::new(x, y);

                let min = cell.as_vec2() * inv_maze_size + cell_offset;
                let max = min + cell_size;

                let neighbors = self.maze.neighbors[cell];

                if neighbors.contains(Directions::EAST) {
                    open_walls_x.insert(cell);
                }

                if neighbors.contains(Directions::NORTH) {
                    open_walls_y.insert(cell);
                }

                let mut color = if self.maze.finalized(cell) {
                    FINALIZED_COLOR
                } else if self.maze.visited(cell) {
                    VISITED_COLOR
                } else {
                    CELL_COLOR
                };

                if let Some(head) = self.maze.head {
                    if y * self.maze.size.x + x == head {
                        color = HEAD_COLOR;
                    }
                }

                ctx.draw_rect(min - 0.5, max - 0.5, color);
            }
        }

        for wall in open_walls_x {
            let west = wall;
            let east = wall + UVec2::new(1, 0);

            let min = wall.as_vec2() * inv_maze_size + Vec2::new(wall_offset.x, cell_offset.y);
            let max = min + Vec2::new(wall_size.x, cell_size.y);

            let color = if self.maze.finalized(west) && self.maze.finalized(east) {
                FINALIZED_COLOR
            } else if self.maze.visited(west) && self.maze.visited(east) {
                VISITED_COLOR
            } else {
                CELL_COLOR
            };

            ctx.draw_rect(min, max, color);
        }

        for wall in open_walls_y {
            let south = wall;
            let north = wall + UVec2::new(0, 1);

            let min = wall.as_vec2() * inv_maze_size + Vec2::new(cell_offset.x, wall_offset.y);
            let max = min + Vec2::new(cell_size.x, wall_size.y);

            let color = if self.maze.finalized(south) && self.maze.finalized(north) {
                FINALIZED_COLOR
            } else if self.maze.visited(south) && self.maze.visited(north) {
                VISITED_COLOR
            } else {
                CELL_COLOR
            };

            ctx.draw_rect(min, max, color);
        }
    }

    fn gui(&mut self, ctx: &Context) {}
}

impl Default for MazeRenderer {
    fn default() -> Self {
        Self {
            pos: Vec2::ZERO,
            scale: 0.5,
            maze: MazeState::new(UVec2::splat(16)),
            info_window_open: false,
            wall_width: 0.3,
        }
    }
}
