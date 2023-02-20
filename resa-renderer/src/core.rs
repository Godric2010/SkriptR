use std::mem::ManuallyDrop;
use std::ptr;
use gfx_hal::{Backend, Features, Instance, Limits};
use gfx_hal::adapter::{Adapter, MemoryType, PhysicalDevice};
use gfx_hal::format::{Format, ImageFeature};
use gfx_hal::image::Tiling;
use gfx_hal::prelude::{QueueFamily, Surface};
use gfx_hal::queue::QueueGroup;
use winit::window::Window;

pub struct Core<B: Backend> {
    pub surface: ManuallyDrop<B::Surface>,
    pub adapter: CoreAdapter<B>,
    pub instance: B::Instance,
}

impl<B: Backend> Core<B> {
    pub fn create(window: &Window) -> Option<Self> {
        let instance_result = Instance::create("RESA", 1);
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

        let mut adapters = instance.enumerate_adapters();

        Some(Self {
            instance,
            adapter: CoreAdapter::new(&mut adapters),
            surface: ManuallyDrop::new(surface),
        })
    }
}

impl<B: Backend> Drop for Core<B> {
    fn drop(&mut self) {
        unsafe {
            let surface = ManuallyDrop::into_inner(ptr::read(&self.surface));
            self.instance.destroy_surface(surface);
        }
    }
}

pub struct CoreAdapter<B: Backend> {
    pub adapter: Option<Adapter<B>>,
    pub memory_types: Vec<MemoryType>,
    pub limits: Limits,
}

impl<B: Backend> CoreAdapter<B> {
    pub fn new(adapters: &mut Vec<Adapter<B>>) -> Self {
        // TODO: Add komplex adapter choosing logic here!

        CoreAdapter::<B>::new_adapter(adapters.remove(0))
    }

    fn new_adapter(adapter: Adapter<B>) -> Self {
        let memory_types = adapter.physical_device.memory_properties().memory_types;
        let limits = adapter.physical_device.properties().limits;
        CoreAdapter {
            adapter: Some(adapter),
            memory_types,
            limits,
        }
    }
}

pub struct CoreDevice<B: Backend> {
    pub device: B::Device,
    pub physical_device: B::PhysicalDevice,
    pub queues: QueueGroup<B>,
}

impl <B: Backend> CoreDevice<B> {
    pub fn new(adapter: Adapter<B>, surface: &B::Surface) -> Self{
        let family = adapter
            .queue_families
            .iter()
            .find(|family|{
                surface.supports_queue_family(family) && family.queue_type().supports_graphics()
            })
            .unwrap();

        let mut gpu = unsafe{
            adapter
                .physical_device
                .open(&[(family, &[1.0])], gfx_hal::Features::empty())
                .unwrap()
        };

        CoreDevice{
            device: gpu.device,
            queues: gpu.queue_groups.pop().unwrap(),
            physical_device: adapter.physical_device,
        }
    }
    pub fn find_supported_format(&self, candidates: &[Format], tiling: Tiling, features: ImageFeature) -> Format{
        let mut format: Option<Format> = None;
        for candidate_format in candidates.iter() {

            let format_properties = self.physical_device.format_properties(Some(candidate_format.clone()));
            if tiling == Tiling::Linear && (format_properties.linear_tiling & features) == features{
                format = Some(candidate_format.clone());
                break;
            }
            else if tiling == Tiling::Optimal && (format_properties.optimal_tiling & features) == features {
                format = Some(candidate_format.clone());
                break;
            }
        }

        if format.is_none(){
            panic!("Cannot find format!")
        }

        format.unwrap()
    }
}
