use crate::component::Component;

pub struct Entity{
    pub id: usize,
    pub components: Vec<Box<dyn Component>>,
}

impl Entity {
    pub fn new(id: usize) -> Self{
        Self{
            id,
            components: Vec::new(),
        }
    }

    pub fn add_component<T: 'static + Component>(&mut self, new_component: T){
       /* for component in self.components.iter_mut(){
            if let Some(component) = component.as_any().downcast_ref::<RefCell<Vec<Option<T>>>>(){
                component.borrow_mut()[0] = Some(new_component);
                return;
            }
        }*/

        self.components.push(Box::new(new_component))
    }
}