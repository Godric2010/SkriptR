use std::cell::RefCell;
use std::mem::size_of;
use std::ptr;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::Backend;
use gfx_hal::buffer::Usage;
use gfx_hal::device::Device;
use gfx_hal::memory::{Properties, Segment, SparseFlags};
use crate::core::CoreDevice;

pub struct Buffer<B: Backend>{
	memory: Option<B::Memory>,
	buffer: Option<B::Buffer>,
	device: Rc<RefCell<CoreDevice<B>>>,
	size: u64,
}

impl<B: Backend> Buffer<B> {

	pub fn get(&self) -> &B::Buffer{
		self.buffer.as_ref().unwrap()
	}

	pub fn new<T>(device_ptr: Rc<RefCell<CoreDevice<B>>>, data_source: &[T], usage: Usage, memory_types: &[MemoryType]) -> Self where T: Copy{

		let mut memory: B::Memory;
		let mut buffer: B::Buffer;
		let size: u64;

		let stride = size_of::<T>();
		let upload_size = data_source.len() * stride;

		unsafe {
			let device = &device_ptr.borrow().device;
			buffer = device.create_buffer(upload_size as u64, usage, SparseFlags::empty()).unwrap();
			let mem_req = device.get_buffer_requirements(&buffer);

			let upload_type = memory_types
				.iter()
				.enumerate()
				.position(|(id, mem_type)|{
					mem_req.type_mask & (1 << id) != 0 && mem_type.properties.contains(Properties::CPU_VISIBLE | Properties::COHERENT)
				})
				.unwrap()
				.into();

			memory = device.allocate_memory(upload_type, mem_req.size).unwrap();
			device.bind_buffer_memory(&memory, 0, &mut buffer).unwrap();
			size = mem_req.size;

			let mapping = device.map_memory(&mut memory, Segment::ALL).unwrap();
			ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
			device.unmap_memory(&mut memory);
		}

		Buffer{
			memory: Some(memory),
			buffer: Some(buffer),
			device: device_ptr,
			size,
		}
	}

	pub fn update_data<T>(&mut self, offset: u64, data_source: &[T]) where T: Copy{
		let device = &self.device.borrow().device;
		let stride = size_of::<T>();
		let upload_size = data_source.len() * stride;

		assert!(offset + upload_size as u64 <=self.size);
		let memory = self.memory.as_mut().unwrap();

		unsafe{
			let mapping = device.map_memory(memory, Segment{offset, size: None}).unwrap();
			ptr::copy_nonoverlapping(data_source.as_ptr() as *const u8, mapping, upload_size);
			device.unmap_memory(memory);
		}
	}

	// pub fn new_texture(device_ptr: Rc<RefCell<CoreDevice<B>>>, device : &B::Device, )

}

impl<B: Backend> Drop for Buffer<B> {
	fn drop(&mut self) {
		let device = &self.device.borrow().device;
		unsafe {
			device.destroy_buffer(self.buffer.take().unwrap());
			device.free_memory(self.memory.take().unwrap());
		}
	}
}