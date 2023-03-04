use std::cell::RefCell;
use std::iter;
use std::ops::Range;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::image::{Access, Layout};
use gfx_hal::memory::Dependencies;
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDependency, SubpassDesc};
use gfx_hal::pso::PipelineStage;
use crate::core::CoreDevice;
use crate::image_buffer::Image;
use crate::swapchain::Swapchain;

pub struct RenderPass<B: Backend> {
	pub render_pass: Option<B::RenderPass>,
	device: Rc<RefCell<CoreDevice<B>>>,
}

// This can be optimized for multiple render pass usage. A render pass builder of some sort would be nice.
impl<B: Backend> RenderPass<B> {
	pub fn new(swapchain: &Swapchain, depth_image: &Image<B>, device: Rc<RefCell<CoreDevice<B>>>) -> Self {
		let mut render_pass = {
			let color_attachment = Attachment {
				format: Some(swapchain.format.clone()),
				samples: 1,
				ops: AttachmentOps::new(
					AttachmentLoadOp::Clear,
					AttachmentStoreOp::Store,
				),
				stencil_ops: AttachmentOps::DONT_CARE,
				layouts: Layout::Undefined..Layout::Present,
			};

			let depth_attachment = Attachment {
				format: Some(depth_image.format),
				samples: 1,
				ops: AttachmentOps{
					load: AttachmentLoadOp::Clear,
					store: AttachmentStoreOp::Store
				},
				stencil_ops: AttachmentOps{
					load: AttachmentLoadOp::Clear,
					store: AttachmentStoreOp::DontCare
				},
				layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
			};

			let attachments = vec![color_attachment, depth_attachment];

			let subpass = SubpassDesc {
				colors: &[(0, Layout::ColorAttachmentOptimal)],
				depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
				inputs: &[],
				resolves: &[],
				preserves: &[],
			};

			let dependencies = SubpassDependency {
				passes: Range { start: None, end: Some(0) },
				stages: Range { start: PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS, end: PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS },
				accesses: Range { start: Access::empty(), end: Access::COLOR_ATTACHMENT_WRITE| Access::DEPTH_STENCIL_ATTACHMENT_WRITE },
				flags: Dependencies::VIEW_LOCAL,
			};

			let rp = unsafe {
				device.borrow().device.create_render_pass(
					attachments.into_iter(),
					iter::once(subpass),
					iter::once(dependencies))
			}
				.ok();
			rp
		};

		if let Some(ref mut rp) = render_pass {
			unsafe { device.borrow().device.set_render_pass_name(rp, "main pass") };
		}

		RenderPass {
			render_pass,
			device,
		}
	}
}


impl<B: Backend> Drop for RenderPass<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_render_pass(self.render_pass.take().unwrap());
		}
	}
}