use std::collections::HashSet;

#[derive(Debug, Copy, Clone, Eq, PartialEq, Hash)]
pub struct Entity(u64);

/// Keeps track of entities in the world, as well as creates and destroys them.
pub (crate) struct EntityGenerator{
    next_id: u64,
    dead_entities: HashSet<Entity>,
}

impl EntityGenerator {

    pub(crate) fn new() -> Self{
        Self{
            next_id: 0,
            dead_entities: HashSet::new(),
        }
    }

    /// Create a new entity instance as long as less than u64::MAX entities have been created
    pub(crate) fn spawn(&mut self) -> Entity{
        let entity = Entity(self.next_id);
        if self.next_id == u64::MAX{
            panic!("Attempted to spawn an entity after running out of IDs");
        }
        self.next_id += 1;
        entity
    }

    /// Add an entity to the dead entries if it is currently alive
    pub(crate) fn desapwn(&mut self, entity: Entity){
        if self.is_alive(entity){
            self.dead_entities.insert(entity);
        }
    }

    /// Checks if an entity has been created and is currently in use
    pub(crate) fn is_alive(&self, entity: Entity) -> bool{

        if entity.0 >= self.next_id{
            panic!("Attemted to use an entity that has not been spawned yet!");
        }
        self.dead_entities.contains(&entity) == false
    }
}