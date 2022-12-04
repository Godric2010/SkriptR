use std::borrow::{Borrow, BorrowMut};
use std::cell::{Ref, RefCell, RefMut};
use std::ops::Index;
use crate::component::ComponentVec;


pub struct World {
    entities_count: usize,
    component_vecs: Vec<Box<dyn ComponentVec>>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            component_vecs: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        for component_vec in self.component_vecs.iter_mut() {
            component_vec.push_none();
        }
        self.entities_count += 1;
        entity_id
    }

    pub fn add_component<ComponentType: 'static>(&mut self, entity: usize, component: ComponentType) {

        // Search for the component vec belonging to this entity and add the component
        for component_vec in self.component_vecs.iter_mut() {
            if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>() {
                component_vec.get_mut()[entity] = Some(component);
                return;
            }
        }

        // No matching component storage exists? Create a new one!\
        // Fill all entities with a none value for this component type
        let mut new_component_vec: Vec<Option<ComponentType>> = Vec::with_capacity(self.entities_count);
        for _ in 0..self.entities_count {
            new_component_vec.push(None);
        }

        // Give this entity the required component!
        new_component_vec[entity] = Some(component);
        self.component_vecs.push(Box::new(RefCell::new(new_component_vec)))
    }

    pub fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<Ref<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(component_vec.borrow());
            }
        }
        None
    }

    pub fn borrow_component_vec_mut<ComponentType: 'static>(&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
        for component_vec in self.component_vecs.iter() {
            if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                return Some(component_vec.borrow_mut());
            }
        }
        None
    }

    pub fn borrow_component_from_entity<ComponentType: 'static>(&self, entity: usize) -> Option<&ComponentType> {
        let components = self.borrow_component_vec_mut::<ComponentType>().unwrap();
        let mut target = None;
        for component in components.iter().enumerate() {
            if component.0 == entity {
                target = component.1.as_ref();
                break;
            }
        }
        target

        // if components.is_none() {
        //     return &None;
        // }

        // let component_vec = components.unwrap();
        // let component:&Option<ComponentType> = component_vec.index(entity);
        // component
    }
}