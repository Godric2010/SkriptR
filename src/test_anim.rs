use std::cell::RefCell;
use std::rc::Rc;
use resa_ecs::entity::Entity;
use resa_ecs::world::World;
use resa_renderer::material::Color;
use crate::rendering::mesh_renderer::MeshRenderer;
use crate::rendering::transform::Transform;

pub fn rotate_entity(world: &Rc<RefCell<World>>, entity: &Entity, delta_time: &f64){

	let mut world_binding = world.borrow_mut();
	let mut transform: &mut Transform = world_binding.get_component_mut::<Transform>(&entity).unwrap();
	transform.angle += (1.0 * (*delta_time)) as f32;

}

pub fn change_color(world: &Rc<RefCell<World>>, entity: &Entity, delta_time: &f64){
	let mut world_binding = world.borrow_mut();
	let mesh_renderer: &mut MeshRenderer = world_binding.get_component_mut::<MeshRenderer>(&entity).unwrap();
	let mut material = mesh_renderer.get_material();

	let red = 1.0  ;
	let add_color = Color::new(red as u8, 0, 0, 0);
	material.color.add(&add_color);

	if material.color.r == 255{
		material.color.r = 0;
	}

	mesh_renderer.update_material(material);
}