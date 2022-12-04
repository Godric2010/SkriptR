/*use bitset::BitSet;

pub struct Entity{
    pub id: usize,
    signature: Signature,
}

impl Entity {
    pub fn new(id: usize) -> Self{
        Self{
            id,
            signature: Signature::new(),
        }
    }
}

pub struct Signature{
    bit_set: BitSet,
}

impl Signature {
    fn new() -> Self{ Signature{bit_set:BitSet::new()}}
}

pub struct EntityManager{
   entities: Vec<Entity>,
}


impl EntityManager{
    pub fn new() -> Self{
        EntityManager{
            entities: Vec::new(),
        }
    }

    pub fn create_entity(&mut self) -> &Entity{

        let index = self.entities.len();
        self.entities.push(Entity::new(index));
        let entity = &self.entities.last().unwrap();
        // self.entities_count += 1;
        entity
    }

    pub fn destroy_entity(&mut self, entity: Entity){
        self.entities.remove(entity.id);
    }

    pub fn set_signature(&mut self, entity: &Entity, signature: Signature){
        let index = match self.entities.iter().position(|e| e.id == entity.id){
            Some(index) => index,
            None => {println!("Entity {} was not found! Cannot apply signature!", entity.id); return;}
        };
        self.entities[index].signature = signature;

    }

    pub fn get_signature(&self, entity: &Entity) -> Option<&Signature>{
        let index = self.entities.iter().position(|e| e.id == entity.id)?;

        Some(&self.entities[index].signature)
        None
    }

    // https://austinmorlan.com/posts/entity_component_system/#the-entity Use this as reference!
}*/