pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub triangle_list: Vec<[u32; 3]>,
}

#[repr(C)]
#[derive(Debug, Copy, Clone)]
pub struct Vertex {
    position: [f32; 3],
    uv: [f32; 2],
}


