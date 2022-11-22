use crate::entity::Entity;

pub struct World {
    entities_count: usize,
    pub entities: Vec<Entity>,
}

impl World {
    pub fn new() -> Self {
        Self {
            entities_count: 0,
            entities: Vec::new(),
        }
    }

    pub fn add_entity(&mut self) -> usize {
        let entity_id = self.entities_count;
        self.entities.push(Entity::new(entity_id));
        self.entities_count += 1;
        entity_id
    }

    pub fn remove_entity(&mut self, id: usize) {
        let entity_to_remove = self.entities.iter().position(|e| e.id == id);
        if entity_to_remove.is_some() {
            self.entities.remove(entity_to_remove.unwrap());
        }
    }

    pub fn get_entity(&self, id: usize) -> Option<&Entity>{
       self.entities.iter().find(|e| e.id == id)
    }
}
