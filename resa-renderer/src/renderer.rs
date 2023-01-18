use gfx_hal::Backend;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::core::{Core, CoreDevice};

pub(crate) struct Renderer<B: Backend>{
    core: Core<B>,
    device: CoreDevice<B>,
}

impl<B: Backend> Renderer<B> {
    pub(crate) fn new(window: &Window, extent: &PhysicalSize<u32>) -> Self{
        let mut core = Core::<B>::create(&window).unwrap();
        let device = CoreDevice::<B>::new(core.adapter.adapter.take().unwrap(), &core.surface);

        Renderer{
            core,
            device
        }
    }
}