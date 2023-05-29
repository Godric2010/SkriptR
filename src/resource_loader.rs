use std::{env, fs};
use std::collections::HashMap;
use std::path::{Path};
use rusttype::Font;
use resa_renderer::material::TextureFormat;
use resa_renderer::shader::ShaderRef;

pub struct ResourceLoader {
	resources_path: String,
}

impl ResourceLoader {
	pub fn new() -> Option<Self> {
		let current_directory = env::current_dir().unwrap();
		let ressource_directory = current_directory.join("resources");

		if !fs::metadata(&ressource_directory).is_ok() {
			return None;
		}

		Some(Self {
			resources_path: ressource_directory.as_path().to_str().unwrap().to_string(),
		})
	}

	pub(crate) fn load_all_shaders(&self) -> Option<Vec<ShaderRef>> {
		let shader_dir= self.resources_path.clone() + "/shaders";
		let shader_paths = fs::read_dir(shader_dir).unwrap();

		let mut shader_collection = HashMap::<String, Vec<(String, String)>>::new();
		for shader_file in shader_paths{
			if let Ok(file) = shader_file {
				let filename = file.file_name().to_str()?.to_string();
				let filename_parts: Vec<&str> = filename.split('.').collect();

				if filename_parts.len() != 2 {
					println!("Could not use {}", filename);
					continue;
				}

				let name = filename_parts[0].to_string();
				let file_type = filename_parts[1].to_string();

				let shader = match fs::read_to_string(file.path().as_path()) {
					Ok(content) => content,
					Err(e) => {
						println!("Could not read shader file! Error {}",e);
						return None;},
				};

				if shader_collection.contains_key(&name){
					shader_collection.get_mut(&name).unwrap().push((file_type, shader));
				}else {
					shader_collection.insert(name.clone(), vec![(file_type, shader)]);
				}
			}
		}

		let mut shaders = Vec::<ShaderRef>::new();
		for (name, shaders_to_compile) in shader_collection {
			let shader_ref = ShaderRef::new(name.to_string(), &shaders_to_compile)?;
			shaders.push(shader_ref);
		}
		Some(shaders)
	}


	pub fn load_image(&self, image_file_name: &str) -> Option<(Vec<u8>, TextureFormat)>{
		let image_path = self.resources_path.clone() + "/"+ image_file_name;
		if !fs::metadata(&image_path).is_ok(){
			return None;
		}

		let image_bytes = match fs::read(Path::new(&image_path)){
			Ok(bytes) => bytes,
			Err(e) => {
				println!("Could not read file {}", image_file_name);
				return None;
			}
		};

		Some((image_bytes, TextureFormat::Png))
	}

	pub fn load_font(&self, font_name: &str) -> Option<Vec<u8>>{
		let font_path = self.resources_path.clone() + "/fonts/" + font_name + ".ttf";
		if !fs::metadata(&font_path).is_ok(){
			return None;
		}

		let font_bytes = match fs::read(Path::new(&font_path)){
			Ok(font_bytes) => font_bytes,
			Err(_) => {
				println!("Could not load font {}", font_name);
				return None;
			}
		};
		Some(font_bytes)
	}
}