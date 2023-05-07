extern crate core;

use std::rc::Rc;
use resa_renderer::mesh::{create_primitive_cube, create_primitive_quad, create_primitive_triangle};

use rendering::camera::Camera;
use rendering::transform::Transform;
use resa_renderer::material::{Color, Material, Texture};
use resa_renderer::render_passes_and_pipelines::RenderStage;

mod rendering;
mod resa_app;
mod resource_loader;
mod Event;
mod test_anim;


fn main() {
	let mut app = match resa_app::ResaApp::new("SkriptR", 640, 360) {
		Some(window) => window,
		None => return,
	};

	let world = Rc::clone(&app.world);//app.borrow().world.borrow_mut();
	let wood_tex = app.resource_loader.load_image("Wood.png").unwrap();

	let camera_entity = world.borrow_mut().new_entity();
	let camera = Camera::new(45., [0.1, 100.], true);
	let transform = Transform { position: [0.0, 0.0, 5.0], angle: 0.0, scale: 1.0 };
	world.borrow_mut().add_component(&camera_entity, camera);
	world.borrow_mut().add_component(&camera_entity, transform);

	let material = Material {
		shader_id: 1,
		render_stage: RenderStage::Transparent,
		color: Color::new(255, 0, 0, 150),
		texture: Texture::None,
	};

	let material02 = Material {
		shader_id: 1,
		render_stage: RenderStage::Opaque,
		color: Color::new(0, 0, 0, 255),
		texture: Texture::None,
	};

	let material03 = Material {
		shader_id: 1,
		render_stage: RenderStage::Opaque,
		color: Color::new(255, 255, 255, 255),
		texture: Texture::Pending(wood_tex),//wood_tex,
	};

	let materials = app.rendering.load_materials(&vec![material, material02, material03]);

	let entity01 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0., 0., 0.0], angle: 0.0, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_triangle());
	mesh_renderer.set_material(materials[0]);
	world.borrow_mut().add_component(&entity01, transform);
	world.borrow_mut().add_component(&entity01, mesh_renderer);

	let entity03 = world.borrow_mut().new_entity();
	let transform = Transform { position: [-0.2, 0., -1.0], angle: 0.0, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_triangle());
	mesh_renderer.set_material(materials[1]);
	world.borrow_mut().add_component(&entity03, transform);
	world.borrow_mut().add_component(&entity03, mesh_renderer);


	let entity02 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0.8, 0.2, 0.0], angle: 0.0, scale: 0.2 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_quad());
	mesh_renderer.set_material(materials[1]);
	world.borrow_mut().add_component(&entity02, transform);
	world.borrow_mut().add_component(&entity02, mesh_renderer);

	let entity04 = world.borrow_mut().new_entity();
	let transform = Transform { position: [-1.1, 1.0, 0.2], angle: 0.3, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_cube());
	mesh_renderer.set_material(materials[2]);
	world.borrow_mut().add_component(&entity04, transform);
	world.borrow_mut().add_component(&entity04, mesh_renderer);

	app.run_window_loop();
}
