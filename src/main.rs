extern crate core;

use std::borrow::{Borrow, BorrowMut};
use winit::dpi::PhysicalSize;

use resa_ecs::world::World;
use resa_renderer::{RendererConfig, ResaRenderer};

use crate::camera::Camera;
use crate::rendering::mesh::create_primitive_quad;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::transform::Transform;

mod rendering;
mod resa_app;
mod transform;
mod camera;


fn main() {
	let app = match resa_app::ResaApp::new("SkriptR", 640, 480) {
		Some(window) => window,
		None => return,
	};

	let world  = &*app.world.clone();//app.borrow().world.borrow_mut();

	let camera_entity = world.borrow_mut().new_entity();
	let camera = Camera::new(45., [0.1, 100.], true);
	let transform = Transform { position: [0.0, 0.0, 5.0] };
	world.borrow_mut().add_component(&camera_entity, camera);
	world.borrow_mut().add_component(&camera_entity, transform);

	let entity01 = world.borrow_mut().new_entity();

	let transform = Transform { position: [0.0, 0.0, 0.0] };
	// let mesh_renderer = MeshRenderer { mesh: create_primitive_quad(), color: [0.5, 0.0, 0.0, 1.0] };

	world.borrow_mut().add_component(&entity01, transform);
	// world.add_component(&entity01, mesh_renderer);


	let entity02 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0.8, 0.2, 0.0] };
	// let mesh_renderer = MeshRenderer { mesh: create_primitive_quad(), color: [0.0, 0.0, 1.0, 1.0] };
	world.borrow_mut().add_component(&entity02, transform);
	// world.add_component(&entity02, mesh_renderer);

	app.run_window_loop();
}
