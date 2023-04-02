use crate::render_resources::material_library::MaterialLibrary;
use crate::render_resources::mesh_library::MeshLibrary;
use crate::render_resources::shader_library::ShaderLibrary;
use crate::shader::ShaderRef;

pub mod material_library;
pub mod mesh_library;
pub mod shader_library;

pub struct RenderResources {
	pub shader_lib: ShaderLibrary,
	pub material_lib: MaterialLibrary,
	pub mesh_lib: MeshLibrary,
}

impl RenderResources {
	pub fn new(shaders: Vec<ShaderRef>) -> Self {

		let mut shader_lib = ShaderLibrary::new();
		for shader in shaders {
			shader_lib.add(shader);
		}

		Self {
			shader_lib,
			material_lib: MaterialLibrary::new(),
			mesh_lib: MeshLibrary::new(),
		}
	}
}