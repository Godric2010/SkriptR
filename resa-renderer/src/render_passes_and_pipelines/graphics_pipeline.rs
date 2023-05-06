use std::cell::RefCell;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use crate::core::CoreDevice;
use crate::render_passes_and_pipelines::RenderStage;

pub struct GraphicsPipeline<B: Backend> {
	device: Rc<RefCell<CoreDevice<B>>>,
	pub pipeline: Option<B::GraphicsPipeline>,
	layout: Option<B::PipelineLayout>,
	pub stage: RenderStage,
}

impl<B: Backend> GraphicsPipeline<B> {
	pub(crate) fn new(device: Rc<RefCell<CoreDevice<B>>>, pipeline: B::GraphicsPipeline, layout: B::PipelineLayout, stage: RenderStage) -> Self {
		GraphicsPipeline{
			device,
			pipeline: Some(pipeline),
			layout: Some(layout),
			stage,
		}
	}
}


impl<B: Backend> Drop for GraphicsPipeline<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
			device.destroy_pipeline_layout(self.layout.take().unwrap());
		}
	}
}