use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::pool::{CommandPool, CommandPoolCreateFlags};
use crate::core::CoreDevice;

pub struct FramebufferData<B: Backend> {
	pub framebuffer: Option<B::Framebuffer>,
	device: Rc<RefCell<CoreDevice<B>>>,
}

impl<B: Backend> FramebufferData<B> {
	pub fn new(device: Rc<RefCell<CoreDevice<B>>>, num_frames: u32, framebuffer: B::Framebuffer) -> Self {
		FramebufferData {
			framebuffer: Some(framebuffer),
			device,
		}
	}

	pub fn get_frame_data(&mut self) -> &B::Framebuffer {
		self.framebuffer.as_ref().unwrap()
	}
}

impl<B: Backend> Drop for FramebufferData<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;

		unsafe {
			if let Some(fb) = self.framebuffer.take() {
				device.destroy_framebuffer(fb);
			}
		}
	}
}