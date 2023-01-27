use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::buffer::Usage;
use gfx_hal::device::Device;
use gfx_hal::prelude::DescriptorPool;
use gfx_hal::pso::{BufferDescriptorFormat, BufferDescriptorType, ColorValue, DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags, Viewport};
use gfx_hal::window::Extent2D;
use winit::dpi::PhysicalSize;
use winit::window::Window;
use crate::buffer::Buffer;
use crate::core::{Core, CoreDevice};
use crate::descriptors::DescSetLayout;
use crate::framebuffer::FramebufferData;
use crate::graphics_pipeline::GraphicsPipeline;
use crate::renderpass::RenderPass;
use crate::swapchain::Swapchain;
use crate::uniform::Uniform;
use crate::vertex::Vertex;

pub(crate) struct Renderer<B: Backend> {
	core: Core<B>,
	device: Rc<RefCell<CoreDevice<B>>>,
	swapchain: Swapchain,
	render_pass: RenderPass<B>,
	framebuffer_data: FramebufferData<B>,
	viewport: Viewport,
	pipeline: GraphicsPipeline<B>,
	vertex_buffer: Buffer<B>,
	uniform: Uniform<B>,
	uniform_desc_pool: Option<B::DescriptorPool>,
	recreate_swapchain: bool,
	bg_color: ColorValue,

}

impl<B: Backend> Renderer<B> {
	pub(crate) fn new(window: &Window, extent: &PhysicalSize<u32>) -> Self {

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

		let uniform = Uniform::new(Rc::clone(&device), &core.adapter.memory_types, &[1f32,1.0f32, 1.0f32, 1.0f32], uniform_desc, 0);

		let vertex_buffer = Buffer::new::<Vertex>(
			Rc::clone(&device),
			&[],
			Usage::VERTEX,
			&core.adapter.memory_types,
		);

		// Create swapchain and render pass and pipelines
		let swapchain = Swapchain::new(&mut *core.surface, &*device.borrow(), Extent2D {
			width: extent.width,
			height: extent.height,
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
			"",
			"",
		);

		let viewport = swapchain.make_viewport();

		Renderer {
			core,
			device,
			swapchain,
			render_pass,
			framebuffer_data,
			viewport,
			pipeline,
			vertex_buffer,
			uniform_desc_pool,
			uniform,
			recreate_swapchain: true,
			bg_color: [0.8,0.8,0.8,1.0],
		}
	}

	pub fn recreate_swapchain(&mut self, dimensions: Extent2D){
		let device = &self.device.borrow().device;
		device.wait_idle().unwrap();

		self.swapchain = Swapchain::new(&mut *self.core.surface, &*self.device.borrow(),dimensions);
		self.render_pass = RenderPass::new(&self.swapchain, Rc::clone(&self.device));

		let new_fb = unsafe{
			device.destroy_framebuffer(self.framebuffer_data.framebuffer.take().unwrap());
			device.create_framebuffer(self.render_pass.render_pass.as_ref().unwrap(),
			iter::once(self.swapchain.framebuffer_attachment.clone()),
			self.swapchain.extent)
		}.unwrap();

		self.framebuffer_data = FramebufferData::new(Rc::clone(&self.device), self.swapchain.frame_queue_size, new_fb);

		self.pipeline= GraphicsPipeline::new(
			vec![self.uniform.get_layout()].into_iter(),
			self.render_pass.render_pass.as_ref().unwrap(),
			Rc::clone(&self.device),
			"","",
		);

		self.viewport = self.swapchain.make_viewport();
	}


}