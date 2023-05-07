use std::cell::RefCell;
use std::{iter};
use std::fmt::Debug;
use std::mem::size_of;
use std::ops::Range;
use std::rc::Rc;
use std::time::Instant;

use gfx_hal::{Backend, IndexType, Limits};
use gfx_hal::adapter::MemoryType;
use gfx_hal::buffer::{SubRange};
use gfx_hal::command::{ClearColor, ClearDepthStencil, ClearValue, CommandBuffer, CommandBufferFlags, Level, RenderAttachmentInfo, SubpassContents};
use gfx_hal::device::Device;
use gfx_hal::format::{Aspects, Format, ImageFeature};
use gfx_hal::image::{Access, Extent, FramebufferAttachment, Layout, Tiling, ViewCapabilities};
use gfx_hal::memory::{Dependencies, Properties};
use gfx_hal::pass::{Attachment, AttachmentLoadOp, AttachmentOps, AttachmentStoreOp, Subpass, SubpassDependency, SubpassDesc};
use gfx_hal::pool::{CommandPool};
use gfx_hal::prelude::PresentationSurface;
use gfx_hal::pso::{BlendState, ColorBlendDesc, ColorMask, ColorValue, Comparison, PipelineStage, ShaderStageFlags, Viewport};
use gfx_hal::queue::Queue;
use gfx_hal::window::Extent2D;
use winit::window::Window;

use crate::core::{Core, CoreAdapter, CoreDevice};
use crate::framebuffer::FramebufferData;
use crate::helper::MVP;
use crate::image_buffer::{Image};
use crate::material::{Material, MaterialRef};
use crate::pipelines::PipelineType;
use crate::render_passes_and_pipelines::{RenderStage, RenderStageController};
use crate::render_passes_and_pipelines::graphics_pipeline::GraphicsPipeline;
use crate::render_passes_and_pipelines::pipeline_builder::{PipelineBuilder, PipelineLayoutDesc};
use crate::render_passes_and_pipelines::render_pass::RenderPass;
use crate::render_passes_and_pipelines::render_pass_builder::RenderPassBuilder;
use crate::swapchain::Swapchain;
use crate::render_resources::RenderResources;

pub struct Renderer<B: Backend> {
	core: Core<B>,
	device: Rc<RefCell<CoreDevice<B>>>,
	swapchain: Swapchain,
	render_stage_controller: RenderStageController<B>,
	render_pass: RenderPass<B>,
	framebuffer_data: FramebufferData<B>,
	viewport: Viewport,
	pipelines: Vec<GraphicsPipeline<B>>,
	depth_image: Image<B>,
	pub recreate_swapchain: bool,
	bg_color: ColorValue,
	frames_drawn: usize,
	start_time: Instant,
}

impl<B: Backend> Renderer<B> {
	pub(crate) fn new(window: &Window, extent: Extent2D) -> Self {
		// Create the connection between code and gpu.
		let mut core = Core::<B>::create(&window).unwrap();
		let device = Rc::new(RefCell::new(CoreDevice::<B>::new(core.adapter.adapter.take().unwrap(), &core.surface)));

		// Create swapchain and render pass and pipelines
		let swapchain = Swapchain::new(&mut *core.surface, &*device.borrow(), extent);
		let depth_image = Renderer::<B>::create_depth_image(device.clone(), &core.adapter, swapchain.extent);
		let render_pass = RenderPass::new(&swapchain.format, &depth_image.format, Rc::clone(&device));

		let framebuffer_attachments = vec![swapchain.framebuffer_attachment.clone(), FramebufferAttachment {
			usage: gfx_hal::image::Usage::DEPTH_STENCIL_ATTACHMENT,
			view_caps: ViewCapabilities::empty(),
			format: depth_image.format.clone(),
		}];
		let framebuffer = unsafe {
			device.borrow().device.create_framebuffer(
				render_pass.render_pass.as_ref().unwrap(),
				framebuffer_attachments.into_iter(),
				swapchain.extent).unwrap()
		};
		let framebuffer_data = FramebufferData::new(Rc::clone(&device), swapchain.frame_queue_size, framebuffer);
		let viewport = swapchain.make_viewport();

		Renderer {
			core,
			device,
			swapchain,
			render_stage_controller: RenderStageController::new(),
			render_pass,
			framebuffer_data,
			viewport,
			pipelines: vec![],
			depth_image,
			recreate_swapchain: true,
			bg_color: [0.1, 0.1, 0.1, 1.0],
			frames_drawn: 0,
			start_time: Instant::now(),
		}
	}

	pub fn get_device(&self) -> Rc<RefCell<CoreDevice<B>>> {
		self.device.clone()
	}

	pub fn get_memory_types(&self) -> Vec<MemoryType> {
		self.core.adapter.memory_types.clone()
	}

	pub fn get_adapter_limits(&self) -> Limits {
		self.core.adapter.limits.clone()
	}

