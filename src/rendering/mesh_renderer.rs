use resa_renderer::material::{Material, MaterialRef};

pub struct MeshRenderer {
	pub mesh_id: u64,
	pub material_id: Option<MaterialRef>,
	dirty: bool,
}

impl MeshRenderer {
	#[allow(dead_code)]
	pub fn new(mesh: u64) -> Self {
		Self {
			mesh_id: mesh,
			material_id: None,
			dirty: false,
		}
	}

	pub fn get_material_mut(&self) -> &mut Material{
		todo!("Implement get material function here!")
	}

	pub fn set_dirty(&mut self){
		self.dirty = true;
	}
}