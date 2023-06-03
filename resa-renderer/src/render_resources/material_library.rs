use std::cell::RefCell;
use std::collections::HashMap;
use std::rc::Rc;
use gfx_hal::adapter::MemoryType;
use gfx_hal::{Backend, Limits};
use crate::core::CoreDevice;
use crate::image_buffer::ImageBuffer;
use crate::material::{Material, MaterialRef};
use crate::render_resources::texture_buffer_library::{TBORef, TextureBufferLibrary};
use crate::render_resources::uniform_buffer_library::{UBORef, UniformBufferLibrary};
use crate::render_stage::RenderStage;
use crate::uniform::Uniform;

struct MaterialEntry {
	material: Material,
	ubo_ref: UBORef,
	texture_ref: TBORef,
	render_stage_index: u16
}

pub struct MaterialLibrary<B: Backend> {
	material_map: HashMap<MaterialRef, MaterialEntry>,
	ubo_library: UniformBufferLibrary<B>,
	tbo_library: TextureBufferLibrary<B>,
	last_entry_id: usize,
}

impl<B: Backend> MaterialLibrary<B> {
	pub fn new(device_ptr: Rc<RefCell<CoreDevice<B>>>, memory_types: Vec<MemoryType>, adapter_limits: Limits) -> Self {
		MaterialLibrary {
			material_map: HashMap::new(),
			ubo_library: UniformBufferLibrary::new(device_ptr.clone(), memory_types.clone()),
			tbo_library: TextureBufferLibrary::new(device_ptr.clone(), memory_types.clone(), adapter_limits),
			last_entry_id: 0,
		}
	}

	pub fn add_materials(&mut self, materials: &[(Material, u16)]) -> Vec<MaterialRef> {
		let materials_ubo_data = materials.iter().map(|mat| (mat.0.get_ubo_data())).collect();
		let ubo_refs = self.ubo_library.add_buffers(materials_ubo_data);

		let materials_tbo_data = materials.iter().map(|mat| mat.0.texture.clone()).collect();
		let tbo_refs = self.tbo_library.add_texture_buffer(materials_tbo_data);

		let mut material_refs = vec![];
		for (index, (material, render_stage_id)) in materials.iter().enumerate() {
			let material_ref = MaterialRef(self.last_entry_id);
			self.last_entry_id += 1;

			let entry = MaterialEntry {
				material: material.clone(),
				ubo_ref: ubo_refs[index],
				texture_ref: tbo_refs[index],
				render_stage_index: render_stage_id.clone(),
			};

			self.material_map.insert(material_ref, entry);
			material_refs.push(material_ref);
		}
		material_refs
	}

	pub fn update_material(&mut self, material_ref: &MaterialRef, new_material: Material) {
		let mut entry = self.material_map.get_mut(material_ref).unwrap();

		self.ubo_library.update_buffer(&entry.ubo_ref, new_material.get_ubo_data().clone());
		// self.tbo_library.update_texture_buffer(&entry.texture_ref, new_material.texture.clone());
		entry.material = new_material;
	}

	#[allow(dead_code, unused)]
	pub fn remove_material(&mut self, material_ref: MaterialRef) {
		todo!()
	}

	pub fn get_material_ref_from_name(&self, name: &str) -> Option<MaterialRef>{
		for (key, value) in &self.material_map{
			if value.material.name == name.to_string(){
				return Some(key.clone())
			}
		}
		None
	}

	pub fn get_material(&self, material_ref: &MaterialRef) -> Option<&Material> {
		Some(&self.material_map.get(material_ref)?.material)
	}

	pub fn get_material_mut(&mut self, material_ref: &MaterialRef) -> Option<&mut Material> {
		Some(&mut self.material_map.get_mut(material_ref)?.material)
	}

	pub(crate) fn get_render_data(&self, material_ref: &MaterialRef) -> (&Uniform<B>, &ImageBuffer<B>, &RenderStage) {
		let entry = &self.material_map[material_ref];
		(self.ubo_library.get_uniform_buffer(&entry.ubo_ref), self.tbo_library.get_texture_buffer(&entry.texture_ref), &entry.material.render_stage)
	}

	pub(crate) fn get_descriptor_layouts(&self) -> Vec<&<B as Backend>::DescriptorSetLayout> {
		let texture_buffer = self.tbo_library.get_texture_buffer(&TextureBufferLibrary::<B>::get_default_ref());
		let uniform_buffer = self.ubo_library.get_uniform_buffer(&UniformBufferLibrary::<B>::get_default_uniform_ref());

		vec![texture_buffer.get_layout(), uniform_buffer.get_layout()]
	}
}
