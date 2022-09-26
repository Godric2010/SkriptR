use winit::dpi::PhysicalSize;
use crate::rendering::mesh::Mesh;
use crate::rendering::renderer::Renderer;
use crate::window::Window;

mod renderer;
mod commands;
mod pass;
mod pipeline;
pub mod mesh;


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

    pub fn reconfigure_swapchain(&mut self, surface_size: &PhysicalSize<u32>) {
        self.renderer_instance.recreate_swapchain(surface_size);
    }

    pub fn render(&mut self, meshes: &[Mesh]) {
        self.renderer_instance.render(meshes);
    }
}