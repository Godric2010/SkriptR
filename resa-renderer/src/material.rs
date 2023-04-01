use crate::graphics_pipeline::PipelineType;
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher}; 

#[derive(Hash)]
#[derive(Copy, Clone)]
pub struct Color {
	r: u8,
	g: u8,
	b: u8,
	a: u8,
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
		self.r += other.r;
		self.g += other.g;
		self.b += other.b;
		self.a += other.a;
	}
}

#[derive(Eq, PartialEq, Hash, Copy, Clone, Default)]
pub struct MaterialRef(pub(crate) usize);

#[derive(Hash)]
#[derive(Copy, Clone)]
pub struct Material {
	pub shader_id: u32,
	pub pipeline_type: PipelineType,
	pub color: Color,
	pub texture: Option<usize>,
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

	// TODO: Implement set color and set texture functions here!
}

