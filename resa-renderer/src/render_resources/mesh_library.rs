use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::adapter::MemoryType;
use gfx_hal::buffer::Usage;
use crate::buffer::Buffer;
use crate::core::CoreDevice;
use crate::mesh::Mesh;
use crate::vertex::Vertex;

pub(crate) struct MeshEntry<B: Backend> {
	mesh: Mesh,
	mesh_hash: u64,
	pub(crate) instances: u64,
	pub(crate) vertex_buffer: Buffer<B>,
	pub(crate) index_buffer: Buffer<B>,
}

pub struct MeshLibrary<B: Backend> {
	pub(crate) mesh_map: HashMap<u64, MeshEntry<B>>,
	last_entry: u64,
	device_ptr: Rc<RefCell<CoreDevice<B>>>,
	memory_types: Vec<MemoryType>,
}

impl<B: Backend> MeshLibrary<B> {
	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>) -> Self {
		MeshLibrary {
			mesh_map: HashMap::new(),
			last_entry: 0,
			device_ptr,
			memory_types,
		}
	}

	pub fn add_mesh(&mut self, mesh: Mesh) -> u64 {
		let mut hasher = DefaultHasher::new();
		mesh.hash(&mut hasher);
		let mesh_hash = hasher.finish();

		let mesh_id = match self.get_mesh_id(&mesh_hash) {
			Some(id) => {
				let entry = self.mesh_map.get_mut(&id).unwrap();
				entry.instances += 1;
				id
			}
			None => {
				let (vertex, index) = self.create_buffers(&mesh);
				let entry = MeshEntry {
					mesh,
					mesh_hash,
					instances: 1,
					vertex_buffer: vertex,
					index_buffer: index,
				};
				let entry_id = self.last_entry + 1;
				self.mesh_map.insert(entry_id, entry);
				entry_id
			}
		};

		mesh_id
	}

	pub fn update_mesh(&mut self, mesh_id: &u64, new_mesh: Mesh) {
		let mut entry = match self.mesh_map.get_mut(mesh_id) {
			Some(entry) => entry,
			None => {
				println!("Try to remove mesh with id {}, but this does not exists!", mesh_id);
				return;
			}
		};
		let mut hasher = DefaultHasher::new();
		new_mesh.hash(&mut hasher);
		let mesh_hash = hasher.finish();

		entry.vertex_buffer.update_data(0, &new_mesh.vertices);
		entry.index_buffer.update_data(0, &new_mesh.indices);
		entry.mesh_hash = mesh_hash;
	}

	pub fn remove_mesh(&mut self, mesh_id: &u64) {
		let mut entry = match self.mesh_map.get_mut(mesh_id) {
			Some(entry) => entry,
			None => {
				println!("Try to remove mesh with id {}, but this does not exists!", mesh_id);
				return;
			}
		};

		entry.instances -= 1;
		if entry.instances > 0 { return; }

		self.mesh_map.remove(mesh_id);
	}

	pub(crate) fn get_mesh_entry(&self, mesh_id: &u64) -> &MeshEntry<B> {
		self.mesh_map.get(mesh_id).unwrap()
	}

	pub(crate) fn get_mesh_index_amount(&self, &mesh_id: &u64) -> u32 {
		self.mesh_map.get(&mesh_id).unwrap().mesh.indices.len() as u32
	}

	fn get_mesh_id(&self, mesh_hash: &u64) -> Option<u64> {
		let result = self.mesh_map.iter().find(|mesh| &mesh.1.mesh_hash == mesh_hash)?.0;
		Some(result.clone())
	}

	fn create_buffers(&self, mesh: &Mesh) -> (Buffer<B>, Buffer<B>) {
		let vertex_buffer = Buffer::new::<Vertex>(
			Rc::clone(&self.device_ptr),
			&mesh.vertices,
			Usage::VERTEX,
			&self.memory_types,
		);

		let index_buffer = Buffer::new::<u16>(
			Rc::clone(&self.device_ptr),
			&mesh.indices,
			Usage::INDEX,
			&self.memory_types,
		);

		(vertex_buffer, index_buffer)
	}
}