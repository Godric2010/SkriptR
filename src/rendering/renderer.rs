use std::mem::ManuallyDrop;
use gfx_hal::adapter::{Adapter, PhysicalDevice};
use gfx_hal::Instance;
use gfx_hal::queue::{QueueFamily, QueueGroup};
use gfx_hal::window::{Extent2D, Surface};
use winit::dpi::PhysicalSize;
use crate::rendering::commands::CommandBufferController;
use crate::rendering::pass::RenderPass;
use crate::rendering::pipeline::GraphicsPipeline;

pub struct Renderer<B: gfx_hal::Backend> {
    instance: ManuallyDrop<B::Instance>,
    surface: ManuallyDrop<B::Surface>,
    surface_extent: Extent2D,
    adapter: Adapter<B>,
    device: ManuallyDrop<B::Device>,
    queue_group: QueueGroup<B>,
    command_buffer_controller: CommandBufferController<B>,
    render_passes: Vec<RenderPass<B>>,
    graphics_pipelines: Vec<GraphicsPipeline<B>>,
}

impl<B: gfx_hal::Backend> Renderer<B> {
    pub fn new(name: &str, surface_size: &PhysicalSize<u32>, window: &winit::window::Window) -> Option<Self> {

        // Create the backend instance
        let instance_result = Instance::create(name, 1);
        if instance_result.is_err() {
            println!("Creating an instance failed due to unsupported backend!");
            return None;
        }
        let instance: B::Instance = instance_result.unwrap();

        // Create the surface to render on
        let surface_result = unsafe { instance.create_surface(&window) };
        if surface_result.is_err() {
            println!("Failed to create surface!");
            return None;
        }
        let surface = surface_result.unwrap();
        let surface_extent = Extent2D {
            width: surface_size.width,
            height: surface_size.height,
        };

        // Get physical device and its data
        let mut adapters = instance.enumerate_adapters();
        let adapter = adapters.remove(0);
        let queue_family_result = adapter.queue_families.iter().find(|family| {
            surface.supports_queue_family(family) && family.queue_type().supports_graphics()
        });

        if queue_family_result.is_none() {
            println!("No compatible queue found!");
            return None;
        }
        let queue_family = queue_family_result.unwrap();

        let gpu_result = unsafe {
            adapter.physical_device.open(&[(queue_family, &[1.0])], gfx_hal::Features::empty())
        };

        if gpu_result.is_err() {
            println!("Failed to open device!");
            return None;
        }
        let mut gpu = gpu_result.unwrap();

        let device: B::Device = gpu.device;
        let queue_group_result = gpu.queue_groups.pop();
        if queue_group_result.is_none() {
            println!("Failed to receive queue group!");
            return None;
        }
        let queue_group = queue_group_result.unwrap();

        // create command buffer pool and alloc commands
        let command_buffer_controller_result = CommandBufferController::new(&device, queue_group.family, 1);
        if command_buffer_controller_result.is_none() {
            println!("Failed to create command pool!");
            return None;
        }
        let command_buffer_controller = command_buffer_controller_result.unwrap();

        // create a simple render pass
        let render_pass_result = RenderPass::new(&adapter, &device, &surface);
        if render_pass_result.is_none() {
            println!("Failed to create render pass");
            return None;
        }
        let mut render_pass = render_pass_result.unwrap();

        // create a basic pipeline
        let pipeline_result = GraphicsPipeline::new(&device, &mut render_pass);
        if pipeline_result.is_none() {
            println!("failed to create pipeline!");
            return None;
        }
        let graphics_pipeline = pipeline_result.unwrap();


        Some(Self {
            instance: ManuallyDrop::new(instance),
            surface: ManuallyDrop::new(surface),
            surface_extent,
            adapter,
            device: ManuallyDrop::new(device),
            queue_group,
            command_buffer_controller,
            render_passes: vec![render_pass],
            graphics_pipelines: vec![graphics_pipeline],

        })
    }
}

impl<B: gfx_hal::Backend> Drop for Renderer<B> {
    fn drop(&mut self) {
        // self.device.wait_idle().unwrap()
        unsafe {
            for graphics_pipeline in self.graphics_pipelines.iter_mut() {
                graphics_pipeline.destroy(&self.device);
            }

            let amount_of_render_passes = self.render_passes.len();
            for _ in 0..amount_of_render_passes{
                let rp = self.render_passes.pop().unwrap();
                rp.destroy(&self.device);
            }

            self.command_buffer_controller.destroy(&self.device);

            let surface = ManuallyDrop::take(&mut self.surface);
            self.instance.destroy_surface(surface);
        }
    }
}