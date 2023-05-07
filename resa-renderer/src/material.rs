use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use crate::render_passes_and_pipelines::RenderStage;
use crate::render_resources::texture_buffer_library::TBORef;

#[derive(Hash)]
#[derive(Copy, Clone)]
pub struct Color {
	pub r: u8,
	pub g: u8,
	pub b: u8,
	pub a: u8,
}

impl Color {
	pub fn new(r: u8, g: u8, b: u8, a: u8) -> Self {
		Color {
			r,
			g,
			b,
			a,
		}
	}

	pub fn add(&mut self, other: &Color){

		let new_r: u16 = (self.r as u16) + (other.r as u16);
		let new_g: u16 = (self.g as u16) + (other.g as u16);
		let new_b: u16 = (self.b as u16) + (other.b as u16);
		let new_a: u16 = (self.a as u16) + (other.a as u16);

		self.r = if new_r > 255 {255} else {new_r as u8};
		self.g = if new_g > 255 {255} else {new_g as u8};
		self.b = if new_b > 255 {255} else {new_b as u8};
		self.a = if new_a > 255 {255} else {new_a as u8};
	}
}

#[derive(Clone, Hash)]
pub enum Texture{
	None,
	Pending(Vec<u8>),
	Some(TBORef)
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default)]
pub struct MaterialRef(pub(crate) usize);

#[derive(Hash)]
#[derive(Clone)]
pub struct Material {
	pub shader_id: u32,
	pub render_stage: RenderStage,
	pub color: Color,
	pub texture: Texture,
}

impl Material {
	pub(crate) fn get_ubo_data(&self) -> Vec<f32> {
		vec![
			(self.color.r as f32 / 255.0),
			(self.color.g as f32 / 255.0),
			(self.color.b as f32 / 255.0),
			(self.color.a as f32 / 255.0),
		]
	}

	pub(crate) fn equals(&self, other: &Material) -> bool{
		let mut hasher = DefaultHasher::new();
		self.hash(&mut hasher);
		let own_hash = hasher.finish();

		let mut other_hasher = DefaultHasher::new();
		other.hash(&mut other_hasher);
		let other_hash = other_hasher.finish();

		own_hash == other_hash
	}
}

