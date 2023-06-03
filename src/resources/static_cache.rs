use resa_renderer::material::Material;
use resa_renderer::shader::ShaderRef;
use crate::resources::loaded_resources::{LoadedFont, LoadedImage, LoadedMaterial};
use crate::resources::resource_loader::ResourceLoader;

pub struct StaticResourceCache {
	images: Vec<LoadedImage>,
	fonts: Vec<LoadedFont>,
	shaders: Vec<ShaderRef>,
	materials: Vec<LoadedMaterial>,
	//meshes: Vec<LoadedMesh>
}

impl StaticResourceCache {
	pub fn new() -> Self {
		StaticResourceCache {
			images: Vec::new(),
			fonts: Vec::new(),
			shaders: Vec::new(),
			materials: Vec::new(),
		}
	}

	pub fn load_from_disk(&mut self, loader: &ResourceLoader) {
		self.shaders = loader.load_all_shaders().unwrap();
		self.images = loader.load_images();
		self.fonts = loader.load_fonts();
		self.materials = loader.load_materials();
	}

	pub fn get_shaders(&self) -> Vec<ShaderRef>{
		self.shaders.clone()
	}

	pub fn get_fonts(&self) -> Vec<LoadedFont>{
		self.fonts.clone()
	}

	pub fn get_image(&self, image_name: &str) -> Option<LoadedImage>{
		Some(self.images.iter().find(|image| image.image_name == image_name.to_string())?.clone())

	}

	pub fn get_materials(&self) -> Vec<LoadedMaterial>{ self.materials.clone()	}
}