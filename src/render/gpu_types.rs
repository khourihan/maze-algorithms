use bytemuck::{Pod, Zeroable};
use glam::Vec2;

#[repr(C)]
#[derive(Clone, Copy)]
pub struct View {
    pub position: Vec2,
    pub scale: f32,
    pub x: u16,
    pub y: u16,
}

unsafe impl Pod for View {}
unsafe impl Zeroable for View {}

#[repr(C)]
#[derive(Clone, Copy)]
pub struct Rect {
    pub min: Vec2,
    pub max: Vec2,
    pub color: [u8; 4],
}

unsafe impl Pod for Rect {}
unsafe impl Zeroable for Rect {}

impl Rect {
    const ATTRIBS: [wgpu::VertexAttribute; 3] = wgpu::vertex_attr_array![0 => Float32x2, 1 => Float32x2, 2 => Unorm8x4];

    pub(super) fn desc<'a>() -> wgpu::VertexBufferLayout<'a> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Rect>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &Self::ATTRIBS,
        }
    }
}
