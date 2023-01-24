#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	position: [f32; 3],
	uv: [f32; 2],
}
