use std::mem::size_of;

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct PushConstants {
    pub projection: [[f32;4];4],
    pub view: [[f32;4]],
    pub model: [[f32; 4]; 4],
    pub color: [f32; 4],
}

impl PushConstants {
    pub fn push_constant_bytes(&self) -> &[u32] {
        let size_in_bytes = size_of::<PushConstants>();
        let size_in_u32s = size_in_bytes / size_of::<u32>();
        let start_ptr = self as *const PushConstants as *const u32;
        unsafe { std::slice::from_raw_parts(start_ptr, size_in_u32s) }
    }
}
