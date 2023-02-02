extern crate core;

use winit::dpi::PhysicalSize;

use resa_ecs::world::World;
use resa_renderer::{RendererConfig, ResaRenderer};

use crate::camera::Camera;
use crate::rendering::mesh::create_primitive_quad;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::transform::Transform;

mod rendering;
mod window;
mod transform;
mod camera;


fn main() {
	let mut window = match window::Window::new("SkriptR", 640, 480) {
		Some(window) => window,
		None => return,
	};

	let mut world: World = World::new();

	let camera_entity = world.new_entity();
	let camera = Camera::new(45., [0.1, 100.], true);
	let transform = Transform { position: [0.0, 0.0, 5.0] };
	world.add_component(&camera_entity, camera);
	world.add_component(&camera_entity, transform);

	let entity01 = world.new_entity();

	let transform = Transform { position: [0.0, 0.0, 0.0] };
	let mesh_renderer = MeshRenderer { mesh: create_primitive_quad(), color: [0.5, 0.0, 0.0, 1.0] };

	world.add_component(&entity01, transform);
	world.add_component(&entity01, mesh_renderer);


	let entity02 = world.new_entity();
	let transform = Transform { position: [0.8, 0.2, 0.0] };
	let mesh_renderer = MeshRenderer { mesh: create_primitive_quad(), color: [0.0, 0.0, 1.0, 1.0] };
	world.add_component(&entity02, transform);
	world.add_component(&entity02, mesh_renderer);

	// let renderer = RenderingController::new(&window);

	let renderer = ResaRenderer::new(&window.instance, RendererConfig {
		extent: PhysicalSize { width: window.physical_size.width, height: window.physical_size.height },
		vertex_shader_path: "./src/rendering/shaders/base.vert".to_string(),
		fragment_shader_path: "./src/rendering/shaders/base.frag".to_string(),
	});
	/*    if renderer.is_none(){
			println!("Creating renderer failed!");
			return;
		}*/

	println!("{}", usize::MAX);
	window.set_renderer_instance(renderer);
	// println!("Start rendering!");
	window.run_window_loop(world);
}
