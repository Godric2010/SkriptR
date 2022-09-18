use std::mem::ManuallyDrop;
use gfx_hal::command::Level;
use gfx_hal::device::Device;
use gfx_hal::pool::{CommandPool, CommandPoolCreateFlags};
use gfx_hal::queue::QueueFamilyId;

pub struct CommandBufferController<B: gfx_hal::Backend> {
    pool: ManuallyDrop<B::CommandPool>,
    graphics_buffer: B::CommandBuffer,
}

impl<B: gfx_hal::Backend> CommandBufferController<B> {
    pub fn new(device: &B::Device, queue_family: QueueFamilyId, size: u32) -> Option<Self> {
        let command_pool_result = unsafe { device.create_command_pool(queue_family, CommandPoolCreateFlags::empty()) };
        if command_pool_result.is_err() {
            println!("Failed to create command pool. Out of memory!");
            return None;
        }
        let mut command_pool = command_pool_result.unwrap();

        let graphics_command_buffer = unsafe { command_pool.allocate_one(Level::Primary) };

        Some(Self{
            pool: ManuallyDrop::new(command_pool),
            graphics_buffer: graphics_command_buffer,
        })
    }

    pub unsafe fn destroy(&mut self, device: &B::Device){
        let pool = ManuallyDrop::take(&mut self.pool);
        device.destroy_command_pool(pool)
    }
}
