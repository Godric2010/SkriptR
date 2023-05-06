use std::collections::HashMap;
use gfx_hal::Backend;
use crate::render_passes_and_pipelines::graphics_pipeline::GraphicsPipeline;
use crate::render_passes_and_pipelines::render_pass::RenderPass;

mod graphics_pipeline;
mod pipeline_builder;
mod render_pass;
mod render_pass_builder;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub enum RenderStage {
	None,
	Opaque,
	Transparent,
	UI,
}

pub struct RenderStageController<B: Backend> {
	pipelines: HashMap<RenderStage, Vec<GraphicsPipeline<B>>>,
	passes: HashMap<RenderStage, Vec<RenderPass<B>>>,
}

impl<B: Backend> RenderStageController<B> {
	pub fn new() -> Self {
		RenderStageController {
			pipelines: HashMap::new(),
			passes: HashMap::new(),
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
			let opaque_pipelines = self.pipelines.get(stage).unwrap();
			for pipe in opaque_pipelines {
				stage_pipelines.push(pipe);
			}
		}
		stage_pipelines
	}

	pub fn add_render_pass(&mut self, render_pass: RenderPass<B>) -> u16 {
		let pass_stage = render_pass.stage();
		let render_weight = get_stage_render_weight(pass_stage);
		if self.passes.contains_key(pass_stage) {
			let passes_in_stage = self.passes.get(pass_stage).unwrap().len() as u16;
			self.passes.get_mut(&render_pass.stage()).unwrap().push(render_pass);
			return render_weight + passes_in_stage;
		}

		self.passes.insert(pass_stage.clone(), vec![render_pass]);
		render_weight
	}

	pub fn get_render_pass(&self, render_pass_id: u16) -> &RenderPass<B> {
		let stage = get_stage_from_weight(render_pass_id);
		let index = render_pass_id % 1000;
		&self.passes.get(&stage).unwrap()[index as usize]
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
