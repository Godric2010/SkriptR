use std::mem::size_of;

pub(crate) struct MVP {
	pub model: [[f32; 4]; 4],
	pub view: [[f32; 4]; 4],
	pub proj: [[f32; 4]; 4],
}

impl MVP {
	pub fn as_bytes(&self) -> &[u32]{
		let size_in_bytes = size_of::<Self>();
		let size_in_u32s = size_in_bytes / size_of::<u32>();
		let start_ptr = self as *const Self as *const u32;
		unsafe { std::slice::from_raw_parts(start_ptr, size_in_u32s) }
	}
}