use std::cell::RefCell;
use std::{fs, iter};
use std::mem::size_of;
use std::rc::Rc;
use gfx_hal::{Backend, spec_const_list};
use gfx_hal::device::Device;
use gfx_hal::format::Format;
use gfx_hal::pass::Subpass;
use gfx_hal::pso::{AttributeDesc, BlendState, ColorBlendDesc, ColorMask, Element, EntryPoint, GraphicsPipelineDesc, InputAssemblerDesc, Primitive, PrimitiveAssemblerDesc, Rasterizer, ShaderStageFlags, Specialization, VertexBufferDesc, VertexInputRate};
use glsl_to_spirv::ShaderType;
use crate::core::CoreDevice;
use crate::vertex::Vertex;

pub struct GraphicsPipeline<B: Backend> {
	pub pipeline: Option<B::GraphicsPipeline>,
	pub pipeline_layout: Option<B::PipelineLayout>,
	pub device: Rc<RefCell<CoreDevice<B>>>,
}

const ENTRY_NAME: &str = "main";

impl<B: Backend> GraphicsPipeline<B> {
	pub fn new<'a, Is>(desc_layouts: Is, render_pass: &B::RenderPass, device_ptr: Rc<RefCell<CoreDevice<B>>>, vert_shader_path: &str, frag_shader_path: &str) -> Self where Is: Iterator<Item = &'a B::DescriptorSetLayout>,
	{
		let device = &device_ptr.borrow().device;
		let pipeline_layout = unsafe {
			device.create_pipeline_layout(
				desc_layouts,
				iter::once((ShaderStageFlags::VERTEX, 0..8)), // use no magic number for push constants!
			)
		}.expect("Cannot create pipeline layout!");

		let pipeline = {
			let vs_module = {
				let shader = match create_shader(vert_shader_path, ShaderType::Vertex) {
					Some(shader) => shader,
					None => panic!("Could not create vertex shader!"),
				};
				unsafe { device.create_shader_module(&shader).unwrap() }
			};

			let fs_module = {
				let shader = match create_shader(frag_shader_path, ShaderType::Fragment) {
					Some(shader) => shader,
					None => panic!("Could not create fragment shader!"),
				};
				unsafe { device.create_shader_module(&shader).unwrap() }
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

				let vertex_buffers = vec![VertexBufferDesc {
					binding: 0,
					stride: size_of::<Vertex>() as u32,
					rate: VertexInputRate::Vertex,
				}];

				let attributes = vec![
					AttributeDesc {
						location: 0,
						binding: 0,
						element: Element {
							format: Format::Rg32Sfloat,
							offset: 0,
						},
					},
					AttributeDesc {
						location: 1,
						binding: 0,
						element: Element {
							format: Format::Rg32Sfloat,
							offset: 12,
						},
					},
				];

				let mut pipeline_desc = GraphicsPipelineDesc::new(
					PrimitiveAssemblerDesc::Vertex {
						buffers: &vertex_buffers,
						attributes: &attributes,
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


fn create_shader(shader_path: &str, shader_type: glsl_to_spirv::ShaderType) -> Option<Vec<u32>> {
	let glsl = match fs::read_to_string(shader_path) {
		Ok(glsl_shader) => glsl_shader,
		Err(_) => return None,
	};
	let file = match glsl_to_spirv::compile(&glsl, shader_type) {
		Ok(spirv_file) => spirv_file,
		Err(_) => return None,
	};

	match gfx_auxil::read_spirv(file) {
		Ok(spirv) => Some(spirv),
		Err(_) => None,
	}
}