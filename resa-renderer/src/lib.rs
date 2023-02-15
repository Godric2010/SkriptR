use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::window::{Window};
use crate::material::Material;
use crate::material_controller::MaterialController;
use crate::mesh::Mesh;
use crate::mesh_controller::MeshController;
use crate::renderer::Renderer;

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


pub struct RendererConfig {
	pub extent: PhysicalSize<u32>,
	pub vertex_shader_path: String,
	pub fragment_shader_path: String,
}


pub struct ResaRenderer {
	renderer: Renderer<backend::Backend>,
	mesh_controller: MeshController,
	material_controller: MaterialController,
}

impl ResaRenderer {
	/// Create a new instance of the renderer
	pub fn new(window: &Window, config: RendererConfig) -> Self {
		let material_controller = MaterialController::new();
		let pipe_types = material_controller.get_registred_pipeline_types();

		let mut renderer = Renderer::new(window, config);
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

	pub fn register_materials(&mut self, materials: &[Material]) -> Vec<u64> {
		self.material_controller.add_new_materials(materials, &mut self.renderer)
	}

	/// Refresh the renderers swapchain setting e.g. after a surface size change
	pub fn refresh(&mut self) {
		self.renderer.recreate_swapchain = true;
	}

	/// Render all given meshes to the given output device
	pub fn render(&mut self, render_objects: &[(u64, u64, [[f32; 4]; 4])], view_mat: [[f32; 4]; 4], projection_mat: [[f32; 4]; 4]) {
		self.renderer.draw(render_objects, view_mat, projection_mat, &self.mesh_controller, &self.material_controller);
	}

	pub fn get_fps(&self) -> f32 {
		self.renderer.get_fps()
	}
}