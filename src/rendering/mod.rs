use std::mem::ManuallyDrop;
use gfx_hal::{Instance};
use crate::window::Window;

pub struct Renderer<B: gfx_hal::Backend> {
    instance: B::Instance,
    surface: B::Surface,
}

pub struct RendererInstance<B: gfx_hal::Backend>(ManuallyDrop<Renderer<B>>);


impl<B: gfx_hal::Backend> RendererInstance<B> {
    pub fn new(window: &Window) -> Option<Self> {
        let renderer = Renderer::new(window);
        if renderer.is_none() {
            return None;
        }

        Some(RendererInstance(ManuallyDrop::new(renderer.unwrap())))
    }
}

pub fn create_renderer(window: &Window)-> RendererInstance<backend::Backend>{
    RendererInstance::new(window).unwrap()
}

impl<B: gfx_hal::Backend> Drop for RendererInstance<B> {
    fn drop(&mut self) {
        unsafe {
            let Renderer {
                instance,
                mut surface,
            } = ManuallyDrop::take(&mut self.0);

            instance.destroy_surface(surface)
        }
    }
}


impl<B: gfx_hal::Backend> Renderer<B> {
    pub fn new(window: &Window) -> Option<Self> {
        let instance_result = Instance::create(&window.name, 1);

        // Use this kind of error handling, because otherwise the instance memory would not be accessible for other things.
        if instance_result.is_err() {
            println!("Creating an instance failed due to unsupported backend!");
            return None;
        }
        let instance: B::Instance = instance_result.unwrap();


        let surface_result = unsafe { instance.create_surface(&window.instance) };

        if surface_result.is_err() {
            println!("Creating surface failed!");
            return None;
        }
        let surface = surface_result.unwrap();


        let mut adapters = instance.enumerate_adapters();//.remove(0);
        println!("Adapters: {}", adapters.len());
        let adapter = adapters.remove(0);
        println!("{}", adapter.info.name);
        let renderer = Renderer { instance: instance as B::Instance , surface: surface as B::Surface};

        Some(renderer)
    }
}