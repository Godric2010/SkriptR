use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::mesh::Mesh;
use crate::renderer::Renderer;

pub struct MeshController{
	pub mesh_map: HashMap<u64, Mesh>,
	pub(crate) buffer_map: HashMap<u64, usize>,
}

impl MeshController {
	pub fn new() -> Self{
		MeshController{
			mesh_map: HashMap::new(),
			buffer_map: HashMap::new(),
		}
	}

	pub(crate) fn add_mesh(&mut self, mesh: Mesh, renderer: &mut Renderer<backend::Backend>) -> u64{
		let mut hasher = DefaultHasher::new();
		mesh.hash(&mut hasher);
		let mesh_hash = hasher.finish();

		self.mesh_map.insert(mesh_hash.clone(), mesh);
		let buffer_index =renderer.add_vertex_and_index_buffer(&self.mesh_map.get(&mesh_hash).unwrap());
		self.buffer_map.insert(mesh_hash.clone(), buffer_index);

		mesh_hash
	}

	pub(crate) fn get_mesh_data(&self, mesh_id: &u64) -> (usize, u32){
		let amount_of_indices = self.mesh_map.get(&mesh_id).unwrap().indices.len() as u32;
		let buffer_id= self.buffer_map.get(&mesh_id).unwrap().clone();
		(buffer_id, amount_of_indices)
	}


}