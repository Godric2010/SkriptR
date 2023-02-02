use winit::dpi::PhysicalSize;
use winit::window::{Window};
use crate::renderer::Renderer;

mod renderer;
mod core;
mod framebuffer;
mod swapchain;
mod renderpass;
mod graphics_pipeline;
mod vertex;
mod buffer;
mod descriptors;
mod uniform;
mod image_buffer;

pub struct RendererConfig {
    pub extent: PhysicalSize<u32>,
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,
}

pub struct ResaRenderer{
    renderer: Renderer<backend::Backend>
}

impl ResaRenderer {

    /// Create a new instance of the renderer
    pub fn new(window: &Window, config: RendererConfig)->Self{
        ResaRenderer{
            renderer: Renderer::new(window, config),
        }
    }

    /// Refresh the renderers swapchain setting e.g. after a surface size change
    pub fn refresh(&mut self){
        self.renderer.recreate_swapchain = true;


    }

    /// Render all given meshes to the given output device
    pub fn render(&mut self){
       self.renderer.draw();
    }

    pub fn get_fps(&self) -> f32{
        self.renderer.get_fps()
    }
}