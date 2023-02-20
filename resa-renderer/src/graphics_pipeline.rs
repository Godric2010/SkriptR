use std::cell::RefCell;
use std::{fs, iter};
use std::mem::size_of;
use std::path::Path;
use std::rc::Rc;
use gfx_hal::{Backend, spec_const_list};
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Comparison, DepthStencilDesc, DepthTest, Element, EntryPoint, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, ShaderStageFlags, Specialization, VertexBufferDesc, VertexInputRate};
use glsl_to_spirv::ShaderType;
use crate::core::CoreDevice;
use crate::helper::MVP;
use crate::vertex::Vertex;

#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum PipelineType{
	Opaque,
	Transparent,
	UI,
}

pub struct GraphicsPipeline<B: Backend> {
	pub pipeline: Option<B::GraphicsPipeline>,
	pub pipeline_layout: Option<B::PipelineLayout>,
	pub device: Rc<RefCell<CoreDevice<B>>>,
	pub pipeline_type: PipelineType,
}

const ENTRY_NAME: &str = "main";

impl<B: Backend> GraphicsPipeline<B> {
	pub fn new<'a, Is>(desc_layouts: Is, render_pass: &B::RenderPass, device_ptr: Rc<RefCell<CoreDevice<B>>>, vert_shader: &[u32], frag_shader: &[u32]) -> Self where Is: Iterator<Item = &'a B::DescriptorSetLayout>,
	{
		let device = &device_ptr.borrow().device;
		let push_constants_bytes = size_of::<MVP>() as u32;
		let pipeline_layout = unsafe {
			device.create_pipeline_layout(
				desc_layouts,
				iter::once((ShaderStageFlags::VERTEX, 0..push_constants_bytes)),
			)
		}.expect("Cannot create pipeline layout!");

		let pipeline = {
			let vs_module = {
				unsafe { device.create_shader_module(vert_shader).unwrap() }
			};

			let fs_module = {
				unsafe { device.create_shader_module(frag_shader).unwrap() }
			};

			let pipeline = {
				let (vs_entry, fs_entry) = (
					EntryPoint::<B> {
						entry: ENTRY_NAME,
						module: &vs_module,
						specialization: spec_const_list![0.8f32],
					},
					EntryPoint::<B> {
						entry: ENTRY_NAME,
						module: &fs_module,
						specialization: Specialization::default(),
					}
				);

				let subpass = Subpass {
					index: 0,
					main_pass: render_pass,
				};

				let vertex_buffers = Vertex::get_vertex_buffer_desc();
				let vertex_attributes = Vertex::get_vertex_attributes();

				let mut pipeline_desc = GraphicsPipelineDesc::new(
					PrimitiveAssemblerDesc::Vertex {
						buffers: &vertex_buffers,
						attributes: &vertex_attributes,
						input_assembler: InputAssemblerDesc {
							primitive: Primitive::TriangleList,
							with_adjacency: false,
							restart_index: None,
						},
						vertex: vs_entry,
						geometry: None,
						tessellation: None,
					},
					Rasterizer::FILL,
					Some(fs_entry),
					&pipeline_layout,
					subpass,
				);

				pipeline_desc.blender.targets.push(ColorBlendDesc {
					mask: ColorMask::ALL,
					blend: Some(BlendState::ALPHA),
				});

				pipeline_desc.depth_stencil = DepthStencilDesc{
					depth: Some(DepthTest{ fun: Comparison::Less, write: true }),
					depth_bounds: false,
					stencil: None,
				};

				unsafe { device.create_graphics_pipeline(&pipeline_desc, None) }
			};

			unsafe {
				device.destroy_shader_module(vs_module);
				device.destroy_shader_module(fs_module);
			}

			pipeline.unwrap()
		};

		GraphicsPipeline {
			pipeline: Some(pipeline),
			pipeline_layout: Some(pipeline_layout),
			device: Rc::clone(&device_ptr),
			pipeline_type: PipelineType::Opaque,
		}
	}
}

impl<B: Backend> Drop for GraphicsPipeline<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_graphics_pipeline(self.pipeline.take().unwrap());
			device.destroy_pipeline_layout(self.pipeline_layout.take().unwrap());
		}
	}
}