use std::mem::ManuallyDrop;
use std::{iter, ptr};
use std::borrow::Borrow;
use gfx_hal::adapter::{Adapter, PhysicalDevice};
use gfx_hal::command::{ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, RenderAttachmentInfo, SubpassContents};
use gfx_hal::device::Device;
use gfx_hal::image::Extent;
use gfx_hal::Instance;
use gfx_hal::pso::{Rect, Viewport};
use gfx_hal::queue::{Queue, QueueFamily, QueueGroup};
use gfx_hal::window::{Extent2D, PresentationSurface, Surface, SwapchainConfig};
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
    submission_complete_fence: ManuallyDrop<B::Fence>,
    rendering_complete_semaphore: ManuallyDrop<B::Semaphore>,
    framebuffer: ManuallyDrop<B::Framebuffer>,
    viewport: Viewport,
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


        let submission_complete_fence_result = device.create_fence(true);
        if submission_complete_fence_result.is_err() {
            println!("Failed to create fence! Out of memory!");
            return None;
        }
        let submission_complete_fence = submission_complete_fence_result.unwrap();

        let rendering_complete_semaphore_result = device.create_semaphore();
        if rendering_complete_semaphore_result.is_err() {
            println!("Failed to create semaphore! Out of memory!");
            return None;
        }
        let rendering_complete_semaphore = rendering_complete_semaphore_result.unwrap();


        let caps = surface.capabilities(&adapter.physical_device);
        let swap_config = SwapchainConfig::from_caps(&caps, render_pass.color_format, surface_extent);
        let fat = swap_config.framebuffer_attachment();

        let framebuffer = ManuallyDrop::new(unsafe {
            device.create_framebuffer(&*render_pass.pass, iter::once(fat), Extent {
                width: surface_extent.width,
                height: surface_extent.height,
                depth: 1
            }).unwrap()
        });

        let viewport = Viewport {
            rect: Rect {
                x: 0,
                y: 0,
                w: surface_extent.width as _,
                h: surface_extent.height as _,
            },
            depth: 0.0..1.0,
        };

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
            submission_complete_fence: ManuallyDrop::new(submission_complete_fence),
            rendering_complete_semaphore: ManuallyDrop::new(rendering_complete_semaphore),
            framebuffer,
            viewport,
        })
    }

    pub fn recreate_swapchain(&mut self, new_surface_size: &PhysicalSize<u32>) {
        self.surface_extent = Extent2D {
            width: new_surface_size.width,
            height: new_surface_size.height
        };

        let capabilities = self.surface.capabilities(&self.adapter.physical_device);
        let mut swapchain_config = SwapchainConfig::from_caps(&capabilities, self.render_passes[0].color_format, self.surface_extent);

        // Fixes some fullscreen slowdowns on macOS
        if capabilities.image_count.contains(&3) {
            swapchain_config.image_count = 3;
        }

        let swap_extent = swapchain_config.extent.to_extent();
        self.viewport.rect.w = swap_extent.width as _;
        self.viewport.rect.h = swap_extent.height as _;

        unsafe {
            self.device.wait_idle().unwrap();

            self.device.destroy_framebuffer(ManuallyDrop::into_inner(ptr::read(&self.framebuffer)));


            let graphics_render_pass = &self.render_passes[0].pass;

            let framebuffer =
                self.device.create_framebuffer(&graphics_render_pass, iter::once(swapchain_config.framebuffer_attachment()), swap_extent).unwrap();
            self.framebuffer = ManuallyDrop::new(framebuffer);
        }

        let res = unsafe { self.surface.configure_swapchain(&self.device, swapchain_config) };
        if res.is_err() {
            println!("Failed to recreate swapchain!")
        }
    }

    pub fn render(&mut self) {
        let size = PhysicalSize {
            width: self.surface_extent.width,
            height: self.surface_extent.height,
        };

        let surface_image = unsafe {
            match self.surface.acquire_image(!0) {
                Ok((image, _)) => image,
                Err(_) => {
                    self.recreate_swapchain(&size);
                    return;
                }
            }
        };

        let mut graphics_command_buffer = &mut self.command_buffer_controller.graphics_buffer;
        unsafe {
            graphics_command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
            graphics_command_buffer.set_viewports(0, iter::once(self.viewport.clone()));
            graphics_command_buffer.set_scissors(0, iter::once(self.viewport.rect));
            graphics_command_buffer.bind_graphics_pipeline(&self.graphics_pipelines[0].pipeline);

            graphics_command_buffer.begin_render_pass(
                &self.render_passes[0].pass,
                &self.framebuffer,
                self.viewport.rect,
                iter::once(RenderAttachmentInfo {
                    image_view: surface_image.borrow(),
                    clear_value: ClearValue {
                        color: ClearColor {
                            float32: [1.0, 1.0, 1.0, 1.0],
                        },
                    },
                }),
                SubpassContents::Inline,
            );
            graphics_command_buffer.draw(0..3, 0..1);
            graphics_command_buffer.end_render_pass();
            graphics_command_buffer.finish();
        };

        unsafe {
            self.queue_group.queues[0].submit(
                iter::once(&*graphics_command_buffer),
                iter::empty(),
                iter::once(&*self.rendering_complete_semaphore),
                Some(&mut self.submission_complete_fence)
            );

            let result = self.queue_group.queues[0].present(
                &mut self.surface,
                surface_image,
                Some(&mut self.rendering_complete_semaphore)
            );

            if result.is_err() {
                self.recreate_swapchain(&size);
            }
        }
    }
}

impl<B: gfx_hal::Backend> Drop for Renderer<B> {
    fn drop(&mut self) {
        // self.device.wait_idle().unwrap()
        unsafe {
            let rendering_complete_semaphore = ManuallyDrop::take(&mut self.rendering_complete_semaphore);
            self.device.destroy_semaphore(rendering_complete_semaphore);

            let submission_complete_fence = ManuallyDrop::take(&mut self.submission_complete_fence);
            self.device.destroy_fence(submission_complete_fence);

            for graphics_pipeline in self.graphics_pipelines.iter_mut() {
                graphics_pipeline.destroy(&self.device);
            }

            let amount_of_render_passes = self.render_passes.len();
            for _ in 0..amount_of_render_passes {
                let rp = self.render_passes.pop().unwrap();
                rp.destroy(&self.device);
            }

            self.command_buffer_controller.destroy(&self.device);

            let surface = ManuallyDrop::take(&mut self.surface);
            self.instance.destroy_surface(surface);
        }
    }
}