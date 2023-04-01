use gfx_hal::window::Extent2D;
use winit::dpi::PhysicalSize;
use winit::window::{Window};
use crate::material::{Material, MaterialRef};
use crate::material_controller::MaterialController;
use crate::mesh::Mesh;
use crate::mesh_controller::MeshController;
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
mod mesh_controller;
pub mod material;
mod helper;
mod material_controller;
pub mod shader;


pub struct RendererConfig {
	pub extent: PhysicalSize<u32>,
	pub shaders: Vec<ShaderRef>,
}


pub struct ResaRenderer {
	renderer: Renderer<backend::Backend>,
	mesh_controller: MeshController,
	material_controller: MaterialController,
}

impl ResaRenderer {
	/// Create a new instance of the renderer
	pub fn new(window: &Window, config: RendererConfig) -> Self {

		let extent = Extent2D{width: config.extent.width, height: config.extent.height};
		let material_controller = MaterialController::new(config.shaders);
		let pipe_types = material_controller.get_registered_pipeline_types();

		let mut renderer = Renderer::new(window, extent);
		for pipeline_type in pipe_types {
			renderer.create_pipeline(pipeline_type, &material_controller);
		}

		ResaRenderer {
			renderer,
			mesh_controller: MeshController::new(),
			material_controller,
		}
	}

	pub fn register_mesh(&mut self, mesh: Mesh) -> u64 {
		self.mesh_controller.add_mesh(mesh, &mut self.renderer)
	}

	pub fn register_material(&mut self, material: Material) -> MaterialRef {
		self.material_controller.add_new_material(material, &mut self.renderer)
	}

	pub fn get_material_mut(&mut self, material_id: &MaterialRef) -> &mut Material{
		self.material_controller.material_map.get_mut(&material_id).unwrap()
	}

	pub fn update_material(&mut self, material_id: &MaterialRef, material :Material){
		// TODO: Implement material update function from material controller here!
	}

	pub fn register_texture(&mut self, image_data: Vec<u8>) -> usize{
		self.material_controller.add_new_texture(image_data, &mut self.renderer)
	}

	/// Refresh the renderers swapchain setting e.g. after a surface size change
	pub fn refresh(&mut self) {
		self.renderer.recreate_swapchain = true;
	}

	/// Render all given meshes to the given output device
	pub fn render(&mut self, render_objects: &[(u64, MaterialRef, [[f32; 4]; 4])], view_mat: [[f32; 4]; 4], projection_mat: [[f32; 4]; 4]) {
		self.renderer.draw(render_objects, view_mat, projection_mat, &self.mesh_controller, &self.material_controller);
	}

	pub fn get_fps(&self) -> f32 {
		self.renderer.get_fps()
	}
}
