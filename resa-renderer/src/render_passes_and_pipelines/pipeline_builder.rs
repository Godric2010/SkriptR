use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::{Device};
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{BlendState, ColorBlendDesc, ColorMask, Comparison, DepthStencilDesc, DepthTest, EntryPoint, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, ShaderStageFlags};
use crate::core::CoreDevice;
use crate::render_passes_and_pipelines::graphics_pipeline::GraphicsPipeline;
use crate::render_passes_and_pipelines::RenderStage;
use crate::vertex::Vertex;

pub struct PipelineLayoutDesc<'a, B: Backend> {
	descriptor_layouts: Vec<&'a <B as Backend>::DescriptorSetLayout>,
	push_constant_bytes: u32,
}

impl<'a, B: Backend> PipelineLayoutDesc<'a, B> {
	pub fn new(desc_layouts: Vec<&'a <B as Backend>::DescriptorSetLayout>, push_constants_byte_size: u32) -> Self {
		PipelineLayoutDesc {
			descriptor_layouts: desc_layouts,
			push_constant_bytes: push_constants_byte_size,
		}
	}
}

const ENTRY_NAME: &str = "main";

pub struct PipelineBuilder<'a, B: Backend> {
	device: Rc<RefCell<CoreDevice<B>>>,
	pipeline_desc: Option<GraphicsPipelineDesc<'a, B>>,
	layout_desc: Option<PipelineLayoutDesc<'a, B>>,
	subpass: Option<Subpass<'a, B>>,
	vertex_shader: Vec<u32>,
	fragment_shader: Vec<u32>,
	color_blend_desc: Option<ColorBlendDesc>,
	depth_desc: Option<DepthTest>,
	stage: RenderStage,
}

impl<'a, B: Backend> PipelineBuilder<'a, B> {
	pub fn new<Is>(device: Rc<RefCell<CoreDevice<B>>>, pipeline_layout_desc: PipelineLayoutDesc<'a, B>) -> Self {
		PipelineBuilder {
			device: device.clone(),
			pipeline_desc: None,
			layout_desc: Some(pipeline_layout_desc),
			subpass: None,
			vertex_shader: vec![],
			fragment_shader: vec![],
			color_blend_desc: None,
			depth_desc: None,
			stage: RenderStage::None,
		}
	}

	pub fn add_render_pass(&mut self, render_pass: &'a <B as Backend>::RenderPass) -> &mut Self {
		self.subpass = Some(Subpass {
			index: 0,
			main_pass: render_pass,
		});
		self
	}

	pub fn add_vertex_shader(&mut self, spirv_shader: &[u32]) -> &mut Self {
		self.vertex_shader = spirv_shader.to_vec();
		self
	}

	pub fn add_fragment_shader(&mut self, spirv_shader: &[u32]) -> &mut Self {
		self.fragment_shader = spirv_shader.to_vec();
		self
	}

	pub fn add_color_blend_state(&mut self, color_mask: ColorMask, blend_state: BlendState) -> &mut Self {
		self.color_blend_desc = Some(ColorBlendDesc {
			blend: Some(blend_state),
			mask: color_mask,
		});
		self
	}

	pub fn add_depth_desc(&mut self, depth_comparison: Comparison, write_to_depth_buffer: bool) -> &mut Self {
		self.depth_desc = Some(DepthTest {
			fun: depth_comparison,
			write: write_to_depth_buffer,
		});
		self
	}

	pub fn set_render_stage(&mut self, stage: RenderStage) -> &mut Self{
		self.stage = stage;
		self
	}

	pub fn build(&mut self) -> Option<GraphicsPipeline<B>> {
		let device = &self.device.borrow_mut().device;
		let vertex_shader_mod = self.create_shader_module(&device, &self.vertex_shader)?;
		let fragment_shader_mod = self.create_shader_module(&device, &self.fragment_shader)?;
		let vertex_shader = EntryPoint {
			entry: ENTRY_NAME,
			module: &vertex_shader_mod,
			specialization: Default::default(),
		};
		let fragment_shader = EntryPoint {
			entry: ENTRY_NAME,
			module: &fragment_shader_mod,
			specialization: Default::default(),
		};

		let vertex_buffers = Vertex::get_vertex_buffer_desc();
		let vertex_attributes = Vertex::get_vertex_attributes();

		let pipeline_layout = self.layout_desc.take().unwrap();
		let pipeline_layout = unsafe {
			device.create_pipeline_layout(
				pipeline_layout.descriptor_layouts.into_iter(),
				iter::once((ShaderStageFlags::VERTEX, 0..pipeline_layout.push_constant_bytes)),
			)
		}.expect("Cannot create pipeline layout");

		let subpass = self.subpass?;

		let mut pipeline_desc = GraphicsPipelineDesc::new(
			PrimitiveAssemblerDesc::Vertex {
				buffers: &vertex_buffers,
				attributes: &vertex_attributes,
				input_assembler: InputAssemblerDesc {
					primitive: Primitive::TriangleList,
					with_adjacency: false,
					restart_index: None,
				},
				vertex: vertex_shader,
				tessellation: None,
				geometry: None,
			},
			Rasterizer::FILL,
			Some(fragment_shader),
			&pipeline_layout,
			subpass,
		);
		if self.color_blend_desc.is_some() {
			pipeline_desc.blender.targets.push(self.color_blend_desc.unwrap());
		}

		if self.depth_desc.is_some() {
			pipeline_desc.depth_stencil = DepthStencilDesc {
				depth: self.depth_desc,
				depth_bounds: false,
				stencil: None,
			}
		}

		let pipeline = unsafe { device.create_graphics_pipeline(&pipeline_desc, None) }.unwrap();
		unsafe {
			device.destroy_shader_module(vertex_shader_mod);
			device.destroy_shader_module(fragment_shader_mod);
		}

		Some(GraphicsPipeline::new(self.device.clone(), pipeline, pipeline_layout, self.stage))
	}

	fn create_shader_module(&self, device: &B::Device, shader: &[u32]) -> Option<<B as Backend>::ShaderModule> {
		return match unsafe { device.create_shader_module(shader) } {
			Ok(module) => Some(module),
			Err(e) => {
				println!("Failed to create shader module!");
				None
			}
		};
	}
}