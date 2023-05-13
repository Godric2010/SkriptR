use std::collections::HashMap;
use gfx_hal::Backend;
use crate::pipelines::graphics_pipeline::GraphicsPipeline;
use crate::render_stage::RenderStage;

pub mod graphics_pipeline;
pub mod pipeline_builder;


pub struct PipelineController<B: Backend> {
	pub pipelines: HashMap<RenderStage, Vec<GraphicsPipeline<B>>>,
}

impl<B: Backend> PipelineController<B> {
	pub fn new() -> Self {
		PipelineController {
			pipelines: HashMap::new(),
		}
	}

	pub fn add_pipeline(&mut self, pipeline: GraphicsPipeline<B>) -> u16 {
		let stage = &pipeline.stage;
		let render_weight = get_stage_render_weight(stage);
		if self.pipelines.contains_key(&stage) {
			let pipelines_in_stage = self.pipelines.get(stage).unwrap().len() as u16;
			self.pipelines.get_mut(stage).unwrap().push(pipeline);
			return render_weight + pipelines_in_stage;
		}

		self.pipelines.insert(stage.clone(), vec![pipeline]);
		render_weight
	}

	pub fn get_render_index_from_pipeline(&self, stage: &RenderStage) -> Option<u16> {
		if self.pipelines.contains_key(stage) {
			return Some(get_stage_render_weight(stage));
		}
		None
	}

	pub fn get_all_pipelines_sorted(&self) -> Vec<&GraphicsPipeline<B>> {
		let mut pipelines = vec![];
		pipelines.append(&mut self.get_all_pipelines_of_stage(&RenderStage::Opaque));
		pipelines.append(&mut self.get_all_pipelines_of_stage(&RenderStage::Transparent));
		pipelines.append(&mut self.get_all_pipelines_of_stage(&RenderStage::UI));

		pipelines
	}

	fn get_all_pipelines_of_stage(&self, stage: &RenderStage) -> Vec<&GraphicsPipeline<B>> {
		let mut stage_pipelines = vec![];
		if self.pipelines.contains_key(stage) {
			let pipelines_of_stage = self.pipelines.get(stage).unwrap();
			for pipe in pipelines_of_stage {
				stage_pipelines.push(pipe);
			}
		}
		stage_pipelines
	}
}

fn get_stage_render_weight(stage: &RenderStage) -> u16 {
	return match stage {
		RenderStage::None => 0,
		RenderStage::Opaque => 1000,
		RenderStage::Transparent => 2000,
		RenderStage::UI => 3000,
	};
}

fn get_stage_from_weight(weight: u16) -> RenderStage {
	if weight < 1000 {
		return RenderStage::None;
	}
	if weight < 2000 {
		return RenderStage::Opaque;
	}
	if weight < 3000 {
		return RenderStage::Transparent;
	}

	return RenderStage::UI;
}
