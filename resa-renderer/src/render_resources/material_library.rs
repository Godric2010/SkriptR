use std::collections::HashMap;
use std::io::Cursor;
use crate::graphics_pipeline::PipelineType;
use crate::material::{Material, MaterialRef};
use crate::renderer::Renderer;

pub struct MaterialLibrary {
	pub material_map: HashMap<MaterialRef, Material>,
	pub(crate) ubo_map: HashMap<MaterialRef, usize>,
	pub(crate) texture_map: HashMap<MaterialRef, usize>,
}

impl MaterialLibrary {
	pub fn new() -> Self {

		MaterialLibrary {
			material_map: HashMap::new(),
			ubo_map: HashMap::new(),
			texture_map: HashMap::new(),
		}
	}

	pub(crate) fn add_new_material(&mut self, material: Material, renderer: &mut Renderer<backend::Backend>) -> MaterialRef {
		let material_id = MaterialRef(self.material_map.len());
		//TODO: Consider using a callback to the renderer here...
		let buffer_id = renderer.add_uniform_buffer(&material.get_ubo_data());
		let texture_id = material.texture.clone();

		self.material_map.insert(material_id.clone(), material);
		self.ubo_map.insert(material_id.clone(), buffer_id);
		self.texture_map.insert(material_id.clone(), texture_id.unwrap_or(0));
		material_id
	}

	/* TODO: Implement material update function here! */

	pub(crate) fn add_new_texture(&mut self, image_data: Vec<u8>, renderer: &mut Renderer<backend::Backend>) -> usize {
		let img = image::load(Cursor::new(&image_data[..]), image::ImageFormat::Png).unwrap().to_rgba8();
		let buffer_index = renderer.add_image_buffer(img);
		buffer_index
	}

	pub(crate) fn find_all_buffers_of_pipeline_type(&self, pipeline_type: PipelineType) -> Vec<usize> {
		let mut buffer_ids = vec![];
		for (hash, material) in &self.material_map {
			if material.pipeline_type == pipeline_type {
				buffer_ids.push(self.ubo_map.get(hash).unwrap().clone());
			}
		}
		buffer_ids
	}
}
