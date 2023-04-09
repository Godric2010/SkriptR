use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::pso::{BufferDescriptorFormat, BufferDescriptorType, DescriptorPoolCreateFlags, DescriptorRangeDesc, DescriptorSetLayoutBinding, DescriptorType, ShaderStageFlags};
use crate::core::CoreDevice;
use crate::descriptors::{DescSet, DescSetLayout};
use crate::uniform::Uniform;

#[derive(Copy, Clone)]
pub struct UBORef(usize, usize);

struct Entry<B: Backend>{
	desc_pool: B::DescriptorPool,
	buffers: Vec<Uniform<B>>,
	capacity: u32,
}

pub(crate) struct UniformBufferLibrary<B: Backend>{
	entries: Vec<Entry<B>>,
	device_ptr: Rc<RefCell<CoreDevice<B>>>,
	memory_types: Vec<MemoryType>,
}

impl<B: Backend> UniformBufferLibrary<B>  {

	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>) -> Self{
		let mut instance = Self{
			entries: vec![],
			device_ptr,
			memory_types,
		};

		let mut pool = instance.create_descriptor_pool(1);
		let buffer = instance.create_uniform_buffer(&[1.0, 0.0, 0.4, 1.0], &mut pool);

		instance.entries.push(
			Entry{
				desc_pool: pool.unwrap(),
				buffers: vec![buffer],
				capacity: 1,
			}
		);

		instance
	}

	pub fn add_buffers(&mut self, data_packages: Vec<Vec<f32>>) -> Vec<UBORef>{

		let pool_id = self.entries.len();
		let mut ubo_refs = vec![];

		let pool_capacity = data_packages.len();
		let mut descriptor_pool = self.create_descriptor_pool(pool_capacity.clone());
		let mut buffers = vec![];
		for (index,data_package) in data_packages.iter().enumerate() {
			buffers.push(self.create_uniform_buffer(data_package, &mut descriptor_pool));
			ubo_refs.push(UBORef(pool_id.clone(), index));
		}

		let entry = Entry{
			capacity: pool_capacity as u32,
			desc_pool: descriptor_pool.unwrap(),
			buffers,
		};
		self.entries.push(entry);
		ubo_refs
	}

	pub fn get_uniform_buffer(&self, ubo_ref: &UBORef) -> &Uniform<B>{
		&self.entries[ubo_ref.0].buffers[ubo_ref.1]
	}

	pub(crate) fn get_default_uniform_ref()-> UBORef{
		UBORef(0,0)
	}

	fn create_descriptor_pool(&self, capacity: usize) -> Option<<B as Backend>::DescriptorPool> {
		let uniform_desc_pool = unsafe {
			self.device_ptr.borrow().device.create_descriptor_pool(
				capacity,
				iter::once(DescriptorRangeDesc {
					ty: DescriptorType::Buffer {
						ty: BufferDescriptorType::Uniform,
						format: BufferDescriptorFormat::Structured {
							dynamic_offset: false,
						},
					},
					count: capacity,
				}),
				DescriptorPoolCreateFlags::empty(),
			)
		}.ok();
		uniform_desc_pool
	}

	fn create_descriptor(&self, descriptor_pool: &mut Option<<B as Backend>::DescriptorPool>) -> DescSet<B>{
		let uniform_desc = DescSetLayout::new(
			Rc::clone(&self.device_ptr),
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
		let uniform_desc = uniform_desc.create_desc_set(
			descriptor_pool.as_mut().unwrap(),
			"uniform",
			Rc::clone(&self.device_ptr),
		);
		uniform_desc
	}

	fn create_uniform_buffer(&self, data: &[f32], descriptor_pool: &mut Option<<B as Backend>::DescriptorPool>) -> Uniform<B>{

		let uniform_desc = self.create_descriptor(descriptor_pool);

		let uniform = Uniform::new(
			Rc::clone(&self.device_ptr),
			&self.memory_types,
			data,
			uniform_desc,
			0,
		);

		uniform
	}
}