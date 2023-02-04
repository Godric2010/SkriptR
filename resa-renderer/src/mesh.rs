use std::hash::{Hash, Hasher};
use crate::vertex::Vertex;

pub struct Mesh {
	pub vertices: Vec<Vertex>,
	pub triangles: Vec<[i32; 3]>,
}

impl Mesh {
	pub fn new(vertices: Vec<Vertex>, tris: Vec<[i32; 3]>) -> Self {
		Mesh {
			vertices,
			triangles: tris,
		}
	}
}

impl Hash for Mesh {
	fn hash<H: Hasher>(&self, state: &mut H) {
		self.vertices.len().hash(state);
		self.triangles.hash(state);
	}
}

pub fn create_primitive_quad() -> Mesh {
	let vertices = vec![
		Vertex { position: [-0.1, -0.1, 0.0], uv: [0.0, 0.0] },
		Vertex { position: [-0.1, 0.1, 0.0], uv: [0.0, 1.0] },
		Vertex { position: [0.1, 0.1, 0.0], uv: [1.0, 1.0] },
		Vertex { position: [-0.1, -0.1, 0.0], uv: [0.0, 0.0] },
		Vertex { position: [0.1, 0.1, 0.0], uv: [1.0, 1.0] },
		Vertex { position: [0.1, -0.1, 0.0], uv: [1.0, 0.0] },
	];
	let triangle_list = vec![[0, 1, 2], [0, 2, 3]];
	Mesh::new(vertices, triangle_list)
}

pub fn create_primitive_triangle() -> Mesh {
	let vertices = vec![
		Vertex { position: [-0.5, -0.5, 0.0], uv: [0.0, 0.0] },
		Vertex { position: [0.0, 0.5, 0.0], uv: [0.5, 1.0] },
		Vertex { position: [0.5, -0.5, 0.0], uv: [1.0, 0.0] },
	];
	let triangle_list = vec![[0, 1, 2]];
	Mesh::new(vertices, triangle_list)
}




