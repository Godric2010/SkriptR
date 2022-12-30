use std::collections::HashMap;
use crate::archetype::Archetype;
use crate::entity::{Entity, EntityGenerator};

pub struct World {
    entity_generator: EntityGenerator,
    entity_location_map: HashMap<Entity, usize>,
    archetypes: Vec<Archetype>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entity_generator: EntityGenerator::new(),
            entity_location_map: HashMap::new(),
            archetypes: Vec::new(),
        }
    }

    pub fn new_entity(&mut self) -> Entity {
        let entity_id = self.entity_generator.spawn();
        let mut archetype = Archetype::new_from_columns(Archetype::builder());

        let archetype_index: usize;
        match self.archetypes.iter().position(|at| at.type_id == archetype.type_id) {
            Some(index) => {
                self.archetypes[index].entities.push(entity_id);
                archetype_index = index;
            }
            None => {
                archetype.entities.push(entity_id);
                self.archetypes.push(archetype);
                archetype_index = self.archetypes.len() - 1;
            }
        }

        self.entity_location_map.insert(entity_id, archetype_index);

        entity_id
    }

    pub fn remove_entity(&mut self, entity: Entity) {
        match self.entity_location_map.get(&entity) {
            Some(index) => {
                self.archetypes[*index].remove_entity(&entity);
            }
            None => panic!("Attempted to delete an entity which is not referenced by any archetype!"),
        };

        self.entity_location_map.remove_entry(&entity);

        self.entity_generator.desapwn(entity);
    }

    pub fn add_component<ComponentType: 'static>(&mut self, entity: &Entity, component: ComponentType) {
        let old_archetype_index = match self.entity_location_map.get(entity) {
            Some(index) => *index,
            None => panic!("Attemted to access a non existing entity!"),
        };

        let new_archetype = Archetype::new_from_add::<ComponentType>(&self.archetypes[old_archetype_index]);

        let migration_target_index = self.find_matching_archetype_or_create_new(new_archetype);

        let (old_archetype, target_archetype) = World::index_twice::<Archetype>(&mut self.archetypes, old_archetype_index, migration_target_index);

        target_archetype.migrate_entity_from(old_archetype, entity);
        target_archetype.set_component_instance(component);
        *self.entity_location_map.get_mut(entity).unwrap() = migration_target_index;
    }

    pub fn remove_component<ComponentType: 'static>(&mut self, entity: &Entity) {
        let old_archetype_index = match self.entity_location_map.get(entity) {
            Some(index) => *index,
            None => panic!("Attemted to access a non existing entity!"),
        };

        let new_archetype = Archetype::new_from_remove::<ComponentType>(&self.archetypes[old_archetype_index]);

        let migration_target_index = self.find_matching_archetype_or_create_new(new_archetype);

        let (old_archetype, target_archetype) = World::index_twice::<Archetype>(&mut self.archetypes, old_archetype_index, migration_target_index);

        target_archetype.migrate_entity_from(old_archetype, entity);
        *self.entity_location_map.get_mut(entity).unwrap() = migration_target_index;
    }

    pub fn get_component<ComponentType: 'static>(&self, entity: &Entity) -> Option<&ComponentType>{
        let archetype_index = *self.entity_location_map.get(entity)?;
        let entity_index = self.archetypes[archetype_index].entities.iter().position(|e| e == entity)?;
        let component_instance = self.archetypes[archetype_index].get_component_instance::<ComponentType>(entity_index)?;


       Some(component_instance)
    }

    pub fn get_component_mut<ComponentType: 'static>(&mut self, entity: &Entity) -> Option<&mut ComponentType>{

        let archetype_index = *self.entity_location_map.get(entity)?;
        let entity_index = self.archetypes[archetype_index].entities.iter().position(|e| e== entity)?;
        let component_instance = self.archetypes[archetype_index].get_component_instance_mut::<ComponentType>(entity_index)?;
        Some(component_instance)
    }

    pub fn get_all_components_of_type<ComponentType: 'static>(&self) -> Option<Vec<(&ComponentType, Entity)>>{

        let mut all_instances = Vec::<(&ComponentType, Entity)>::new();
        for archetype in self.archetypes.iter() {
            if !archetype.has_component_type::<ComponentType>(){
                continue
            }

            for (instance, entity) in archetype.get_components::<ComponentType>()?{
                all_instances.push((instance, entity));
            }
        }

        if all_instances.len() == 0{
            return None;
        }

        Some(all_instances)

    }

    /* pub fn remove_entity(&mut self, entity: Entity){
         for component_vec in self.component_vecs.iter_mut(){
             component_vec.set_none(entity)
         }
     }

     pub fn add_component<ComponentType: 'static>(&mut self, entity: Entity, component: ComponentType) {

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

     pub fn remove_component<ComponentType: 'static>(&mut self, entity: Entity){
         for component_vec in self.component_vecs.iter_mut(){
             if let Some(component_vec) = component_vec.as_any_mut().downcast_mut::<RefCell<Vec<Option<ComponentType>>>>(){
                 component_vec.get_mut()[entity] = None;
                 return;
             }
         }
     }

     // pub fn borrow_component_vec<ComponentType: 'static>(&self) -> Option<Ref<Vec<Option<ComponentType>>>> {
     //     for component_vec in self.component_vecs.iter() {
     //         if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
     //             return Some(component_vec.borrow());
     //         }
     //     }
     //     None
     // }

     pub fn borrow_component_vec_mut<ComponentType: 'static>(&self) -> Option<RefMut<Vec<Option<ComponentType>>>> {
         for component_vec in self.component_vecs.iter() {
             if let Some(component_vec) = component_vec.as_any().downcast_ref::<RefCell<Vec<Option<ComponentType>>>>() {
                 return Some(component_vec.borrow_mut());
             }
         }
         None
     }

 */

    fn find_matching_archetype_or_create_new(&mut self, new_archetype: Archetype) -> usize {
        let matching_archetype_index = match self.archetypes.iter().position(|at| at.type_id == new_archetype.type_id) {
            Some(index) => index,
            None => {
                let index = self.archetypes.len();
                self.archetypes.push(new_archetype);
                index
            }
        };
        matching_archetype_index
    }

    fn index_twice<T>(slice: &mut [T], first: usize, second: usize) -> (&mut T, &mut T) {
        if first < second {
            let (a, b) = slice.split_at_mut(second);
            (&mut a[first], &mut b[0])
        } else {
            let (a, b) = slice.split_at_mut(first);
            (&mut b[0], &mut a[second])
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::world::World;

    #[test]
    fn add_with_same_signature() {
        let mut world = World::new();
        world.new_entity();
        world.new_entity();

        assert_eq!(world.archetypes.len(), 1);
    }

    #[test]
    fn add_and_remove_entities() {
        let mut world = World::new();
        let entity_a = world.new_entity();
        let _entity_b = world.new_entity();

        world.remove_entity(entity_a);

        assert_eq!(world.archetypes.len(), 1);
        assert_eq!(world.entity_generator.is_alive(entity_a), false);
        assert_eq!(world.archetypes[0].entities.len(), 1);
    }

    struct Mock(i32);

    #[test]
    fn add_component_to_entity() {
        let mut world = World::new();
        let entity_a = world.new_entity();
        let entity_b = world.new_entity();
        let entity_c = world.new_entity();

        let mock = Mock(42);
        world.add_component(&entity_a, mock);
        let mock2 = Mock(2);
        world.add_component(&entity_c, mock2);

        assert_eq!(world.archetypes.len(), 2);
        assert!(world.archetypes[0].entities.contains(&entity_b));
        assert!(!world.archetypes[0].entities.contains(&entity_a));
        assert!(world.archetypes[1].entities.contains(&entity_a) && world.archetypes[1].entities.contains(&entity_c));
    }

    #[test]
    fn remove_component_from_entity() {
        let mut world = World::new();
        let entity_a = world.new_entity();
        let entity_b = world.new_entity();

        let mock_a = Mock(10);
        let mock_b = Mock(20);

        world.add_component(&entity_a, mock_a);
        world.add_component(&entity_b, mock_b);

        world.remove_component::<Mock>(&entity_a);

        assert_eq!(world.archetypes.len(), 2);
        assert!(world.archetypes[0].entities.contains(&entity_a));
        assert!(world.archetypes[1].entities.contains(&entity_b) && !world.archetypes[1].entities.contains(&entity_a));
    }
}