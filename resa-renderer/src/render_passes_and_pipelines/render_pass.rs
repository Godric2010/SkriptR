use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use crate::core::CoreDevice;
use crate::render_passes_and_pipelines::RenderStage;

pub struct RenderPass<B: Backend> {
	pass: Option<B::RenderPass>,
	stage: RenderStage,
	device: Rc<RefCell<CoreDevice<B>>>,
}

impl<B: Backend> RenderPass<B> {
	pub(crate) fn new(device: Rc<RefCell<CoreDevice<B>>>, pass: B::RenderPass, stage: RenderStage) -> Self{
		RenderPass{
			pass: Some(pass),
			stage,
			device,
		}
	}

	pub fn get(&self) -> &B::RenderPass{
		&self.pass.as_ref().unwrap()
	}

	pub fn stage(&self) -> &RenderStage{
		&self.stage
	}
}

impl<B: Backend> Drop for RenderPass<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_render_pass(self.pass.take().unwrap());
		}
	}
}