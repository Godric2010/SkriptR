use gfx_hal::Backend;
use crate::render_resources::material_library::MaterialLibrary;
use crate::render_resources::mesh_library::MeshLibrary;
use crate::render_resources::shader_library::ShaderLibrary;
use crate::renderer::Renderer;
use crate::shader::ShaderRef;

pub mod material_library;
pub mod mesh_library;
pub mod shader_library;
pub mod texture_buffer_library;
mod uniform_buffer_library;


pub struct RenderResources<B: Backend> {
	pub shader_lib: ShaderLibrary,
	pub material_lib: MaterialLibrary<B>,
	pub mesh_lib: MeshLibrary<B>,
}

impl<B:Backend> RenderResources<B> {
	pub fn new(shaders: Vec<ShaderRef>, renderer: &Renderer<B>) -> Self {

		let mut shader_lib = ShaderLibrary::new();
		for shader in shaders {
			shader_lib.add(shader);
		}

		Self {
			shader_lib,
			material_lib: MaterialLibrary::new(renderer.get_device(), renderer.get_memory_types(), renderer.get_adapter_limits()),
			mesh_lib: MeshLibrary::new(renderer.get_device(), renderer.get_memory_types()),
		}
	}
}