	pub fn get_material_render_stage_index(&mut self, material: &Material, resources: &RenderResources<B>) -> u16 {
		let pipeline_stage_index = self.render_stage_controller.get_render_index_from_pipeline(&material.render_stage);
		if pipeline_stage_index.is_some() {
			return pipeline_stage_index.unwrap();
		}

		let result = self.render_stage_controller.get_render_pass_from_stage(&material.render_stage);
		let render_pass = match result {
			Some(rp) => rp,
			None => {
				let rp = self.create_render_pass(&material.render_stage);
				let id = self.render_stage_controller.add_render_pass(rp);
				self.render_stage_controller.get_render_pass(id)
			}
		};


		0
	}

	pub fn recreate_swapchain(&mut self, dimensions: Extent2D) {
		let device = &self.device.borrow().device;
		device.wait_idle().unwrap();

		self.swapchain = Swapchain::new(&mut *self.core.surface, &*self.device.borrow(), dimensions);
		self.depth_image = Renderer::<B>::create_depth_image(self.device.clone(), &self.core.adapter, self.swapchain.extent);
		// self.render_pass = RenderPass::new(&self.swapchain.format, &self.depth_image.format, Rc::clone(&self.device));

		let new_fb = unsafe {
			device.destroy_framebuffer(self.framebuffer_data.framebuffer.take().unwrap());
			device.create_framebuffer(self.render_pass.render_pass.as_ref().unwrap(),
				iter::once(self.swapchain.framebuffer_attachment.clone()),
				self.swapchain.extent)
		}.unwrap();

		self.framebuffer_data = FramebufferData::new(Rc::clone(&self.device), self.swapchain.frame_queue_size, new_fb);

		// for pipeline in &self.pipelines {
		// 	// let pipe_type = pipeline.pipeline_type;
		// 	// &mut self.create_pipeline(&pipe_type);
		// }

		self.viewport = self.swapchain.make_viewport();
	}

	fn create_depth_image(device: Rc<RefCell<CoreDevice<B>>>, adapter: &CoreAdapter<B>, dimensions: Extent) -> Image<B> {
		let depth_formats = [Format::D24UnormS8Uint, Format::D32SfloatS8Uint, Format::D32Sfloat];
		let depth_format = device.borrow().find_supported_format(&depth_formats, Tiling::Optimal, ImageFeature::DEPTH_STENCIL_ATTACHMENT);
		let depth_image = Image::new(device.clone(), &adapter.memory_types, dimensions, depth_format, Tiling::Optimal, gfx_hal::image::Usage::DEPTH_STENCIL_ATTACHMENT, Properties::DEVICE_LOCAL, Aspects::DEPTH, gfx_hal::image::Usage::SAMPLED);
		depth_image
	}

	fn create_render_pass(&self, render_stage: &RenderStage) -> RenderPass<B> {
		let color_attachment = Attachment {
			format: Some(self.swapchain.format.clone()),
			samples: 1,
			ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::Store),
			stencil_ops: AttachmentOps::DONT_CARE,
			layouts: Layout::Undefined..Layout::ColorAttachmentOptimal,
		};

		let depth_attachment = Attachment {
			format: Some(self.depth_image.format.clone()),
			samples: 1,
			ops: AttachmentOps::new(AttachmentLoadOp::Clear, AttachmentStoreOp::DontCare),
			stencil_ops: AttachmentOps::DONT_CARE,
			layouts: Layout::Undefined..Layout::DepthStencilAttachmentOptimal,
		};

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
			.set_render_stage(render_stage.clone())
			.add_subpass(subpass)
			.add_dependency(dependency)
			.add_name(&render_stage.to_string())
			.build();

