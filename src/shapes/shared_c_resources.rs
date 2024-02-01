#[repr(C)]
pub struct Vec2 {
    pub(crate) x: f32,
    pub(crate) y: f32,
}
#[repr(C)]
pub struct Vertex {
    pub(crate) pos: Vec2,
    pub(crate) uv: Vec2,
}
