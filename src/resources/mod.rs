use resa_renderer::shader::ShaderRef;
use crate::resources::loaded_resources::{LoadedImage, LoadedMaterial};
use crate::resources::resource_loader::ResourceLoader;
use crate::resources::static_cache::StaticResourceCache;

pub mod loaded_resources;
pub mod resource_loader;
mod static_cache;


pub struct ResourceManager{
    #[allow(dead_code)]
	static_loader: ResourceLoader,
	// stream_loader: ResourceStreamer,
	static_cache: StaticResourceCache,

}

impl ResourceManager {
	pub fn new() -> Option<Self>{

		let resource_loader = ResourceLoader::new()?;
		let mut static_cache = StaticResourceCache::new();
		static_cache.load_from_disk(&resource_loader);

		Some(Self{
			static_loader: resource_loader,
			static_cache,
		})
	}

	pub fn get_shaders(&self) -> Vec<ShaderRef>{
		self.static_cache.get_shaders()
	}

	pub fn get_fonts(&self) -> Vec<(String, Vec<u8>)>{
		self.static_cache.get_fonts().iter().map(|font| (font.font_name.clone(), font.font_data.clone())).collect()
	}

	pub fn get_materials(&self) -> Vec<LoadedMaterial>{ self.static_cache.get_materials() }

	pub fn get_image(&self, name: &str, streaming: bool)-> Option<LoadedImage>{

		if streaming{
			println!("Streaming images is not implemented yet!");
			return None;
		}

		self.static_cache.get_image(name)
	}

}
