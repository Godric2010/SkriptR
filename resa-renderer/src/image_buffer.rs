use std::iter;
use std::rc::Rc;
use gfx_hal::{Backend, buffer};
use gfx_hal::command::{BufferImageCopy, CommandBuffer, CommandBufferFlags, Level};
use gfx_hal::device::Device;
use gfx_hal::format::{AsFormat, Aspects, Rgba8Srgb, Swizzle};
use gfx_hal::image::{Access, Extent, Filter, Kind, Layout, Offset, SamplerDesc, Size, SubresourceLayers, SubresourceRange, Tiling, Usage, ViewCapabilities, ViewKind, WrapMode};
use gfx_hal::memory::{Barrier, Dependencies, Properties, SparseFlags};
use gfx_hal::pool::CommandPool;
use gfx_hal::pso::{Descriptor, PipelineStage};
use gfx_hal::queue::Queue;
use crate::buffer::Buffer;
use crate::core::{CoreAdapter, CoreDevice};
use crate::descriptors::{DescSet, DescSetWrite};

pub type ColorFormat = Rgba8Srgb;

pub struct Dimensions<T> {
	pub width: T,
	pub height: T,
}

pub struct ImageBuffer<B: Backend> {
	desc: DescSet<B>,
	buffer: Option<Buffer<B>>,
	sampler: Option<B::Sampler>,
	image_view: Option<B::ImageView>,
	image: Option<B::Image>,
	memory: Option<B::Memory>,
	transferred_image_fence: Option<B::Fence>,
}

impl<B: Backend> ImageBuffer<B> {
	pub fn new(mut desc: DescSet<B>, img: &image::ImageBuffer<::image::Rgba<u8>, Vec<u8>>, adapter: CoreAdapter<B>, usage: buffer::Usage, device_ptr: &mut CoreDevice<B>, staging_pool: &mut B::CommandPool) -> Self{
		let (buffer, dimensions, row_pitch, stride) = Buffer::new_texture(Rc::clone(&desc.layout.device), &mut device_ptr.device, img, &adapter, usage);

		let buffer = Some(buffer);
		let device = &mut device_ptr.device;

		let kind = Kind::D2(dimensions.width as Size, dimensions.height as Size, 1, 1);
		let mut image = unsafe {
			device.create_image(
				kind,
				1,
				ColorFormat::SELF,
				Tiling::Optimal,
				Usage::TRANSFER_DST | Usage::SAMPLED,
				SparseFlags::empty(),
				ViewCapabilities::empty(),
			).unwrap()
		};

		let image_buffer = unsafe {
			let req = device.get_image_requirements(&image);

			let device_type = adapter
				.memory_types
				.iter()
				.enumerate()
				.position(|(id, memory_type)| {
					req.type_mask & (1 << id) != 0 && memory_type.properties.contains(Properties::DEVICE_LOCAL)
				})
				.unwrap()
				.into();

			let memory = device.allocate_memory(device_type, req.size).unwrap();
			device.bind_image_memory(&memory, 0, &mut image).unwrap();
			let image_view = device.create_image_view(
				&image,
				ViewKind::D2,
				ColorFormat::SELF,
				Swizzle::NO,
				Usage::SAMPLED,
				SubresourceRange {
					aspects: Aspects::COLOR,
					..Default::default()
				},
			)
			                       .unwrap();

			let sampler = device.create_sampler(&SamplerDesc::new(Filter::Linear, WrapMode::Clamp)).expect(" Cannot create sampler!");

			desc.write_to_state(DescSetWrite {
				binding: 0,
				array_offset: 0,
				descriptors: iter::once(Descriptor::Image(
					&image_view,
					Layout::ShaderReadOnlyOptimal,
				)),
			},
				device,
			);

			desc.write_to_state(DescSetWrite {
				binding: 1,
				array_offset: 0,
				descriptors: iter::once(Descriptor::Sampler(&sampler)),
			},
				device,
			);

			let mut transfer_image_fence = device.create_fence(false).expect("Cannot create fence");

			{
				let mut command_buffer = staging_pool.allocate_one(Level::Primary);
				command_buffer.begin_primary(CommandBufferFlags::ONE_TIME_SUBMIT);

				let image_barrier = Barrier::Image {
					states: (Access::empty(), Layout::Undefined)
						..(Access::TRANSFER_WRITE, Layout::TransferDstOptimal),
					target: &image,
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
					&image,
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
					target: &image,
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

				device_ptr.queues.queues[0].submit(
					iter::once(&command_buffer),
					iter::empty(),
					iter::empty(),
					Some(&mut transfer_image_fence),
				);
			}

			ImageBuffer{
				desc: desc,
				buffer: buffer,
				sampler: Some(sampler),
				image_view: Some(image_view),
				image: Some(image),
				memory: Some(memory),
				transferred_image_fence: Some(transfer_image_fence)
			}
		};

		image_buffer
	}
}