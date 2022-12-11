use winit::dpi::PhysicalSize;
use resa_ecs::world::World;
use crate::rendering::mesh::Mesh;
use crate::rendering::renderer::Renderer;
use crate::window::Window;

mod renderer;
mod commands;
mod pass;
mod pipeline;
pub mod mesh;
mod buffers;
mod push_constants;
pub mod mesh_renderer;
mod camera_system;


pub struct RenderingController {
    renderer_instance: Renderer<backend::Backend>,
}

impl RenderingController {
    pub fn new(window: &Window) -> Self {
        Self {
            renderer_instance: Renderer::new(&window.name, &window.physical_size, &window
                .instance).unwrap()
        }
    }

    pub fn add_mesh_to_renderer(&mut self, mesh : &Mesh){
        self.renderer_instance.register_mesh_vertex_buffer(mesh);
    }

    pub fn reconfigure_swapchain(&mut self, surface_size: &PhysicalSize<u32>) {
        self.renderer_instance.recreate_swapchain(surface_size);
    }

    pub fn render(&mut self, world: &mut World) {
        self.renderer_instance.render(world);
    }
}