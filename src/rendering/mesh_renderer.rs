use crate::rendering::mesh::Mesh;

pub struct MeshRenderer {
    pub mesh: Mesh,
    pub color: [f32; 4],
}

impl MeshRenderer {
    pub fn new(mesh: Mesh, color: [f32; 4]) -> Self {
        Self {
            mesh,
            color,
        }
    }
}