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
use crate::uniform::Uniform;

struct MaterialEntry{
	material: Material,
	ubo_ref: UBORef,
	texture_ref: TBORef,
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

	pub fn add_materials(&mut self, materials: &[Material]) -> Vec<MaterialRef>{

		let materials_ubo_data = materials.iter().map(|mat| (mat.get_ubo_data())).collect();
		let ubo_refs = self.ubo_library.add_buffers(materials_ubo_data);

		let materials_tbo_data = materials.iter().map(|mat| mat.texture.clone()).collect();
		let tbo_refs = self.tbo_library.add_texture_buffer(materials_tbo_data);

		let mut material_refs = vec![];
		for (index,material) in materials.iter().enumerate(){
			let material_ref = MaterialRef(self.last_entry_id);
			self.last_entry_id += 1;

			let entry = MaterialEntry{
				material: material.clone(),
				ubo_ref: ubo_refs[index],
				texture_ref: tbo_refs[index],
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

	pub fn get_material_mut(&mut self, material_ref: &MaterialRef) -> Option<&mut Material>{
		Some(&mut self.material_map.get_mut(material_ref)?.material)
	}

	pub(crate) fn get_render_data(&self, material_ref: &MaterialRef) -> (&Uniform<B>, &ImageBuffer<B>){
		let entry = &self.material_map[material_ref];
		(self.ubo_library.get_uniform_buffer(&entry.ubo_ref), self.tbo_library.get_texture_buffer(&entry.texture_ref))
	}

	pub(crate) fn get_descriptor_layouts(&self) -> Vec<&<B as Backend>::DescriptorSetLayout>{
		let texture_buffer = self.tbo_library.get_texture_buffer(&TextureBufferLibrary::<B>::get_default_ref());
		let uniform_buffer = self.ubo_library.get_uniform_buffer(&UniformBufferLibrary::<B>::get_default_uniform_ref());

		vec![texture_buffer.get_layout(), uniform_buffer.get_layout()]
	}

	/*pub(crate) fn add_new_texture(&mut self, image_data: Vec<u8>, renderer: &mut Renderer<backend::Backend>) -> usize {
		let img = image::load(Cursor::new(&image_data[..]), image::ImageFormat::Png).unwrap().to_rgba8();
		let buffer_index = renderer.add_image_buffer(img);
		buffer_index
	}*/


}
