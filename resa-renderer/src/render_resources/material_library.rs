use std::cell::RefCell;
use std::collections::HashMap;
use std::io::Cursor;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::Backend;
use crate::core::CoreDevice;
use crate::graphics_pipeline::PipelineType;
use crate::material::{Material, MaterialRef};
use crate::render_resources::uniform_buffer_library::{UBORef, UniformBufferLibrary};
use crate::renderer::Renderer;
use crate::uniform::Uniform;

struct MaterialEntry{
	material: Material,
	ubo_ref: UBORef,
	texture_ref: usize,
}

pub struct MaterialLibrary<B: Backend> {
	material_map: HashMap<MaterialRef, MaterialEntry>,
	pub material_map_old: HashMap<MaterialRef, Material>,
	pub(crate) ubo_map: HashMap<MaterialRef, usize>,
	pub(crate) texture_map: HashMap<MaterialRef, usize>,
	ubo_library: UniformBufferLibrary<B>,
	last_entry_id: usize,
}

impl<B: Backend> MaterialLibrary<B> {
	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>) -> Self {

		MaterialLibrary {
			material_map: HashMap::new(),
			material_map_old: HashMap::new(),
			ubo_map: HashMap::new(),
			texture_map: HashMap::new(),
			ubo_library: UniformBufferLibrary::new(device_ptr.clone(), memory_types.clone()),
			last_entry_id: 0,
		}
	}

	pub fn add_materials(&mut self, materials: &[Material]) -> Vec<MaterialRef>{

		let materials_ubo_data = materials.iter().map(|mat| mat.get_ubo_data()).collect();
		let ubo_refs = self.ubo_library.add_buffers(materials_ubo_data);

		let mut material_refs = vec![];
		for (index,material) in materials.iter().enumerate(){
			let material_ref = MaterialRef(self.last_entry_id);
			self.last_entry_id += 1;

			let entry = MaterialEntry{
				material: *material,
				ubo_ref: ubo_refs[index],
				texture_ref: material.texture.unwrap_or(0)
			};

			self.material_map.insert(material_ref, entry);
			material_refs.push(material_ref);
		}
		material_refs
	}

	pub fn update_material(&mut self, material_ref: &MaterialRef, new_material: Material){
		todo!()
	}

	pub fn remove_material(&mut self, material_ref: MaterialRef){
		todo!()
	}

	pub fn get_material(&self, material_ref: &MaterialRef) -> Option<&Material>{
		Some(&self.material_map.get(material_ref)?.material)
	}

	pub(crate) fn get_render_data(&self, material_ref: &MaterialRef) -> (&Uniform<B>, &usize){
		let entry = &self.material_map[material_ref];
		(self.ubo_library.get_uniform_buffer(&entry.ubo_ref), &entry.texture_ref)
	}

	pub(crate) fn add_new_material(&mut self, material: Material, renderer: &mut Renderer<backend::Backend>) -> MaterialRef {
		let material_id = MaterialRef(self.material_map_old.len());
		//TODO: Consider using a callback to the renderer here...
		let buffer_id = renderer.add_uniform_buffer(&material.get_ubo_data());
		let texture_id = material.texture.clone();

		self.material_map_old.insert(material_id.clone(), material);
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
		for (hash, material) in &self.material_map_old {
			if material.pipeline_type == pipeline_type {
				buffer_ids.push(self.ubo_map.get(hash).unwrap().clone());
			}
		}
		buffer_ids
	}
}
