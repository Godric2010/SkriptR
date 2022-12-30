use std::any::{Any, TypeId};
use std::borrow::{Borrow, BorrowMut};
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use crate::component::ComponentInstanceCollection;
use crate::entity::Entity;

pub struct Archetype {
    pub entities: Vec<Entity>,
    component_type_map: HashMap<TypeId, usize>,
    component_collections: Vec<Box<dyn ComponentInstanceCollection>>,
    pub type_id: u64,
}

impl Archetype {
    pub fn new_from_add<T: 'static>(from_archetype: &Archetype) -> Self {
        let mut columns: Vec<_> = from_archetype
            .component_collections
            .iter()
            .map(|column| column.new_empty_column())
            .collect();

        let mut type_map = from_archetype.component_type_map.clone();

        assert!(columns
            .iter()
            .find(|column| column.as_any().is::<Vec<T>>())
            .is_none());


        type_map.insert(TypeId::of::<T>(), columns.len());
        columns.push(Box::new(Vec::<T>::new()));

        let mut types_in_archetype = Vec::new();
        for type_id in type_map.keys() {
            types_in_archetype.push(*type_id);
        };

        let type_id = Archetype::calculate_archetype_hash(&types_in_archetype);
        Self {
            entities: Vec::new(),
            component_type_map: type_map,
            component_collections: columns,
            type_id,
        }
    }

    pub fn new_from_remove<T: 'static>(from_archetype: &Archetype) -> Self {
        let mut columns: Vec<_> = from_archetype
            .component_collections
            .iter()
            .map(|column| column.new_empty_column())
            .collect();

        let mut type_map = from_archetype.component_type_map.clone();
        let type_id_to_remove = TypeId::of::<T>();

        let index = *type_map.get(&type_id_to_remove).unwrap();
        // columns
        // .iter()
        // .position(|column| column.as_any().is::<Vec<T>>())
        // .unwrap();

        columns.remove(index);
        type_map.remove_entry(&type_id_to_remove);

        let mut types_in_archetype = Vec::new();
        for type_id in type_map.keys() {
            types_in_archetype.push(*type_id);
        };


        let type_id = Archetype::calculate_archetype_hash(&types_in_archetype);
        Self {
            entities: Vec::new(),
            component_type_map: type_map,
            component_collections: columns,
            type_id,
        }
    }

    pub fn builder() -> ColumnsBuilder {
        ColumnsBuilder(Vec::new(), Vec::new())
    }

    pub fn new_from_columns(columns: ColumnsBuilder) -> Self {
        let type_id = Archetype::calculate_archetype_hash(&columns.1);
        let mut hash_map: HashMap<TypeId, usize> = HashMap::new();
        for col in columns.1.iter().enumerate() {
            hash_map.insert(*col.1, col.0);
        }
        Self {
            entities: Vec::new(),
            component_type_map: hash_map,
            component_collections: columns.0,
            type_id,
        }
    }

    pub fn remove_entity(&mut self, entity: &Entity) {
        let entity_index = match self.entities.iter().position(|e| e == entity) {
            Some(index) => index,
            None => return,
        };
        self.entities.remove(entity_index);

        for component_type in self.component_collections.iter_mut(){
            component_type.remove_at(entity_index);
        }

    }

    pub(crate) fn set_component_instance<T: 'static>(&mut self, component_instance: T){
        let type_id = TypeId::of::<T>();
        let slot = *self.component_type_map.get(&type_id).unwrap();
        let component_vec = self.component_collections[slot].as_any_mut().downcast_mut::<Vec<T>>().unwrap();
        component_vec.push(component_instance);

    }

    pub(crate) fn get_component_instance<T: 'static>(&self, index: usize) -> Option<&T> {
        let type_id = TypeId::of::<T>();
        let slot = *self.component_type_map.get(&type_id)?;
        let component_vec = self.component_collections[slot].as_any().downcast_ref::<Vec<T>>()?;

        let amount_of_components = component_vec.len();
        if index >= amount_of_components {
            return None;
        }

        let component_instance = component_vec[index].borrow();

        Some(component_instance)
    }

    pub(crate) fn get_component_instance_mut<T: 'static>(&mut self, index: usize) -> Option<&mut T> {
        let type_id = TypeId::of::<T>();
        let slot = *self.component_type_map.get(&type_id)?;
        let component_vec = self.component_collections[slot].as_any_mut().downcast_mut::<Vec<T>>()?;
        let amount_of_components = component_vec.len();
        if index >= amount_of_components{
            return None;
        }

        let component_instance_mut = component_vec[index].borrow_mut();
        Some(component_instance_mut)
    }

    pub(crate) fn get_components<T: 'static>(&self) -> Option<Vec<(&T, Entity)>>{
        let type_id = TypeId::of::<T>();
        let slot = *self.component_type_map.get(&type_id)?;
        let component_vec = self.component_collections[slot].as_any().downcast_ref::<Vec<T>>()?;

        let mut references = Vec::<(&T, Entity)>::new();
        for (index, instance) in component_vec.iter().enumerate() {
            let entity = self.entities[index].clone();
            references.push((instance, entity));
        }
        Some(references)
    }

    pub(crate) fn get_components_mut<T: 'static>(&mut self) -> Option<Vec<(&mut T, Entity)>>{
        let type_id = TypeId::of::<T>();
        let slot = *self.component_type_map.get(&type_id)?;
        let component_vec = self.component_collections[slot].as_any_mut().downcast_mut::<Vec<T>>()?;

        let mut references = Vec::<(&mut T, Entity)>::new();
        for (index, instance) in component_vec.iter_mut().enumerate() {
            let entity = self.entities[index].clone();
            references.push((instance, entity));
        }
        Some(references)
    }

    pub(crate) fn has_component_type<T:'static>(&self) -> bool{
        let type_id = TypeId::of::<T>();
        match self.component_type_map.get(&type_id) {
            Some(_) => true,
            None => false,
        }
    }

    pub fn migrate_entity_from(&mut self, from: &mut Archetype, entity: &Entity) {

        let entity_index = from.entities.iter().position(|e| e == entity).unwrap();

        for (index, component_collections) in from.component_collections.iter_mut().enumerate(){
            component_collections.migrate(entity_index, &mut *self.component_collections[index] )
        }

        from.entities.remove(entity_index);
        self.entities.push(*entity);
    }

    fn calculate_archetype_hash(columns: &Vec<TypeId>) -> u64 {
        let mut s = DefaultHasher::new();
        columns.hash(&mut s);
        s.finish()
    }
}

pub struct ColumnsBuilder(Vec<Box<dyn ComponentInstanceCollection>>, Vec<TypeId>);

impl ColumnsBuilder {
    #[allow(dead_code)]
    pub fn with_column_type<T: 'static>(mut self) -> Self {
        if let Some(_) = self.0.iter().find(|col| col.as_any().type_id() == TypeId::of::<Vec<T>>()) {
            panic!("Attempted to create an invalid archetype");
        }

        self.0.push(Box::new(Vec::<T>::new()));
        self.1.push(TypeId::of::<T>());
        self
    }
}
