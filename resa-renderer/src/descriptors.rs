use std::cell::RefCell;
use std::iter;
use std::rc::Rc;
use gfx_hal::Backend;
use gfx_hal::device::Device;
use gfx_hal::pso::{Descriptor, DescriptorArrayIndex, DescriptorBinding, DescriptorPool, DescriptorSetLayoutBinding, DescriptorSetWrite};
use crate::core::CoreDevice;

pub struct DescSet<B: Backend> {
	pub set: Option<B::DescriptorSet>,
	pub layout: DescSetLayout<B>,
}

pub struct DescSetWrite<W>{
	pub binding: DescriptorBinding,
	pub array_offset: DescriptorArrayIndex,
	pub descriptors: W,
}

pub struct DescSetLayout<B: Backend>{
	layout: Option<B::DescriptorSetLayout>,
	pub device: Rc<RefCell<CoreDevice<B>>>
}

impl<B: Backend> DescSet<B> {
	pub fn write_to_state<'a, 'b: 'a, W>(&'b mut self, d: DescSetWrite<W>, device: &mut B::Device) where W: Iterator<Item = Descriptor<'a,B>>,{
		let set = self.set.as_mut().unwrap();
		unsafe {
			device.write_descriptor_set(DescriptorSetWrite {
				binding: d.binding,
				array_offset: d.array_offset,
				descriptors: d.descriptors,
				set
			});
		}
	}

	pub fn get_layout(&self) -> &B::DescriptorSetLayout{
		self.layout.layout.as_ref().unwrap()
	}
}

impl<B: Backend> DescSetLayout<B> {

	pub fn new(device: Rc<RefCell<CoreDevice<B>>>, bindings: Vec<DescriptorSetLayoutBinding>) -> Self{
		let desc_set_layout = unsafe{ device
			.borrow()
			.device
			.create_descriptor_set_layout(bindings.into_iter(), iter::empty())
			.ok()};

		DescSetLayout{
			layout: desc_set_layout,
			device,
		}
	}

	pub fn create_desc_set(self, desc_pool: &mut B::DescriptorPool, name: &str, device: Rc<RefCell<CoreDevice<B>>>) -> DescSet<B>{
		let mut desc_set = unsafe{desc_pool.allocate_one(self.layout.as_ref().unwrap()).unwrap()};
		unsafe{ device.borrow().device.set_descriptor_set_name(&mut desc_set, name);}
		DescSet{
			layout: self,
			set: Some(desc_set),
		}
	}
}

impl<B: Backend> Drop for DescSetLayout<B> {
	fn drop(&mut self) {
	let device = &self.device.borrow().device;
		unsafe {
			device.destroy_descriptor_set_layout(self.layout.take().unwrap());
		}
	}
}