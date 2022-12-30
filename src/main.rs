extern crate core;


use resa_ecs::world::World;
use crate::camera::Camera;
use crate::rendering::mesh::create_primitive_quad;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::RenderingController;
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
    let camera = Camera::new(45., [0.1,100.], true);
    let transform = Transform{ position: [0.0,0.0, 5.0]};
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

    let renderer = RenderingController::new(&window);
    /*    if renderer.is_none(){
            println!("Creating renderer failed!");
            return;
        }*/

    window.set_renderer_instance(renderer);
    println!("Start rendering!");
    window.run_window_loop(world);
}
