use crate::component::{entitiy_component::Component, transform::Transform};

struct Entity{
    pub name: String,

    components: Vec<Box<dyn Component>>,
}

impl Entity {
    fn new() -> Self
    {
        let name = String::from("New Entity");
        let components: Vec<Box<dyn Component>> = vec![Box::new(Transform::new())];

        Self { name, components }
    }

    
}
