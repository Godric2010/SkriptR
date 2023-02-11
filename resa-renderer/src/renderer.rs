use std::cell::RefCell;
use std::{iter};
use std::rc::Rc;
use std::time::Instant;

use gfx_hal::Backend;
use gfx_hal::buffer::{SubRange, Usage};
use gfx_hal::command::{ClearColor, ClearValue, CommandBuffer, CommandBufferFlags, Level, RenderAttachmentInfo, SubpassContents};
use gfx_hal::device::Device;
use gfx_hal::pool::CommandPool;
use gfx_hal::prelude::PresentationSurface;
use gfx_hal::pso::{BufferDescriptorFormat, BufferDescriptorType, ColorValue, DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags, Viewport};
use gfx_hal::queue::Queue;
use gfx_hal::window::Extent2D;
use winit::window::Window;

use crate::buffer::Buffer;
use crate::core::{Core, CoreDevice};
use crate::descriptors::DescSetLayout;
use crate::framebuffer::FramebufferData;
use crate::graphics_pipeline::GraphicsPipeline;
use crate::helper::MVP;
use crate::mesh_controller::MeshController;
use crate::RendererConfig;
use crate::renderpass::RenderPass;
use crate::swapchain::Swapchain;
use crate::uniform::Uniform;
use crate::vertex::Vertex;

pub(crate) struct Renderer<B: Backend> {
	config: RendererConfig,
	core: Core<B>,
	device: Rc<RefCell<CoreDevice<B>>>,
	swapchain: Swapchain,
	render_pass: RenderPass<B>,
	framebuffer_data: FramebufferData<B>,
	viewport: Viewport,
	pipeline: GraphicsPipeline<B>,
	vertex_buffers: Vec<Buffer<B>>,
	uniform: Uniform<B>,
	uniform_desc_pool: Option<B::DescriptorPool>,
	pub recreate_swapchain: bool,
	bg_color: ColorValue,
	frames_drawn: usize,
	start_time: Instant,
}

impl<B: Backend> Renderer<B> {
	pub(crate) fn new(window: &Window, config: RendererConfig) -> Self {

		// Create the connection between code and gpu.
		let mut core = Core::<B>::create(&window).unwrap();
		let device = Rc::new(RefCell::new(CoreDevice::<B>::new(core.adapter.adapter.take().unwrap(), &core.surface)));

		// Create buffers
		let uniform_desc = DescSetLayout::new(
			Rc::clone(&device),
			vec![
				DescriptorSetLayoutBinding {
					binding: 0,
					ty: DescriptorType::Buffer {
						ty: BufferDescriptorType::Uniform,
						format: BufferDescriptorFormat::Structured {
							dynamic_offset: false,
						},
					},
					count: 1,
					stage_flags: ShaderStageFlags::FRAGMENT,
					immutable_samplers: false,
				}],
		);

		let mut uniform_desc_pool = unsafe {
			device.borrow().device.create_descriptor_pool(
				1,
				iter::once(DescriptorRangeDesc {
					ty: DescriptorType::Buffer {
						ty: BufferDescriptorType::Uniform,
						format: BufferDescriptorFormat::Structured {
							dynamic_offset: false,
						},
					},
					count: 1,
				}),
				DescriptorPoolCreateFlags::empty(),
			)
		}.ok();

		let uniform_desc = uniform_desc.create_desc_set(
			uniform_desc_pool.as_mut().unwrap(),
			"uniform",
			Rc::clone(&device),
		);

		let uniform = Uniform::new(Rc::clone(&device), &core.adapter.memory_types, &[1f32, 1.0f32, 1.0f32, 1.0f32], uniform_desc, 0);

		// let vertex_buffer = Buffer::new::<Vertex>(
		// 	Rc::clone(&device),
		// 	&[],
		// 	Usage::VERTEX,
		// 	&core.adapter.memory_types,
		// );


		// Create swapchain and render pass and pipelines
		let swapchain = Swapchain::new(&mut *core.surface, &*device.borrow(), Extent2D {
			width: config.extent.width,
			height: config.extent.height,
		});
		let render_pass = RenderPass::new(&swapchain, Rc::clone(&device));
		let framebuffer = unsafe {
			device.borrow().device.create_framebuffer(
				render_pass.render_pass.as_ref().unwrap(),
				iter::once(swapchain.framebuffer_attachment.clone()),
				swapchain.extent).unwrap()
		};
		let framebuffer_data = FramebufferData::new(Rc::clone(&device), swapchain.frame_queue_size, framebuffer);

		let pipeline = GraphicsPipeline::new(
			vec![].into_iter(),
			render_pass.render_pass.as_ref().unwrap(),
			Rc::clone(&device),
			config.vertex_shader_path.as_str(),
			config.fragment_shader_path.as_str(),
		);

		let viewport = swapchain.make_viewport();

		Renderer {
			config,
			core,
			device,
			swapchain,
			render_pass,
			framebuffer_data,
			viewport,
			pipeline,
			vertex_buffers: vec![],
			uniform_desc_pool,
			uniform,
			recreate_swapchain: true,
			bg_color: [0.1, 0.1, 0.1, 1.0],
			frames_drawn: 0,
			start_time: Instant::now(),

		}
	}

