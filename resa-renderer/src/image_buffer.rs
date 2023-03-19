use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::{Backend, buffer};
use gfx_hal::command::{BufferImageCopy, CommandBuffer, CommandBufferFlags, Level};
use gfx_hal::device::Device;
use gfx_hal::format::{AsFormat, Aspects, Format, Rgba8Srgb, Swizzle};
use gfx_hal::image::{Access, Extent, Filter, Kind, Layout, Offset, SamplerDesc, Size, SubresourceLayers, SubresourceRange, Tiling, Usage, ViewCapabilities, ViewKind, WrapMode};
use gfx_hal::memory::{Barrier, Dependencies, Properties, SparseFlags};
use gfx_hal::pool::CommandPool;
use gfx_hal::pso::{Descriptor, PipelineStage};
use gfx_hal::queue::Queue;
use crate::buffer::Buffer;
use crate::core::{CoreAdapter, CoreDevice};
use crate::descriptors::{DescSet, DescSetWrite};
use crate::image_buffer;

pub struct Dimensions<T> {
	pub width: T,
	pub height: T,
}

pub struct ImageBuffer<B: Backend> {
	desc: DescSet<B>,
	buffer: Option<Buffer<B>>,
	sampler: Option<B::Sampler>,
	image: Option<Image<B>>,
	transferred_image_fence: Option<B::Fence>,
}

pub struct Image<B: Backend> {
	pub image_view: Option<B::ImageView>,
	pub image: Option<B::Image>,
	pub memory: Option<B::Memory>,
	pub format: Format,
	device: Rc<RefCell<CoreDevice<B>>>
}

