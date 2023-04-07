use gfx_hal::window::Extent2D;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::material::{Material, MaterialRef};
use crate::mesh::Mesh;
use crate::graphics_pipeline::PipelineType;
use crate::render_resources::RenderResources;
use crate::renderer::Renderer;
use crate::shader::ShaderRef;

mod renderer;
mod core;
mod framebuffer;
mod swapchain;
mod renderpass;
pub mod graphics_pipeline;
pub mod vertex;
mod buffer;
mod descriptors;
mod uniform;
mod image_buffer;
pub mod mesh;
pub mod material;
mod helper;
pub mod shader;
mod render_resources;


pub struct RendererConfig {
	pub extent: PhysicalSize<u32>,
	pub shaders: Vec<ShaderRef>,
}


pub struct ResaRenderer {
	renderer: Renderer<backend::Backend>,
	render_resources: RenderResources<backend::Backend>
}

impl ResaRenderer {
	/// Create a new instance of the renderer
	pub fn new(window: &Window, config: RendererConfig) -> Self {

		let extent = Extent2D{width: config.extent.width, height: config.extent.height};
		let pipe_types = vec![PipelineType::Opaque];//material_controller.get_registered_pipeline_types();

		let mut renderer = Renderer::new(window, extent);
		let render_resources = RenderResources::new(config.shaders, &renderer);
		for pipeline_type in pipe_types.iter() {
			renderer.create_pipeline(pipeline_type, &render_resources.shader_lib.get_by_id(&0).unwrap());
		}

		ResaRenderer {
			renderer,
			render_resources,
		}
	}

	pub fn register_mesh(&mut self, mesh: Mesh) -> u64 {
		self.render_resources.mesh_lib.add_mesh(mesh)
	}

	pub fn register_materials(&mut self, materials: &[Material]) -> Vec<MaterialRef> {
		self.render_resources.material_lib.add_materials(materials)
	}

	pub fn get_material_mut(&mut self, material_id: &MaterialRef) -> &mut Material{
		self.render_resources.material_lib.material_map_old.get_mut(&material_id).unwrap()
	}

	pub fn update_material(&mut self, material_id: &MaterialRef, material :Material){
		todo!("Implement material update function from material controller here!")
	}

	pub fn register_texture(&mut self, image_data: Vec<u8>) -> usize{
		self.render_resources.material_lib.add_new_texture(image_data, &mut self.renderer)
	}

	/// Refresh the renderers swapchain setting e.g. after a surface size change
	pub fn refresh(&mut self) {
		self.renderer.recreate_swapchain = true;
	}

	/// Render all given meshes to the given output device
	pub fn render(&mut self, render_objects: &[(u64, MaterialRef, [[f32; 4]; 4])], view_mat: [[f32; 4]; 4], projection_mat: [[f32; 4]; 4]) {
		self.renderer.draw(render_objects, view_mat, projection_mat, &self.render_resources);
	}

	pub fn get_fps(&self) -> f32 {
		self.renderer.get_fps()
	}
}
