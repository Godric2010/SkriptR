use crate::graphics_pipeline::PipelineType;

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
}

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
}

