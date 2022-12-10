extern crate core;


use resa_ecs::world::World;
use crate::rendering::mesh::create_primitive_quad;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::RenderingController;
use crate::transform::Transform;

mod rendering;
mod window;
mod transform;


fn main() {
    let mut window = match window::Window::new("SkriptR", 512, 512){
        Some(window) => window,
        None => return,
    };

    let mut world: World = World::new();
    let entity01 = world.new_entity();

    let transform = Transform{position: [0.0,0.0,0.0]};

    world.add_component(entity01, transform);

    let renderer = RenderingController::new(&window);
/*    if renderer.is_none(){
        println!("Creating renderer failed!");
        return;
    }*/

    window.set_renderer_instance(renderer);
    window.run_window_loop(&mut world);
    
}
