use std::hash::{Hash, Hasher};
use glm::{Vector2, Vector3};
use crate::vertex::Vertex;

// #[derive(Copy, Clone)]
pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub indices: Vec<u16>,
}

impl Mesh {
	pub fn new(vertices: Vec<Vertex>, indices: Vec<u16>) -> Self {
		Mesh {
			vertices,
			indices,
		}
	}
}

impl Hash for Mesh {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.vertices.len().hash(state);
		self.indices.hash(state);
	}
}

pub fn create_primitive_quad() -> Mesh {
	let vertices = vec![
		Vertex { position: Vector3::new(-1.0, -1.0, 0.0), uv: Vector2::new(0.0, 0.0) },
		Vertex { position: Vector3::new(-1.0, 1.0, 0.0), uv: Vector2::new(0.0, 0.0) },
		Vertex { position: Vector3::new(1.0, 1.0, 0.0), uv: Vector2::new(0.0, 0.0) },
		Vertex { position: Vector3::new(1.0, -1.0, 0.0), uv: Vector2::new(0.0, 0.0) },
	];
	let triangle_list = vec![0, 1, 2, 0, 2, 3];
	Mesh::new(vertices, triangle_list)
}

pub fn create_primitive_triangle() -> Mesh {
	let vertices = vec![
		Vertex { position: Vector3::new(-0.5, -0.5, 0.0), uv: Vector2::new(0.0, 0.0) },
		Vertex { position: Vector3::new(0.0, 0.5, 0.0), uv: Vector2::new(0.0, 0.0) },
		Vertex { position: Vector3::new(0.5, -0.5, 0.0), uv: Vector2::new(0.0, 0.0) },
	];
	let triangle_list = vec![0, 1, 2];
	Mesh::new(vertices, triangle_list)
}




