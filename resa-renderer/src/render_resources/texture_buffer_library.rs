use std::cell::{RefCell};
use std::io::Cursor;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::{Backend, Limits};
use gfx_hal::buffer::Usage;
use gfx_hal::device::Device;
use gfx_hal::pool::CommandPoolCreateFlags;
use gfx_hal::pso::{DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ImageDescriptorType, ShaderStageFlags};
use image::{Rgba, RgbaImage};
use crate::core::CoreDevice;
use crate::descriptors::{DescSet, DescSetLayout};
use crate::image_buffer::ImageBuffer;
use crate::material::Texture;

#[derive(Copy, Clone, Hash)]
pub struct TBORef(usize, usize);

struct TextureEntry<B: Backend> {
	image_descriptor_pool: B::DescriptorPool,
	images: Vec<ImageBuffer<B>>,
	capacity: u32,
}

pub struct TextureBufferLibrary<B: Backend> {
	device_ptr: Rc<RefCell<CoreDevice<B>>>,
	memory_types: Vec<MemoryType>,
	adapter_limits: Limits,
	entries: Vec<TextureEntry<B>>,
}

impl<B: Backend> TextureBufferLibrary<B> {
	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>, adapter_limits: Limits) -> Self {
		let mut instance =
			Self {
				device_ptr,
				memory_types,
				adapter_limits,
				entries: vec![],
			};

		let mut pool = instance.create_image_desc_pool(1);
		let image_buffer = instance.create_image_buffer(vec![], &mut pool);

		instance.entries.push(TextureEntry {
			image_descriptor_pool: pool.unwrap(),
			images: vec![image_buffer],
			capacity: 1,
		});

		instance
	}

	pub fn add_texture_buffer(&mut self, new_tbos: Vec<Texture>) -> Vec<TBORef> {
		let required_capacity = new_tbos.len();
		let mut descriptor_pool = None;
		let mut tbo_refs = vec![];
		let mut new_image_buffers = vec![];
		let pool_index = self.entries.len();

		for new_tbo in new_tbos {
			let tbo_ref = match new_tbo {
				Texture::None => TBORef(0, 0),
				Texture::Some(tbo_ref) => tbo_ref,
				Texture::Pending(image_data) => {
					if descriptor_pool.is_none() {
						descriptor_pool = self.create_image_desc_pool(required_capacity);
					}
					let image_buffer = self.create_image_buffer(image_data, &mut descriptor_pool);
					let buffer_index = new_image_buffers.len();
					new_image_buffers.push(image_buffer);
					TBORef(pool_index, buffer_index)
				}
			};
			tbo_refs.push(tbo_ref);
		}

		if new_image_buffers.len() > 0 {
			self.entries.push(
				TextureEntry {
					image_descriptor_pool: descriptor_pool.unwrap(),
					images: new_image_buffers,
					capacity: required_capacity as u32,
				}
			)
		}

		tbo_refs
	}

	#[allow(unused)]
	pub fn update_texture_buffer(&mut self, texture_ref: &TBORef, new_texture_data: Texture) {
		println!("Texture changed!")
	}

	pub fn remove_texture_buffer(&mut self) {
		todo!()
	}

	pub fn get_texture_buffer(&self, texture_ref: &TBORef) -> &ImageBuffer<B> {
		&self.entries[texture_ref.0].images[texture_ref.1]
	}

	pub(crate) fn get_default_ref()->TBORef{
		TBORef(0,0)
	}

	fn create_image_desc_pool(&self, capacity: usize) -> Option<<B as Backend>::DescriptorPool> {
		let image_desc_pool = unsafe {
			self.device_ptr.borrow().device.create_descriptor_pool(
				capacity,
				vec![DescriptorRangeDesc {
					ty: DescriptorType::Image {
						ty: ImageDescriptorType::Sampled {
							with_sampler: false,
						},
					},
					count: capacity,
				},
					DescriptorRangeDesc {
						ty: DescriptorType::Sampler,
						count: capacity,
					},
				].into_iter(),
				DescriptorPoolCreateFlags::empty(),
			)
		}.ok();
		image_desc_pool
	}

	fn create_descriptor(&self, descriptor_pool: &mut Option<<B as Backend>::DescriptorPool>) -> DescSet<B> {
		let image_desc = DescSetLayout::new(
			Rc::clone(&self.device_ptr),
			vec![
				DescriptorSetLayoutBinding {
					binding: 0,
					ty: DescriptorType::Image {
						ty: ImageDescriptorType::Sampled {
							with_sampler: false,
						},
					},
					count: 1,
					stage_flags: ShaderStageFlags::FRAGMENT,
					immutable_samplers: false,
				},
				DescriptorSetLayoutBinding {
					binding: 1,
					ty: DescriptorType::Sampler,
					count: 1,
					stage_flags: ShaderStageFlags::FRAGMENT,
					immutable_samplers: false,
				},
			],
		);

		let image_desc = image_desc.create_desc_set(
			descriptor_pool.as_mut().unwrap(),
			"image",
			Rc::clone(&self.device_ptr),
		);
		image_desc
	}

	fn create_image_buffer(&self, rgba_image: Vec<u8>, descriptor_pool: &mut Option<<B as Backend>::DescriptorPool>) -> ImageBuffer<B> {
		let image_desc = self.create_descriptor(descriptor_pool);

		let mut staging_pool = unsafe {
			self.device_ptr.borrow().device.create_command_pool(
				self.device_ptr.borrow().queues.family,
				CommandPoolCreateFlags::empty(),
			)
		}.expect("Cannot create staging command pool");

		let img=  if rgba_image.len() > 0 {
			image::load(Cursor::new(&rgba_image[..]), image::ImageFormat::Png).unwrap().to_rgba8()
		}
		else {
			RgbaImage::from_pixel(1,1,Rgba::from([255,255,255,255]))
		};

		let image_buffer = ImageBuffer::new(
			image_desc,
			&img,
			&self.adapter_limits,
			&self.memory_types,
			Usage::TRANSFER_SRC,
			Rc::clone(&self.device_ptr),
			&mut staging_pool,
		);

		image_buffer.wait_for_transfer_completion();

		image_buffer
	}
}