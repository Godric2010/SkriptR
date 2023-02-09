extern crate core;

use resa_renderer::mesh::{create_primitive_quad, create_primitive_triangle};

use crate::camera::Camera;
use crate::transform::Transform;

mod rendering;
mod resa_app;
mod transform;
mod camera;


fn main() {
	let mut app = match resa_app::ResaApp::new("SkriptR", 640, 480) {
		Some(window) => window,
		None => return,
	};

	let world = &*app.world.clone();//app.borrow().world.borrow_mut();

	let camera_entity = world.borrow_mut().new_entity();
	let camera = Camera::new(45., [0.1, 100.], true);
	let transform = Transform { position: [0.0, 0.0, 5.0] };
	world.borrow_mut().add_component(&camera_entity, camera);
	world.borrow_mut().add_component(&camera_entity, transform);

	let entity01 = world.borrow_mut().new_entity();

	let transform = Transform { position: [0.0, 0.0, 0.0] };
	let mesh_renderer = app.load_mesh(create_primitive_triangle());

	world.borrow_mut().add_component(&entity01, transform);
	world.borrow_mut().add_component(&entity01, mesh_renderer);


	let entity02 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0.8, 0.2, 0.0] };
	let mesh_renderer = app.load_mesh(create_primitive_quad());
	world.borrow_mut().add_component(&entity02, transform);
	world.borrow_mut().add_component(&entity02, mesh_renderer);

	app.run_window_loop();
}
