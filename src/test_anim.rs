use std::cell::RefCell;
use std::rc::Rc;
use resa_ecs::entity::Entity;
use resa_ecs::world::World;
use crate::rendering::transform::Transform;

pub fn rotate_entity(world: &Rc<RefCell<World>>, entity: &Entity, delta_time: &f64){

	let mut world_binding = world.borrow_mut();
	let mut transform: &mut Transform = world_binding.get_component_mut::<Transform>(&entity).unwrap();
	transform.angle += (1.0 * (*delta_time)) as f32;

}