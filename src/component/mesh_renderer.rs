use std::any::Any;
use crate::component::entitiy_component::Component;
use crate::rendering::mesh::Mesh;

pub struct MeshRenderer{

    pub mesh: Mesh,

}

impl MeshRenderer{
    pub fn new(mesh: Mesh) -> Self{
        MeshRenderer{
            mesh
        }
    }
}

impl Component<'_> for MeshRenderer{
    fn enable(&mut self) {
        todo!()
    }

    fn disable(&mut self) {
        todo!()
    }

    fn update(&mut self) {

    }

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
       self
    }
}