impl<B: Backend> ImageBuffer<B> {
	pub fn new(
		mut desc: DescSet<B>,
		img: &image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>,
		adapter: &CoreAdapter<B>,
		usage: buffer::Usage,
		device_ptr:Rc<RefCell<CoreDevice<B>>>,
		staging_pool: &mut B::CommandPool) -> Self {
		let (buffer, dimensions, row_pitch, stride) = Buffer::new_texture(Rc::clone(&device_ptr), img, &adapter, usage);

		let buffer = Some(buffer);
		let dimensions = Extent { width: dimensions.width, height: dimensions.height, depth: 1 };
		let image = Image::new(Rc::clone(&device_ptr), &adapter, dimensions, Format::Rgba8Srgb, Tiling::Optimal, Usage::TRANSFER_DST | Usage::SAMPLED, Properties::DEVICE_LOCAL, Aspects::COLOR, Usage::SAMPLED);

		let device_ref = &mut device_ptr.borrow_mut();
		// let device = &mut device_ref.device;

		let image_buffer = unsafe {
			let sampler = device_ref.device.create_sampler(
				&SamplerDesc::new(Filter::Linear, WrapMode::Clamp)).expect(" Cannot create sampler!");

			desc.write_to_state(DescSetWrite {
				binding: 0,
				array_offset: 0,
				descriptors: iter::once(Descriptor::Image(
					image.image_view.as_ref().unwrap(),
					Layout::ShaderReadOnlyOptimal,
				)),
			},
				&mut device_ref.device,
			);

			desc.write_to_state(DescSetWrite {
				binding: 1,
				array_offset: 0,
				descriptors: iter::once(Descriptor::Sampler(&sampler)),
			},
				&mut device_ref.device,
			);

			let mut transfer_image_fence = device_ref.device.create_fence(false).expect("Cannot create fence");

			{
				let mut command_buffer = staging_pool.allocate_one(Level::Primary);
				command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

				let image_barrier = Barrier::Image {
					states: (Access::empty(), Layout::Undefined)
						..(Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
					target: image.image.as_ref().unwrap(),
					families: None,
					range: SubresourceRange {
						aspects: Aspects::COLOR,
						..Default::default()
					},
				};
				command_buffer.pipeline_barrier(
					PipelineStage::TOP_OF_PIPE..PipelineStage::TRANSFER,
					Dependencies::empty(),
					iter::once(image_barrier),
				);

				command_buffer.copy_buffer_to_image(
					buffer.as_ref().unwrap().get(),
					image.image.as_ref().unwrap(),
					Layout::TransferDstOptimal,
					iter::once(BufferImageCopy {
						buffer_offset: 0,
						buffer_width: row_pitch / (stride as u32),
						buffer_height: dimensions.height as u32,
						image_layers: SubresourceLayers {
							aspects: Aspects::COLOR,
							level: 0,
							layers: 0..1,
						},
						image_offset: Offset { x: 0, y: 0, z: 0 },
						image_extent: Extent {
							width: dimensions.width,
							height: dimensions.height,
							depth: 1,
						},
					}),
				);

				let image_barrier = Barrier::Image {
					states: (Access::TRANSFER_WRITE, Layout::TransferDstOptimal)
						..(Access::SHADER_READ, Layout::ShaderReadOnlyOptimal),
					target: image.image.as_ref().unwrap(),
					families: None,
					range: SubresourceRange {
						aspects: Aspects::COLOR,
						..Default::default()
					},
				};

				command_buffer.pipeline_barrier(
					PipelineStage::TRANSFER..PipelineStage::FRAGMENT_SHADER,
					Dependencies::empty(),
					iter::once(image_barrier),
				);

				command_buffer.finish();

				device_ref.queues.queues[0].submit(
					iter::once(&command_buffer),
					iter::empty(),
					iter::empty(),
					Some(&mut transfer_image_fence),
				);
			}

			ImageBuffer {
				desc: desc,
				buffer: buffer,
				sampler: Some(sampler),
				image: Some(image),
				transferred_image_fence: Some(transfer_image_fence),
			}
		};

		image_buffer
	}

	pub fn wait_for_transfer_completion(&self){
		let device = &self.desc.layout.device.borrow().device;
		unsafe{
			device.wait_for_fence(self.transferred_image_fence.as_ref().unwrap(), !0)
				.unwrap();
		}
	}
}

impl<B: Backend> Image<B> {
	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, adapter: &CoreAdapter<B>, dimensions: Extent, format: Format, tiling: Tiling, usage: Usage, memory_properties: Properties, aspects: Aspects, view_usage: Usage) -> Self {
		let device = &device_ptr.borrow().device;
		let kind = Kind::D2(dimensions.width, dimensions.height, 1, 1);
		let mut image = unsafe {
			device.create_image(
				kind,
				1,
				format,
				tiling,
				usage,
				SparseFlags::empty(),
				ViewCapabilities::empty(),
			).unwrap()
		};

		let req = unsafe { device.get_image_requirements(&image) };

		let device_type = adapter
			.memory_types
			.iter()
			.enumerate()
			.position(|(id, memory_type)| {
				req.type_mask & (1 << id) != 0 && memory_type.properties.contains(memory_properties)
			})
			.unwrap()
			.into();

		let memory =  unsafe { device.allocate_memory(device_type, req.size).unwrap() };
		unsafe { device.bind_image_memory(&memory, 0, &mut image).unwrap(); }
		let image_view = unsafe {
			device.create_image_view(
				&image,
				ViewKind::D2,
				format,
				Swizzle::NO,
				view_usage,
				SubresourceRange {
					aspects,
					level_start: 0,
					level_count: Some(1),
					layer_start: 0,
					layer_count: Some(1),
				},
			)
		}
			.unwrap();

		Image {
			image: Some(image),
			memory: Some(memory),
			image_view: Some(image_view),
			format,
			device: device_ptr.clone(),
		}
	}
}

impl<B: Backend> Drop for Image<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_image_view(self.image_view.take().unwrap());
			device.destroy_image(self.image.take().unwrap());
			device.free_memory(self.memory.take().unwrap());
		}
	}
}