use winit::dpi::PhysicalSize;
use crate::rendering::renderer::Renderer;
use crate::window::Window;

mod renderer;
mod commands;
mod pass;
mod pipeline;


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
        println!("Reconfigure sawpchain!");
    }

    pub fn render(&self) {
        // println!("Render!");
        // self.renderer_instance.render();
    }
}