use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::{Device, OutOfMemory};
use gfx_hal::pass::{Attachment, SubpassDependency, SubpassDesc};
use crate::core::CoreDevice;
use crate::render_passes::render_pass::RenderPass;
use crate::render_stage::RenderStage;

pub struct RenderPassBuilder<'a, B: Backend> {
	pub(crate) device: Rc<RefCell<CoreDevice<B>>>,
	attachments: Vec<Attachment>,
	subpasses: Vec<SubpassDesc<'a>>,
	dependencies: Vec<SubpassDependency>,
	name: String,
	stage: RenderStage,
}

impl<'a, B: Backend> RenderPassBuilder<'a, B> {
	pub fn new(device: Rc<RefCell<CoreDevice<B>>>) -> Self {
		RenderPassBuilder {
			device,
			attachments: Vec::new(),
			subpasses: Vec::new(),
			dependencies: Vec::new(),
			name: "no_name".to_string(),
			stage: RenderStage::None,
		}
	}

	pub fn add_attachment(&mut self, attachment: Option<Attachment>) -> &mut Self {
		if let Some(attachment) = attachment {
			self.attachments.push(attachment);
		}
		self
	}

	pub fn add_subpass(&mut self, subpass: SubpassDesc<'a>) -> &mut Self {
		self.subpasses.push(subpass);
		self
	}

	pub fn add_dependency(&mut self, dependency: SubpassDependency) -> &mut Self {
		self.dependencies.push(dependency);
		self
	}

	pub fn add_name(&mut self, name: &str) -> &mut Self {
		self.name = name.to_string();
		self
	}

	pub fn set_render_stage(&mut self, stage: RenderStage) -> &mut Self {
		self.stage = stage;
		self
	}

	pub fn build(&self) -> Result<RenderPass<B>, OutOfMemory> {
		let pass = unsafe {
			self.device.borrow_mut().device.create_render_pass(
				self.attachments.clone().into_iter(),
				self.subpasses.clone().into_iter(),
				self.dependencies.clone().into_iter(),
			)
		}?;
		Ok(RenderPass::new(self.device.clone(), pass, self.stage))
	}
}
