use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::window::Extent2D;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::core::{Core, CoreDevice};
use crate::swapchain::Swapchain;

pub(crate) struct Renderer<B: Backend> {
    core: Core<B>,
    device: Rc<RefCell<CoreDevice<B>>>,
    swapchain: Swapchain,
}

impl<B: Backend> Renderer<B> {
    pub(crate) fn new(window: &Window, extent: &PhysicalSize<u32>) -> Self {

        // Create the connection between code and gpu.
        let mut core = Core::<B>::create(&window).unwrap();
        let device = Rc::new(RefCell::new(CoreDevice::<B>::new(core.adapter.adapter.take().unwrap(), &core.surface)));

        // Create buffers

        // Create swapchain and render pass and pipelines
        let swapchain = Swapchain::new(&mut *core.surface, &*device.borrow(), Extent2D{
            width: extent.width,
            height: extent.height,
        });

        Renderer {
            core,
            device,
            swapchain,
        }
    }
}