use std::cell::RefCell;
use std::rc::Rc;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use resa_ecs::world::World;
use resa_renderer::{RendererConfig, ResaRenderer};
use resa_renderer::material::{Color, Material, MaterialRef, Texture};
use resa_renderer::mesh::Mesh;
use resa_renderer::render_stage::RenderStage;
use crate::rendering::camera::Camera;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::transform::{make_transform_matrix, Transform};
use crate::resources::loaded_resources::LoadedMaterial;
use crate::resources::ResourceManager;

pub mod mesh_renderer;
mod camera_system;
pub mod transform;
pub mod camera;


pub struct RenderingSystem {
	resa_renderer: Rc<RefCell<ResaRenderer>>,
	reconfigure_swapchain: bool,
	frames_drawn: u32,

}

impl RenderingSystem {
	pub fn new(window: &Window, size: PhysicalSize<u32>, resources: &ResourceManager) -> RenderingSystem {

		let shaders = resources.get_shaders();
		let materials = RenderingSystem::load_materials(&resources.get_materials());
		let config = RendererConfig{
			extent: size,
			shaders,
		};
		let mut renderer = ResaRenderer::new(window, config);
		renderer.register_materials(&materials);

		RenderingSystem {
			resa_renderer: Rc::new(RefCell::new(renderer)),
			reconfigure_swapchain: true,
			frames_drawn: 0,
		}
	}

	pub fn set_dirty(&mut self) {
		self.reconfigure_swapchain = true;
	}

	pub fn render(&mut self, world: &Rc<RefCell<World>>) {
		if self.reconfigure_swapchain {
			self.resa_renderer.borrow_mut().refresh();
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
			let mat_id = mesh.get_material_ref().unwrap_or(MaterialRef::default());
			mesh_data.push((mesh.mesh_id, mat_id, transform))
		}

		let (camera, cam_entity) = world_binding.get_all_components_of_type::<Camera>().unwrap()[0];
		let cam_transform = world_binding.get_component::<Transform>(&cam_entity).unwrap();

		let view_matrix = camera_system::get_camera_view_matrix(&cam_transform);
		let proj_matrix = camera_system::get_camera_projection_matrix(&camera);

		self.resa_renderer.borrow_mut().render(&mesh_data, view_matrix, proj_matrix);
		self.frames_drawn += 1;
		if self.frames_drawn % 10 == 0 {
			self.frames_drawn = 0;
			// println!("{}", self.resa_renderer.get_fps());
		}
	}

	pub fn create_mesh_renderer(&mut self, mesh: Mesh) -> MeshRenderer {
		let mesh_id = self.resa_renderer.borrow_mut().register_mesh(mesh);
		MeshRenderer::new(mesh_id, self.resa_renderer.clone())
	}

	fn load_materials(loaded_materials: &[LoadedMaterial]) -> Vec<Material> {
		let materials: Vec<Material> = loaded_materials.iter().map(|loaded_mat|
			Material {
				name: loaded_mat.name.clone(),
				shader_id: loaded_mat.shader.clone() as u32,
				render_stage: RenderStage::get_stage_form_index(loaded_mat.stage),
				color: Color {
					r: loaded_mat.color[0],
					g: loaded_mat.color[1],
					b: loaded_mat.color[2],
					a: loaded_mat.color[3],
				},
				texture: Texture::None /*if loaded_mat.texture.len() == 0 { Texture::None } else {Texture::Pending(loaded_mat.texture)}*/,
			}
		).collect();
		materials

		// self.resa_renderer.borrow_mut().register_materials(&materials)
	}
}
