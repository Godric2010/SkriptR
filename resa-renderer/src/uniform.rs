use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::Backend;
use gfx_hal::buffer::{SubRange, Usage};
use gfx_hal::pso::Descriptor;
use crate::buffer::Buffer;
use crate::core::CoreDevice;
use crate::descriptors::{DescSet, DescSetWrite};

pub struct Uniform<B: Backend> {
	buffer: Option<Buffer<B>>,
	pub desc: Option<DescSet<B>>,
}

impl<B: Backend> Uniform<B> {
	pub fn new<T>(device: Rc<RefCell<CoreDevice<B>>>, memory_types: &[MemoryType], data: &[T], mut desc: DescSet<B>, binding: u32) -> Self where T: Copy {
		let buffer = Buffer::new(Rc::clone(&device), &data, Usage::UNIFORM, memory_types);
		let buffer = Some(buffer);

		desc.write_to_state(
			DescSetWrite {
				binding,
				array_offset: 0,
				descriptors:
				iter::once(Descriptor::Buffer(buffer.as_ref().unwrap().get(), SubRange::WHOLE)),
			},
			&mut device.borrow_mut().device,
		);

		Uniform {
			buffer,
			desc: Some(desc),
		}
	}

	pub fn get_layout(&self) -> &B::DescriptorSetLayout{
		self.desc.as_ref().unwrap().get_layout()
	}
}