		match render_pass {
			Ok(rp) => rp,
			Err(e) => {
				panic!("Could not create {}", &render_stage.to_string());
			}
		}
	}

	fn create_graphics_pipeline(&self, material: &Material, render_pass: &RenderPass<B>, resources: &RenderResources<B>) -> GraphicsPipeline<B> {

		let layout_desc = PipelineLayoutDesc::new(
			resources.material_lib.get_descriptor_layouts(),
			size_of::<MVP>() as u32,
		);

		let shader_ref = resources.shader_lib.get_by_id(&material.shader_id).unwrap();

		let pipeline = PipelineBuilder::new(self.device.clone(), layout_desc)
			.add_render_pass(render_pass)
			.set_render_stage(material.render_stage.clone())
			.add_vertex_shader(&shader_ref.vertex)
			.add_fragment_shader(&shader_ref.fragment)
			.add_color_blend_state(ColorMask::ALL, BlendState::ALPHA)
			.add_depth_desc(Comparison::LessEqual, true)
			.build();

		match pipeline{
			Some(pipeline) => pipeline,
			None => panic!("Could not create pipeline for {}", &material.render_stage.to_string()),
		}

	}

	pub fn draw(&mut self, render_objects: &[(u64, MaterialRef, [[f32; 4]; 4])], view_mat: [[f32; 4]; 4], projection_mat: [[f32; 4]; 4], resource_binding: &RenderResources<B>) {
		if self.recreate_swapchain {
			self.recreate_swapchain(Extent2D { width: self.swapchain.extent.width, height: self.swapchain.extent.height });
			self.recreate_swapchain = false;
		}

		let surface_image = match unsafe { self.core.surface.acquire_image(!0) } {
			Ok((image, _)) => image,
			Err(_) => {
				self.recreate_swapchain = true;
				return;
			}
		};

		let frame_index = (self.swapchain.frame_index % self.swapchain.frame_queue_size) as usize;
		self.swapchain.frame_index += 1;
		self.frames_drawn += 1;

		let (framebuffer, command_pool, command_buffers, sem_image_presentation) = self.framebuffer_data.get_frame_data(frame_index);

		if self.pipelines.len() == 0 {
			return;
		}

		let pipeline = &self.pipelines[0];

		unsafe {
			let (mut cmd_buffer, mut fence) = match command_buffers.pop() {
				Some((cmd_buffer, fence)) => (cmd_buffer, fence),
				None => (
					command_pool.allocate_one(Level::Primary),
					self.device.borrow().device.create_fence(true).unwrap(),
				),
			};

			self.device.borrow().device.wait_for_fence(&mut fence, u64::MAX).unwrap();
			self.device.borrow().device.reset_fence(&mut fence).unwrap();

			command_pool.reset(false);

			cmd_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);
			cmd_buffer.set_viewports(0, iter::once(self.viewport.clone()));
			cmd_buffer.set_scissors(0, iter::once(self.viewport.rect));
			cmd_buffer.bind_graphics_pipeline(pipeline.pipeline.as_ref().unwrap());

			let attachments = vec![RenderAttachmentInfo {
				image_view: std::borrow::Borrow::borrow(&surface_image),
				clear_value: ClearValue {
					color: ClearColor {
						float32: self.bg_color,
					},
				},
			},
				RenderAttachmentInfo {
					image_view: self.depth_image.image_view.as_ref().unwrap(),
					clear_value: ClearValue {
						depth_stencil: ClearDepthStencil { depth: 1.0, stencil: 0 }
					},
				},
			];
			cmd_buffer.begin_render_pass(
				self.render_pass.render_pass.as_ref().unwrap(),
				framebuffer,
				self.viewport.rect,
				attachments.into_iter(),
				SubpassContents::Inline,
			);

			for (mesh_id, material_id, transform) in render_objects.iter() {
				let mesh_data = resource_binding.mesh_lib.get_mesh_entry(mesh_id);
				let mesh_index_amount = resource_binding.mesh_lib.get_mesh_index_amount(mesh_id);
				let material_render_data = resource_binding.material_lib.get_render_data(material_id);

				let mvp = MVP {
					model: *transform,
					view: view_mat,
					proj: projection_mat,
				};

				let mvp_bytes = mvp.as_bytes();
				let pipeline_layout = pipeline.pipeline_layout.as_ref().unwrap();
				cmd_buffer.bind_vertex_buffers(0, iter::once((mesh_data.vertex_buffer.get(), SubRange::WHOLE)));
				cmd_buffer.bind_index_buffer(mesh_data.index_buffer.get(), SubRange::WHOLE, IndexType::U16);
				cmd_buffer.push_graphics_constants(&pipeline_layout, ShaderStageFlags::VERTEX, 0, mvp_bytes);

				let sets = vec![
					material_render_data.1.desc.set.as_ref().unwrap(),
					material_render_data.0.desc.as_ref().unwrap().set.as_ref().unwrap()];


				cmd_buffer.bind_graphics_descriptor_sets(pipeline.pipeline_layout.as_ref().unwrap(),
					0,
					sets.into_iter(),
					iter::empty(),
				);

				cmd_buffer.draw_indexed(0..mesh_index_amount, 0, 0..1);
			}


			cmd_buffer.end_render_pass();
			cmd_buffer.finish();

			self.device.borrow_mut().queues.queues[0].submit(
				iter::once(&cmd_buffer),
				iter::empty(),
				iter::once(&*sem_image_presentation),
				Some(&mut fence),
			);
			command_buffers.push((cmd_buffer, fence));

			// present frame
			if let Err(_) = self.device.borrow_mut().queues.queues[0].present(
				&mut *self.core.surface,
				surface_image,
				Some(sem_image_presentation),
			) {
				self.recreate_swapchain = true;
			}
		}
	}

	pub fn get_fps(&self) -> f32 {
		let elapsed_time = self.start_time.elapsed();
		let fps = self.frames_drawn as f32 / elapsed_time.as_secs_f32();
		fps
	}
}

