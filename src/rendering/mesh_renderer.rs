use std::cell::RefCell;
use std::rc::Rc;
use resa_renderer::material::{Material, MaterialRef};
use resa_renderer::ResaRenderer;

pub struct MeshRenderer {
	pub mesh_id: u64,
	pub(crate) material_id: Option<MaterialRef>,
	resa_renderer: Rc<RefCell<ResaRenderer>>,
}

impl MeshRenderer {
	#[allow(dead_code)]
	pub(crate) fn new(mesh: u64, resa_renderer: Rc<RefCell<ResaRenderer>>) -> Self {
		Self {
			mesh_id: mesh,
			material_id: None,
			resa_renderer,
		}
	}

	pub fn set_material(&mut self, material: MaterialRef){
		self.material_id = Some(material);
	}

	pub fn get_material_ref(&self) -> &Option<MaterialRef>{
		&self.material_id
	}

	pub fn get_material(&self) -> Material{
		let mut binding = self.resa_renderer.borrow_mut();
		let mat = binding.get_material_mut(&self.material_id.unwrap());
		mat.clone()
	}

	pub fn update_material(&mut self, new_material: Material){
		let mut binding = self.resa_renderer.borrow_mut();
		binding.update_material(&self.material_id.unwrap(), new_material);
	}
}