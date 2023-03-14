use std::cell::RefCell;
use std::rc::Rc;
use winit::window::Window;
use resa_ecs::world::World;
use resa_renderer::{RendererConfig, ResaRenderer};
use resa_renderer::material::Material;
use resa_renderer::mesh::Mesh;
use crate::rendering::camera::Camera;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::transform::{make_transform_matrix, Transform};

pub mod mesh_renderer;
mod camera_system;
pub mod transform;
pub mod camera;


pub struct RenderingSystem {
	resa_renderer: ResaRenderer,
	reconfigure_swapchain: bool,
	frames_drawn: u32,

}

impl RenderingSystem {
	pub fn new(window: &Window, config: RendererConfig) -> RenderingSystem {
		RenderingSystem {
			resa_renderer: ResaRenderer::new(window, config),
			reconfigure_swapchain: true,
			frames_drawn: 0,
		}
	}

	pub fn set_dirty(&mut self) {
		self.reconfigure_swapchain = true;
	}

	pub fn render(&mut self, world: &Rc<RefCell<World>>) {
		if self.reconfigure_swapchain {
			self.resa_renderer.refresh();
			self.reconfigure_swapchain = false;
		}

		let world_binding = world.borrow();
		let meshes = world_binding.get_all_components_of_type::<MeshRenderer>().unwrap();

		let mut mesh_data = vec![];
		for (mesh, entity) in meshes.iter() {
			let transform = if let Some(t) = world.borrow().get_component::<Transform>(&entity) {
				make_transform_matrix(&t)
			} else {
				make_transform_matrix(&Transform::idle())
			};
			let mat_id = mesh.material_id.unwrap_or(0);
			mesh_data.push((mesh.mesh_id, mat_id, transform))
		}

		let (camera, cam_entity) = world_binding.get_all_components_of_type::<Camera>().unwrap()[0];
		let cam_transform = world_binding.get_component::<Transform>(&cam_entity).unwrap();

		let view_matrix = camera_system::get_camera_view_matrix(&cam_transform);
		let proj_matrix = camera_system::get_camera_projection_matrix(&camera);

		self.resa_renderer.render(&mesh_data, view_matrix, proj_matrix);
		self.frames_drawn += 1;
		if self.frames_drawn % 10 == 0 {
			self.frames_drawn = 0;
			// println!("{}", self.resa_renderer.get_fps());
		}
	}

	pub fn load_mesh(&mut self, mesh: Mesh) -> MeshRenderer {
		let mesh_id = self.resa_renderer.register_mesh(mesh);
		MeshRenderer::new(mesh_id)
	}

	pub fn assign_material_to_mesh(&mut self, mesh_renderer: &mut MeshRenderer, material: Material) {
		let material_id = self.resa_renderer.register_materials(&[material])[0];
		mesh_renderer.material_id = Some(material_id);
	}

	pub fn register_texture(&self, bytes: Vec<u8>) -> usize {
		/*self.resa_renderer.*/
		1
	}
}
