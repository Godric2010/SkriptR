use crate::component::{entitiy_component::Component, transform::Transform, mesh_renderer::MeshRenderer};
use crate::rendering::mesh;

pub struct Entity {
    pub name: String,

    components: Vec<Box<dyn for<'a> Component<'a>>>,
}

impl Entity {
    pub fn new() -> Self
    {
        let name = String::from("New Entity");
        let primitive = mesh::create_primitive_quad();
        let components: Vec<Box<dyn for<'a> Component<'a> + 'static>> = vec![Box::new(Transform::new()), Box::new(MeshRenderer::new(primitive))];

        Self { name, components }
    }

    pub fn get_transform(&mut self) -> &mut Transform {
        let index = 0;

        let transform_boxed = &mut self.components[index];
        let transform = match transform_boxed.as_any_mut().downcast_mut::<Transform>() {
            Some(transform) => transform,
            None => panic!("Transform is not at index 0!"),
        };
        transform
    }

    pub fn get_component<'a, C: Component<'a> + 'static>(&self) -> Option<&C> {
        let mut target_component: Option<&C> = None;

        for c in &self.components {
            let target:&C = match c.as_any().downcast_ref::<C>() {
                Some(t) => t,
                None => continue,
            };
            target_component = Some(target);
            break;
        }
        target_component
    }

    fn enable(&mut self){
        for component in &mut self.components {
            component.enable()
        }

    }

    pub fn update(&mut self){
        for component in &mut self.components {
            component.update();
        }
    }
}
