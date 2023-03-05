use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::fs;
use std::hash::{Hash, Hasher};
use std::path::Path;
use glsl_to_spirv::ShaderType;
use crate::graphics_pipeline::PipelineType;
use crate::graphics_pipeline::PipelineType::Opaque;
use crate::material::Material;
use crate::renderer::Renderer;

pub struct MaterialController {
	pub material_map: HashMap<u64, Material>,
	pub(crate) ubo_map: HashMap<u64, usize>,
	pub(crate) texture_map: HashMap<u64, usize>,
	pub(crate) shader_map: HashMap<u64, Vec<u32>>,
	pub(crate) pipeline_shader_map: HashMap<PipelineType, (u64, u64)>
}

impl MaterialController {
	pub fn new() -> Self {

		let (shader_map, pipeline_shader_map) = MaterialController::load_shaders();


		MaterialController {
			material_map: HashMap::new(),
			ubo_map: HashMap::new(),
			texture_map: HashMap::new(),
			shader_map,
			pipeline_shader_map,
		}
	}

	//TODO: Make this dynamic in the future!
	fn load_shaders() -> (HashMap<u64, Vec<u32>>, HashMap<PipelineType,(u64,u64)>){
		let mut shader_map = HashMap::new();
		let mut pipeline_shader_map = HashMap::new();

		let vert_path = "./src/rendering/shaders/base.vert";
		let frag_path = "./src/rendering/shaders/base.frag";

		shader_map.insert(0, create_shader(vert_path, ShaderType::Vertex).unwrap());
		shader_map.insert(1, create_shader(frag_path, ShaderType::Fragment).unwrap());

		pipeline_shader_map.insert(PipelineType::Opaque, (0, 1));


		(shader_map, pipeline_shader_map)
	}

	pub(crate) fn get_registred_pipeline_types(&self) -> Vec<&PipelineType>{
		let mut types = vec![];
		for (pipeline_type, _) in &self.pipeline_shader_map {
			types.push(pipeline_type);
		}
		types
	}

	pub(crate) fn add_new_materials(&mut self, materials: &[Material], renderer: &mut Renderer<backend::Backend>) -> Vec<u64> {
		let mut material_ids: Vec<u64> = Vec::new();

		let mut hasher = DefaultHasher::new();
		for material in materials {
			material.hash(&mut hasher);
			let material_hash = hasher.finish();
			let buffer_id = renderer.add_uniform_buffer(&material.get_ubo_data());

			self.material_map.insert(material_hash, *material);
			self.ubo_map.insert(material_hash, buffer_id);

			material_ids.push(material_hash);
		}

		let mut buffer_ids = vec![];
		for (_, buffer_id) in &self.ubo_map{
			buffer_ids.push(buffer_id.clone());
		}
		renderer.update_pipeline(&buffer_ids, &Opaque, &self);

		material_ids
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



fn create_shader(shader_path: &str, shader_type: ShaderType) -> Option<Vec<u32>> {
	let path = Path::new(shader_path);

	let glsl = match fs::read_to_string(&path) {
		Ok(glsl_shader) => glsl_shader,
		Err(e) => {
			println!("{}", e);
			return None;
		}
	};
	let file = match glsl_to_spirv::compile(&glsl, shader_type) {
		Ok(spirv_file) => spirv_file,
		Err(_) => return None,
	};

	match gfx_auxil::read_spirv(file) {
		Ok(spirv) => Some(spirv),
		Err(_) => None,
	}
}
