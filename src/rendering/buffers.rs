use std::mem::ManuallyDrop;
use gfx_hal::adapter::PhysicalDevice;
use gfx_hal::buffer::Usage;
use gfx_hal::device::Device;
use gfx_hal::memory::{Properties, SparseFlags};
use gfx_hal::MemoryTypeId;

pub struct Buffer<B: gfx_hal::Backend> {
    pub buffer_memory: ManuallyDrop<B::Memory>,
    pub buffer: ManuallyDrop<B::Buffer>,
}

impl<B: gfx_hal::Backend> Buffer<B> {
    pub fn new(device: &B::Device, physical_device: &B::PhysicalDevice, buffer_len: usize, usage: Usage, properties: Properties) -> Option<Self> {
        unsafe {
            let buffer_result = device.create_buffer(buffer_len as u64, usage, SparseFlags::SPARSE_RESIDENCY);
            if buffer_result.is_err() {
                return None;
            }
            let mut buffer = buffer_result.unwrap();

            let requirements = device.get_buffer_requirements(&buffer);

            let memory_types = physical_device.memory_properties().memory_types;
            let memory_type = memory_types
                .iter()
                .enumerate()
                .find(|(id, mem_type)| {
                    let type_supported = requirements.type_mask & (1_u32 << id) != 0;
                    type_supported && mem_type.properties.contains(properties)
                })
                .map(|(id, _ty)| {
                    MemoryTypeId(id)
                })
                .expect("No compatible memory type available");

            let buffer_memory_result = device.allocate_memory(memory_type, requirements.size);
            if buffer_memory_result.is_err() {
                return None;
            }
            let buffer_memory = buffer_memory_result.unwrap();

            device.bind_buffer_memory(&buffer_memory, 0, &mut buffer).unwrap();

            Some(Buffer{
                buffer: ManuallyDrop::new(buffer),
                buffer_memory: ManuallyDrop::new(buffer_memory),
            })
        }
    }

    pub fn release(&mut self, device: &B::Device) {
        unsafe {
            let buffer_memory = ManuallyDrop::take(&mut self.buffer_memory);
            device.free_memory(buffer_memory);

            let buffer = ManuallyDrop::take(&mut self.buffer);
            device.destroy_buffer(buffer);
        }
    }
}