use crate::rendering::mesh::Mesh;

pub struct MeshRenderer{
    pub mesh: Mesh,
}

impl MeshRenderer {
    pub fn new(mesh: Mesh) -> Self{
        Self{
            mesh,
        }
    }
}