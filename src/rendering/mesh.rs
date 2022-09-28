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

pub fn create_primitive_quad() -> Mesh {
    let mut vertices = vec![
        Vertex { position: [-0.1, -0.1, 0.0], uv: [0.0, 0.0] },
        Vertex { position: [-0.1, 0.1, 0.0], uv: [0.0, 1.0] },
        Vertex { position: [0.1, 0.1, 0.0], uv: [1.0, 1.0] },

        Vertex { position: [-0.1, -0.1, 0.0], uv: [0.0, 0.0] },
        Vertex { position: [0.1, 0.1, 0.0], uv: [1.0, 1.0] },
        Vertex { position: [0.1, -0.1, 0.0], uv: [1.0, 0.0] },
    ];
    let triangle_list = vec![[0, 1, 2], [0, 2, 3]];

    Mesh {
        vertices,
        triangle_list
    }
}
