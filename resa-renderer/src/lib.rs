use winit::dpi::PhysicalSize;
use winit::window::{Window, WindowBuilder};
use crate::renderer::Renderer;

mod renderer;
mod core;
mod framebuffer;
mod swapchain;

pub struct ResaRenderer{
    renderer: Renderer<backend::Backend>
}

impl ResaRenderer {

    /// Create a new instance of the renderer
    pub fn new(window: &Window, extent: &PhysicalSize<u32>)->Self{
        ResaRenderer{
            renderer: Renderer::new(window, extent),
        }
    }

    /// Refresh the renderers swapchain setting e.g. after a surface size change
    pub fn refresh(){

    }

    /// Render all given meshes to the given output device
    pub fn render(){

    }
}