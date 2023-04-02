use crate::render_resources::material_controller::MaterialController;
use crate::render_resources::shader_library::ShaderLibrary;
use crate::shader::ShaderRef;

pub mod material_controller;
pub mod mesh_controller;
pub mod shader_library;

pub struct RenderResources {
	pub shader_lib: ShaderLibrary,
	pub material_lib: MaterialController,
}

impl RenderResources {
	pub fn new(shaders: Vec<ShaderRef>) -> Self {

		let mut shader_lib = ShaderLibrary::new();
		for shader in shaders {
			shader_lib.add(shader);
		}

		Self {
			shader_lib,
			material_lib: MaterialController::new(),
		}
	}
}