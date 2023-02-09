
pub struct MeshRenderer {
	pub mesh_id: u64,
	pub material_id: u64,
}

impl MeshRenderer {
	#[allow(dead_code)]
	pub fn new(mesh: u64, material: u64) -> Self {
		Self {
			mesh_id: mesh,
			material_id: material,
		}
	}
}