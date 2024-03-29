extern crate core;

use std::rc::Rc;
use resa_renderer::mesh::{create_primitive_cube, create_primitive_quad, create_primitive_triangle};

use rendering::camera::Camera;
use rendering::transform::Transform;

mod rendering;
mod resa_app;
mod event;
mod test_anim;
mod resources;


fn main() {
	let mut app = match resa_app::ResaApp::new("SkriptR", 640, 360) {
		Some(window) => window,
		None => return,
	};

	let world = Rc::clone(&app.world);

	let camera_entity = world.borrow_mut().new_entity();
	let camera = Camera::new(45., [0.1, 100.], true);
	let transform = Transform { position: [0.0, 0.0, 5.0], angle: 0.0, scale: 1.0 };
	world.borrow_mut().add_component(&camera_entity, camera);
	world.borrow_mut().add_component(&camera_entity, transform);

	/*let material = Material {
		name: "Material 01".to_string(),
		shader_id: 0,
		render_stage: RenderStage::Transparent,
		color: Color::new(125, 125, 125, 150),
		texture: Texture::None,
	};

	let material02 = Material {
		name: "Material 02".to_string(),
		shader_id: 0,
		render_stage: RenderStage::Opaque,
		color: Color::new(0, 0, 0, 255),
		texture: Texture::None,
	};

	let material03 = Material {
		name: "Material 03".to_string(),
		shader_id: 0,
		render_stage: RenderStage::Opaque,
		color: Color::new(255, 255, 255, 255),
		texture: Texture::None//Pending(wood_tex.0, wood_tex.1),//wood_tex,
	};*/

	// let (font_pixels, size) = font_library.get_font_atlas_by_name("Arial").unwrap();
	// let material04 = Material{
	// 	name: "Material 04".to_string(),
	// 	shader_id: 0,
	// 	render_stage: RenderStage::Transparent,
	// 	color: Color::new(0,0,0,255),
	// 	texture: Texture::Pending(font_pixels, TextureFormat::Custom(size)),
	// };

	// let materials = app.rendering.load_materials(&vec![material, material02, material03, /*material04*/]);

	let entity01 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0., 0., 0.0], angle: 0.0, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_triangle());
	mesh_renderer.set_material("material01");
	world.borrow_mut().add_component(&entity01, transform);
	world.borrow_mut().add_component(&entity01, mesh_renderer);

	let entity03 = world.borrow_mut().new_entity();
	let transform = Transform { position: [-0.2, 0., -1.0], angle: 0.0, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_triangle());
	mesh_renderer.set_material("material02");
	world.borrow_mut().add_component(&entity03, transform);
	world.borrow_mut().add_component(&entity03, mesh_renderer);


	let entity02 = world.borrow_mut().new_entity();
	let transform = Transform { position: [0.8, 0.2, 0.0], angle: 0.0, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_quad());
	mesh_renderer.set_material("material03");
	world.borrow_mut().add_component(&entity02, transform);
	world.borrow_mut().add_component(&entity02, mesh_renderer);

	let entity04 = world.borrow_mut().new_entity();
	let transform = Transform { position: [-1.1, 1.0, 0.2], angle: 0.3, scale: 1.0 };
	let mut mesh_renderer = app.rendering.create_mesh_renderer(create_primitive_cube());
	mesh_renderer.set_material("material03");
	world.borrow_mut().add_component(&entity04, transform);
	world.borrow_mut().add_component(&entity04, mesh_renderer);

	app.run_window_loop();
}