	pub fn add_vertex_buffer(&mut self, vertices: &[Vertex]) -> usize {
		let vertex_buffer = Buffer::new::<Vertex>(
			Rc::clone(&self.device),
			vertices,
			Usage::VERTEX,
			&self.core.adapter.memory_types,
		);
		let buffer_id = self.vertex_buffers.len();
		self.vertex_buffers.push(vertex_buffer);
		buffer_id
	}

	pub fn recreate_swapchain(&mut self, dimensions: Extent2D) {
		let device = &self.device.borrow().device;
		device.wait_idle().unwrap();

		self.swapchain = Swapchain::new(&mut *self.core.surface, &*self.device.borrow(), dimensions);
		self.render_pass = RenderPass::new(&self.swapchain, Rc::clone(&self.device));

		let new_fb = unsafe {
			device.destroy_framebuffer(self.framebuffer_data.framebuffer.take().unwrap());
			device.create_framebuffer(self.render_pass.render_pass.as_ref().unwrap(),
				iter::once(self.swapchain.framebuffer_attachment.clone()),
				self.swapchain.extent)
		}.unwrap();

		self.framebuffer_data = FramebufferData::new(Rc::clone(&self.device), self.swapchain.frame_queue_size, new_fb);

		self.pipeline = GraphicsPipeline::new(
			vec![self.uniform.get_layout()].into_iter(),
			self.render_pass.render_pass.as_ref().unwrap(),
			Rc::clone(&self.device),
			self.config.vertex_shader_path.as_str(),
			self.config.fragment_shader_path.as_str(),
		);

		self.viewport = self.swapchain.make_viewport();
	}

	pub fn draw(&mut self, mesh_ids: &[(u64, [[f32; 4]; 4])], view_mat: [[f32; 4]; 4], projection_mat: [[f32; 4]; 4], mesh_controller: &MeshController) {
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
			cmd_buffer.bind_graphics_pipeline(self.pipeline.pipeline.as_ref().unwrap());
			/*cmd_buffer.bind_graphics_descriptor_sets(self.pipeline.pipeline_layout.as_ref().unwrap(),
				0,
				vec![
					self.uniform.desc.as_ref().unwrap().set.as_ref().unwrap(),
				].into_iter(),
				iter::empty(),
			);*/

			cmd_buffer.begin_render_pass(
				self.render_pass.render_pass.as_ref().unwrap(),
				framebuffer,
				self.viewport.rect,
				iter::once(RenderAttachmentInfo {
					image_view: std::borrow::Borrow::borrow(&surface_image),
					clear_value: ClearValue {
						color: ClearColor {
							float32: self.bg_color,
						}
					},
				}),
				SubpassContents::Inline,
			);

			for (mesh_id, transform) in mesh_ids.iter() {
				let (buffer_id, amount_of_verts) = mesh_controller.get_mesh_data(mesh_id);

				let model_transform = transform;

				let mvp = MVP {
					model: *transform,
					view: view_mat,
					proj: projection_mat,
				};

				let mvp_bytes = mvp.as_bytes();
				let pipeline_layout = self.pipeline.pipeline_layout.as_ref().unwrap();

				cmd_buffer.bind_vertex_buffers(0, iter::once((self.vertex_buffers[buffer_id as usize].get(), SubRange::WHOLE)));
				cmd_buffer.push_graphics_constants(&pipeline_layout, ShaderStageFlags::VERTEX, 0, mvp_bytes);

				cmd_buffer.bind_graphics_descriptor_sets(&pipeline_layout,
					0,
					vec![
						self.uniform.desc.as_ref().unwrap().set.as_ref().unwrap(),
					].into_iter(),
					iter::empty(),
				);
				cmd_buffer.draw(0..amount_of_verts as u32, 0..1);
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

