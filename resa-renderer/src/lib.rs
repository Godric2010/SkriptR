use winit::dpi::PhysicalSize;
use winit::window::{Window};
use crate::mesh::Mesh;
use crate::mesh_controller::MeshController;
use crate::renderer::Renderer;

mod renderer;
mod core;
mod framebuffer;
mod swapchain;
mod renderpass;
mod graphics_pipeline;
pub mod vertex;
mod buffer;
mod descriptors;
mod uniform;
mod image_buffer;
pub mod mesh;
mod mesh_controller;
pub mod material;

pub struct RendererConfig {
    pub extent: PhysicalSize<u32>,
    pub vertex_shader_path: String,
    pub fragment_shader_path: String,
}

pub struct ResaRenderer{
    renderer: Renderer<backend::Backend>,
    mesh_controller: MeshController,
}

impl ResaRenderer {

    /// Create a new instance of the renderer
    pub fn new(window: &Window, config: RendererConfig)->Self{
        ResaRenderer{
            renderer: Renderer::new(window, config),
            mesh_controller: MeshController::new(),
        }
    }

    pub fn register_mesh(&mut self, mesh: Mesh)-> u64{
        self.mesh_controller.add_mesh(mesh,&mut self.renderer)
    }

    /// Refresh the renderers swapchain setting e.g. after a surface size change
    pub fn refresh(&mut self){
        self.renderer.recreate_swapchain = true;


    }

    /// Render all given meshes to the given output device
    pub fn render(&mut self, mesh_ids: &[u64]){
       self.renderer.draw(mesh_ids, &self.mesh_controller);
    }

    pub fn get_fps(&self) -> f32{
        self.renderer.get_fps()
    }
}