use resa_renderer::shader::ShaderRef;
use crate::resources::loaded_resources::{LoadedFont, LoadedImage};
use crate::resources::resource_loader::ResourceLoader;

pub struct StaticResourceCache {
	images: Vec<LoadedImage>,
	fonts: Vec<LoadedFont>,
	shaders: Vec<ShaderRef>,
	//meshes: Vec<LoadedMesh>
}

impl StaticResourceCache {
	pub fn new() -> Self {
		StaticResourceCache {
			images: Vec::new(),
			fonts: Vec::new(),
			shaders: Vec::new(),
		}
	}

	pub fn load_from_disk(&mut self, loader: &ResourceLoader) {
		self.shaders = loader.load_all_shaders().unwrap();
		self.images = loader.load_images();
		self.fonts = loader.load_fonts();
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


}