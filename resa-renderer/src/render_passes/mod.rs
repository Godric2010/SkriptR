mod render_pass;
mod render_pass_builder;
mod framebuffer;

use std::cell::RefCell;
use std::ops::Range;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::image::{Access, Extent, FramebufferAttachment, Layout, Usage, ViewCapabilities};
use gfx_hal::memory::Dependencies;
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, SubpassDependency, SubpassDesc};
use gfx_hal::pso::PipelineStage;
use crate::core::CoreDevice;
use crate::render_passes::framebuffer::FramebufferData;
use crate::render_passes::render_pass::RenderPass;
use crate::render_passes::render_pass_builder::RenderPassBuilder;
use crate::render_stage::RenderStage;


#[derive(Copy, Clone, PartialEq, Hash)]
pub struct RenderPassDescriptor {
	pub render_stage: RenderStage,
	pub image_format: Format,
	pub depth_format: Option<Format>,
}

struct RenderPassEntry<B: Backend> {
	render_pass: RenderPass<B>,
	framebuffer: FramebufferData<B>,
	descriptor: RenderPassDescriptor,
}

pub struct RenderPassController<B: Backend> {
	device: Rc<RefCell<CoreDevice<B>>>,
	render_images: u32,
	image_extent: Extent,
	entries: Vec<RenderPassEntry<B>>,
}

impl<B: Backend> RenderPassController<B> {
	pub fn new(device: Rc<RefCell<CoreDevice<B>>>, extent: &Extent, image_amount: u32) -> Self {
		Self {
			device,
			image_extent: extent.clone(),
			render_images: image_amount,
			entries: vec![],
		}
	}

	pub fn create_new_render_pass_and_framebuffer(&mut self, desc: &RenderPassDescriptor) -> usize {
		if let Some(idx) = self.does_desc_already_exists(desc) {
			return idx;
		}
		let entry_id = self.entries.len();
		let render_pass = self.create_render_pass(&desc);
		let framebuffer = self.create_framebuffer(&desc, &render_pass);

		let entry = RenderPassEntry{
			render_pass,
			framebuffer,
			descriptor: desc.clone()
		};

		self.entries.push(entry);

		entry_id
	}

	pub fn get_render_pass_ref(&self, id: usize) -> &B::RenderPass {
		self.entries[id].render_pass.get()
	}

	pub fn get_framebuffer_data(&mut self, stage: &RenderStage) -> Option<(&B::Framebuffer, &B::RenderPass)> {
		let entry_idx = self.entries.iter().position(|entry| &entry.descriptor.render_stage == stage)?;
		let entry = &mut self.entries[entry_idx];
		let framebuffer  = entry.framebuffer.get_frame_data();
		let render_pass = entry.render_pass.get();
		Some((framebuffer, render_pass))
	}


	fn does_desc_already_exists(&self, desc: &RenderPassDescriptor) -> Option<usize> {
		for (idx, entry) in self.entries.iter().enumerate() {
			if entry.descriptor == desc.clone() {
				return Some(idx);
			}
		}
		None
	}

	fn create_color_attachment(&self, image_format: &Format, load_op: AttachmentLoadOp) -> Attachment {
		let color_attachment = Attachment {
			format: Some(image_format.clone()),
			samples: 1,
			ops: AttachmentOps::new(load_op, AttachmentStoreOp::Store),
			stencil_ops: AttachmentOps::DONT_CARE,
			layouts: Layout::Undefined..Layout::ColorAttachmentOptimal,
		};
		color_attachment
	}

	fn create_depth_attachment(&self, depth_format: &Format, load_op: AttachmentLoadOp) -> Attachment{
		let depth_attachment = Attachment {
			format: Some(depth_format.clone()),
			samples: 1,
			ops: AttachmentOps::new(load_op, AttachmentStoreOp::Store),
			stencil_ops: AttachmentOps::DONT_CARE,
			layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
		};
		depth_attachment
	}

	fn create_render_pass(&self, desc: &RenderPassDescriptor) -> RenderPass<B> {
		let attachment_load_op = match desc.render_stage {
			RenderStage::None => panic!("Render stage none is not allowed at this point anymore!"),
			RenderStage::Opaque => AttachmentLoadOp::Clear,
			RenderStage::Transparent => AttachmentLoadOp::Load,
			RenderStage::UI => AttachmentLoadOp::Clear,
		};


		let color_attachment = Some(self.create_color_attachment(&desc.image_format, attachment_load_op ));

		let mut depth_attachment = None;
		if let Some(depth_format) = desc.depth_format {
			depth_attachment = Some(self.create_depth_attachment(&depth_format, attachment_load_op));
		}

		let subpass = SubpassDesc {
			colors: &[(0, Layout::ColorAttachmentOptimal)],
			depth_stencil: Some(&(1, Layout::DepthStencilAttachmentOptimal)),
			inputs: &[],
			resolves: &[],
			preserves: &[],
		};

		let dependency = SubpassDependency {
			passes: Range { start: None, end: Some(0) },
			stages: Range { start: PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS, end: PipelineStage::COLOR_ATTACHMENT_OUTPUT | PipelineStage::EARLY_FRAGMENT_TESTS },
			accesses: Range { start: Access::empty(), end: Access::COLOR_ATTACHMENT_WRITE | Access::DEPTH_STENCIL_ATTACHMENT_WRITE },
			flags: Dependencies::VIEW_LOCAL,
		};


		let render_pass = RenderPassBuilder::new(self.device.clone())
			.add_attachment(color_attachment)
			.add_attachment(depth_attachment)
			.set_render_stage(desc.render_stage.clone())
			.add_subpass(subpass)
			.add_dependency(dependency)
			.add_name(&desc.render_stage.to_string())
			.build();

		match render_pass {
			Ok(rp) => rp,
			Err(_) => {
				panic!("Could not create {}", &desc.render_stage.to_string());
			}
		}
	}

	fn create_framebuffer_attachments(&self, desc: &RenderPassDescriptor) -> Vec<FramebufferAttachment> {
		let mut attachments = vec![FramebufferAttachment {
			format: desc.image_format,
			usage: Usage::COLOR_ATTACHMENT,
			view_caps: ViewCapabilities::empty(),
		}];

		if let Some(depth_format) = desc.depth_format {
			attachments.push(FramebufferAttachment {
				format: depth_format,
				usage: Usage::DEPTH_STENCIL_ATTACHMENT,
				view_caps: ViewCapabilities::empty(),
			})
		}

		attachments
	}

	fn create_framebuffer(&self, desc: &RenderPassDescriptor, render_pass: &RenderPass<B>) -> FramebufferData<B> {
		let attachments = self.create_framebuffer_attachments(&desc);
		let framebuffer = unsafe {
			self.device.borrow().device.create_framebuffer(
				render_pass.get(),
				attachments.into_iter(),
				self.image_extent).unwrap()
		};
		FramebufferData::new(Rc::clone(&self.device), self.render_images, framebuffer)
	}
}

