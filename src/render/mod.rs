use egui_wgpu::ScreenDescriptor;
use glam::Vec2;
use gpu_types::Rect;
use state::State;
use std::sync::Arc;
use winit::application::ApplicationHandler;
use winit::dpi::PhysicalSize;
use winit::event::WindowEvent;
use winit::event_loop::ActiveEventLoop;
use winit::window::{Window, WindowId};

use crate::input::InputManager;

mod gpu_types;
mod gui;
mod state;

pub struct RenderContext {
    pos: Vec2,
    scale: f32,
    rects: Vec<Rect>,
    clear_color: [f32; 4],
}

impl RenderContext {
    pub fn new() -> Self {
        Self {
            pos: Vec2::ZERO,
            scale: 1.0,
            rects: Vec::new(),
            clear_color: [0.0; 4],
        }
    }

    pub fn set_view_pos(&mut self, pos: Vec2) {
        self.pos = pos;
    }

    pub fn set_view_scale(&mut self, scale: f32) {
        self.scale = scale;
    }

    pub fn set_clear_color(&mut self, color: [f32; 4]) {
        self.clear_color = color;
    }

    pub fn clear_rects(&mut self) {
        self.rects.clear();
    }

    pub fn draw_rect(&mut self, min: Vec2, max: Vec2, color: [u8; 4]) {
        self.rects.push(Rect { min, max, color });
    }
}

pub trait Renderer {
    fn input(&mut self, input: &InputManager, width: u16, height: u16);

    fn render(&mut self, ctx: &mut RenderContext);

    fn gui(&mut self, ctx: &egui::Context);
}

pub struct App<R: Renderer> {
    width: u32,
    height: u32,
    instance: wgpu::Instance,
    state: Option<State>,
    window: Option<Arc<Window>>,
    input: InputManager,
    render_ctx: RenderContext,
    renderer: R,
}

impl<R: Renderer> App<R> {
    pub fn new(renderer: R) -> Self {
        let instance = wgpu::Instance::new(&wgpu::InstanceDescriptor::default());
        Self {
            instance,
            width: 1360,
            height: 768,
            state: None,
            window: None,
            input: InputManager::new(),
            render_ctx: RenderContext::new(),
            renderer,
        }
    }

    async fn set_window(&mut self, window: Window) {
        let window = Arc::new(window);

        let _ = window.request_inner_size(PhysicalSize::new(self.width, self.height));

        let surface = self
            .instance
            .create_surface(window.clone())
            .expect("failed to create surface.");

        let state = State::new(&self.instance, surface, &window, self.width, self.height).await;

        self.window.get_or_insert(window);
        self.state.get_or_insert(state);
    }

    fn handle_resized(&mut self, width: u32, height: u32) {
        if width > 0 && height > 0 {
            self.state.as_mut().unwrap().resize_surface(width, height);
        }
    }

    fn handle_redraw(&mut self) -> Result<(), wgpu::SurfaceError> {
        if let Some(window) = self.window.as_ref() {
            if let Some(min) = window.is_minimized() {
                if min {
                    println!("window is minimized");
                    return Ok(());
                }
            }
        }

        let state = self.state.as_mut().unwrap();

        let screen_descriptor = ScreenDescriptor {
            size_in_pixels: [state.surface_config.width, state.surface_config.height],
            pixels_per_point: self.window.as_ref().unwrap().scale_factor() as f32 * state.scale_factor,
        };

        let surface_texture = state.surface.get_current_texture()?;

        let surface_view = surface_texture
            .texture
            .create_view(&wgpu::TextureViewDescriptor::default());

        let mut encoder = state
            .device
            .create_command_encoder(&wgpu::CommandEncoderDescriptor { label: None });

        let window = self.window.as_ref().unwrap();

        let clipped_primitives = state.gui.prepare(
            &state.device,
            &state.queue,
            window,
            &mut encoder,
            &screen_descriptor,
            |ctx| self.renderer.gui(ctx),
        );

        {
            let mut pass = encoder.begin_render_pass(&wgpu::RenderPassDescriptor {
                label: Some("render_pass"),
                color_attachments: &[Some(wgpu::RenderPassColorAttachment {
                    view: &surface_view,
                    resolve_target: None,
                    ops: wgpu::Operations {
                        load: wgpu::LoadOp::Clear(wgpu::Color {
                            r: self.render_ctx.clear_color[0] as f64 * 0.1,
                            g: self.render_ctx.clear_color[1] as f64 * 0.1,
                            b: self.render_ctx.clear_color[2] as f64 * 0.1,
                            a: self.render_ctx.clear_color[3] as f64 * 0.1,
                        }),
                        store: wgpu::StoreOp::Store,
                    },
                })],
                depth_stencil_attachment: None,
                timestamp_writes: None,
                occlusion_query_set: None,
            });

            pass.set_pipeline(&state.rect_render_pipeline);
            pass.set_bind_group(0, &state.view_bind_group, &[]);
            pass.set_vertex_buffer(0, state.rect_buffer.slice(..));
            pass.draw(0..4, 0..state.rects);

            state
                .gui
                .renderer
                .render(&mut pass.forget_lifetime(), &clipped_primitives, &screen_descriptor);
        }

        state.queue.submit(std::iter::once(encoder.finish()));
        surface_texture.present();

        Ok(())
    }
}

impl<R: Renderer> ApplicationHandler for App<R> {
    fn resumed(&mut self, event_loop: &ActiveEventLoop) {
        let window = event_loop.create_window(Window::default_attributes()).unwrap();
        pollster::block_on(self.set_window(window));
    }

    fn device_event(
        &mut self,
        _event_loop: &ActiveEventLoop,
        _device_id: winit::event::DeviceId,
        event: winit::event::DeviceEvent,
    ) {
        self.input.process_device_event(&event);
    }

    fn window_event(&mut self, event_loop: &ActiveEventLoop, _: WindowId, event: WindowEvent) {
        if !self
            .state
            .as_mut()
            .unwrap()
            .gui
            .handle_input(self.window.as_ref().unwrap(), &event)
        {
            self.input.process_window_event(&event);
        }

        match event {
            WindowEvent::CloseRequested => {
                event_loop.exit();
            },
            WindowEvent::RedrawRequested => {
                let view = &self.state.as_ref().unwrap().view;
                self.renderer.input(&self.input, view.x, view.y);
                self.renderer.render(&mut self.render_ctx);

                let state = self.state.as_mut().unwrap();

                state.view.position = self.render_ctx.pos;
                state.view.scale = self.render_ctx.scale;

                state.set_rects(&self.render_ctx.rects);

                state
                    .queue
                    .write_buffer(&state.view_buffer, 0, bytemuck::cast_slice(&[state.view]));

                match self.handle_redraw() {
                    Err(wgpu::SurfaceError::Lost) => self.handle_resized(self.width, self.height),
                    Err(wgpu::SurfaceError::OutOfMemory) => event_loop.exit(),
                    Err(e) => eprintln!("{:?}", e),
                    Ok(_) => (),
                }

                self.window.as_ref().unwrap().request_redraw();
            },
            WindowEvent::Resized(new_size) => {
                self.handle_resized(new_size.width, new_size.height);
            },
            _ => (),
        }
    }

    fn new_events(&mut self, _event_loop: &ActiveEventLoop, _cause: winit::event::StartCause) {
        self.input.step();
    }
}
