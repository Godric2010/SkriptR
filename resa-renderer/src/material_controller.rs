use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Cursor;
use crate::graphics_pipeline::PipelineType;
use crate::material::{Material, MaterialRef};
use crate::renderer::Renderer;
use crate::shader::ShaderRef;

pub struct MaterialController {
	pub material_map: HashMap<MaterialRef, Material>,
	pub(crate) ubo_map: HashMap<MaterialRef, usize>,
	pub(crate) texture_map: HashMap<MaterialRef,usize>,
	pipeline_shader_map: HashMap<PipelineType, usize>,
	shader_ref_list: Vec<ShaderRef>
}

impl MaterialController {
	pub fn new(shaders: Vec<ShaderRef>) -> Self {
		let mut pipeline_shader_map = HashMap::<PipelineType, usize>::new();
		pipeline_shader_map.insert(PipelineType::Opaque, 0);

	     MaterialController {
			material_map: HashMap::new(),
			ubo_map: HashMap::new(),
			texture_map: HashMap::new(),
			pipeline_shader_map,
			shader_ref_list: shaders,
		}
	}

	pub(crate) fn get_registered_pipeline_types(&self) -> Vec<&PipelineType>{
		let mut types = vec![];
		for (pipeline_type, _) in &self.pipeline_shader_map {
			types.push(pipeline_type);
		}
		types
	}

	// TODO: Move pipeline stuff into the pipeline!
	pub(crate) fn get_pipeline_shaders(&self, pipeline_type: &PipelineType) -> Option<&ShaderRef>{
		let shader_ref_id = self.pipeline_shader_map.get(pipeline_type)?;
		let shader_ref = self.shader_ref_list.get(*shader_ref_id)?;
		Some(shader_ref)
	}

	pub(crate) fn add_new_material(&mut self, material: Material, renderer: &mut Renderer<backend::Backend>) -> MaterialRef{
		let material_id = MaterialRef(self.material_map.len());
		//TODO: Consider using a callback to the renderer here...
		let buffer_id = renderer.add_uniform_buffer(&material.get_ubo_data());
		let texture_id = material.texture.clone();

		self.material_map.insert(material_id.clone(), material);
		self.ubo_map.insert(material_id.clone(), buffer_id);
		self.texture_map.insert(material_id.clone(), texture_id.unwrap_or(0));
		material_id
	}

	//TODO: Implement material update function here!

	pub(crate) fn add_new_texture(&mut self, image_data: Vec<u8>, renderer: &mut Renderer<backend::Backend>) -> usize{
		let img = image::load(Cursor::new(&image_data[..]), image::ImageFormat::Png).unwrap().to_rgba8();
		let buffer_index = renderer.add_image_buffer(img);
		buffer_index
	}

	pub(crate) fn find_all_buffers_of_pipeline_type(&self, pipeline_type: PipelineType) -> Vec<usize>{
		let mut buffer_ids = vec![];
		for (hash, material) in &self.material_map {
			if material.pipeline_type == pipeline_type{
				buffer_ids.push(self.ubo_map.get(hash).unwrap().clone());
			}
		}
		buffer_ids
	}
}
