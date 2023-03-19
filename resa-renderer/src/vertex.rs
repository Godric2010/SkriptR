use std::mem::size_of;
use gfx_hal::format::Format;
use gfx_hal::pso::{AttributeDesc, Element, VertexBufferDesc, VertexInputRate};
use glm::{Vec2, Vec3};

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
	pub position: Vec3,
	pub uv: Vec2,
}

impl Vertex {
	pub fn get_vertex_buffer_desc() -> Vec<VertexBufferDesc>{
		let vertex_buffers = vec![VertexBufferDesc {
			binding: 0,
			stride: size_of::<Vertex>() as u32,
			rate: VertexInputRate::Vertex,
		}];
		vertex_buffers
	}

	pub fn get_vertex_attributes() -> Vec<AttributeDesc>{
		let attributes = vec![
			AttributeDesc {
				location: 0,
				binding: 0,
				element: Element {
					format: Format::Rgb32Sfloat,
					offset: 0, // Zero bytes form the beginning of the vertex struct to the position value
				},
			},
			AttributeDesc{
				location: 1,
				binding: 0,
				element: Element{
					format: Format::Rg32Sfloat,
					offset: size_of::<Vec3>() as u32,
				}
			},
		];
		attributes
	}
}