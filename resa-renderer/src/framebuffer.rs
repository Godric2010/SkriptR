use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::pool::{CommandPool, CommandPoolCreateFlags};
use crate::core::CoreDevice;

pub struct FramebufferData<B: Backend> {
    pub framebuffer: Option<B::Framebuffer>,
    command_pools: Option<Vec<B::CommandPool>>,
    command_buffer_lists: Vec<Vec<(B::CommandBuffer, B::Fence)>>,
    present_semaphores: Option<Vec<B::Semaphore>>,
    device: Rc<RefCell<CoreDevice<B>>>,
}

impl<B: Backend> FramebufferData<B> {
    pub fn new(device: Rc<RefCell<CoreDevice<B>>>, num_frames: u32, framebuffer: B::Framebuffer) -> Self {
        let mut command_pools: Vec<_> = vec![];
        let mut command_buffer_lists = Vec::new();
        let mut present_semaphores: Vec<B::Semaphore> = vec![];

        unsafe {
            for _ in 0..num_frames {
                command_pools.push(
                    device
                        .borrow()
                        .device
                        .create_command_pool(
                            device.borrow().queues.family,
                            CommandPoolCreateFlags::empty(),
                        )
                        .expect("Cannot create command pool"),
                );
                command_buffer_lists.push(Vec::new());

                present_semaphores.push(device.borrow().device.create_semaphore().unwrap())
            }
        }

        FramebufferData {
            framebuffer: Some(framebuffer),
            command_pools: Some(command_pools),
            command_buffer_lists,
            present_semaphores: Some(present_semaphores),
            device,
        }
    }

    pub fn get_frame_data(&mut self, index: usize) -> (&B::Framebuffer, &mut B::CommandPool, &mut Vec<(B::CommandBuffer, B::Fence)>, &mut B::Semaphore) {
        (
            self.framebuffer.as_ref().unwrap(),
            &mut self.command_pools.as_mut().unwrap()[index],
            &mut self.command_buffer_lists[index],
            &mut self.present_semaphores.as_mut().unwrap()[index]
        )
    }
}

impl<B: Backend> Drop for FramebufferData<B> {
    fn drop(&mut self) {
        let device = &self.device.borrow().device;

        unsafe {
            if let Some(fb) = self.framebuffer.take(){
                device.destroy_framebuffer(fb);
            }
            for (mut command_pool, command_buffer_list) in self.command_pools
                .take().unwrap().into_iter().zip(self.command_buffer_lists.drain(..)){
                command_pool.free(command_buffer_list.into_iter().map(|(c,f)|{
                    device.destroy_fence(f);
                    c
                }));
                device.destroy_command_pool(command_pool);
            }

            for present_semaphore in self.present_semaphores.take().unwrap() {
                device.destroy_semaphore(present_semaphore);
            }
        }
    }

}