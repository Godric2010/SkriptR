use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::pso::Viewport;
use gfx_hal::window::Extent2D;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::core::{Core, CoreDevice};
use crate::framebuffer::FramebufferData;
use crate::graphics_pipeline::GraphicsPipeline;
use crate::renderpass::RenderPass;
use crate::swapchain::Swapchain;

pub(crate) struct Renderer<B: Backend> {
    core: Core<B>,
    device: Rc<RefCell<CoreDevice<B>>>,
    swapchain: Swapchain,
    render_pass: RenderPass<B>,
    framebuffer_data: FramebufferData<B>,
    viewport: Viewport,
    pipeline: GraphicsPipeline<B>,
}

impl<B: Backend> Renderer<B> {
    pub(crate) fn new(window: &Window, extent: &PhysicalSize<u32>) -> Self {

        // Create the connection between code and gpu.
        let mut core = Core::<B>::create(&window).unwrap();
        let device = Rc::new(RefCell::new(CoreDevice::<B>::new(core.adapter.adapter.take().unwrap(), &core.surface)));

        // Create buffers

        // Create swapchain and render pass and pipelines
        let swapchain = Swapchain::new(&mut *core.surface, &*device.borrow(), Extent2D {
            width: extent.width,
            height: extent.height,
        });
        let render_pass = RenderPass::new(&swapchain, Rc::clone(&device));
        let framebuffer = unsafe {
            device.borrow().device.create_framebuffer(
                render_pass.render_pass.as_ref().unwrap(),
                iter::once(swapchain.framebuffer_attachment.clone()),
                swapchain.extent).unwrap()
        };
        let framebuffer_data = FramebufferData::new(Rc::clone(&device), swapchain.frame_queue_size, framebuffer);

        let pipeline = GraphicsPipeline::new(
            vec![].into_iter(),
            render_pass.render_pass.as_ref().unwrap(),
            Rc::clone(&device),
            "",
            "",
        );

        let viewport = swapchain.make_viewport();

        Renderer {
            core,
            device,
            swapchain,
            render_pass,
            framebuffer_data,
            viewport,
            pipeline,
        }
    }
}