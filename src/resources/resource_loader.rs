use std::{env, fs};
use std::collections::HashMap;
use std::fs::DirEntry;
use resa_renderer::material::{TextureFormat};
use resa_renderer::shader::ShaderRef;
use crate::resources::loaded_resources::{LoadedFont, LoadedImage, LoadedMaterial};

pub struct ResourceLoader {
	resources_path: String,
}

impl ResourceLoader {
	pub(crate) fn new() -> Option<Self> {
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
		let shader_dir = self.resources_path.clone() + "/shaders";
		let shader_paths = fs::read_dir(shader_dir).unwrap();

		let mut shader_collection = HashMap::<String, Vec<(String, String)>>::new();
		for shader_file in shader_paths {
			if let Ok(file) = shader_file {
				let (name, file_type) = self.get_filename_and_type(&file)?;
				let shader = match fs::read_to_string(file.path().as_path()) {
					Ok(content) => content,
					Err(e) => {
						println!("Could not read shader file! Error {}", e);
						return None;
					}
				};

				if shader_collection.contains_key(&name) {
					shader_collection.get_mut(&name).unwrap().push((file_type, shader));
				} else {
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

	pub fn load_images(&self) -> Vec<LoadedImage> {
		let image_dir = self.resources_path.clone() + "/images";
		let image_paths = fs::read_dir(image_dir).unwrap();

		let mut images: Vec<LoadedImage> = vec![];
		for image_path in image_paths {
			if let Ok(file) = image_path {
				let (name, _) = match self.get_filename_and_type(&file) {
					Some(result) => result,
					None => continue,
				};
				let image = match self.read_file_to_bytes(&file) {
					Some(image) => image,
					None => continue,
				};

				images.push(
					LoadedImage {
						image_name: name,
						image_data: image,
						image_format: TextureFormat::Png,
					}
				);
			}
		}
		images
	}

	pub fn load_fonts(&self) -> Vec<LoadedFont> {
		let font_dir = self.resources_path.clone() + "/fonts";
		let font_paths = fs::read_dir(font_dir).unwrap();

		let mut fonts: Vec<LoadedFont> = vec![];
		for font_path in font_paths {
			if let Ok(file) = font_path {
				let (name, file_type) = match self.get_filename_and_type(&file) {
					Some(result) => result,
					None => continue,
				};

				if file_type != "ttf".to_string() {
					continue;
				}

				let font = match self.read_file_to_bytes(&file) {
					Some(font) => font,
					None => continue,
				};


				fonts.push(
					LoadedFont {
						font_name: name,
						font_data: font,
					}
				);
			}
		}

		fonts
	}

	pub fn load_materials(&self) -> Vec<LoadedMaterial> {
		let material_dir = self.resources_path.clone() + "/materials";
		let material_path = fs::read_dir(material_dir).unwrap();

		let mut materials = vec![];
		for material_file in material_path {
			if let Ok(file) = material_file {
				let (name, _file_type) = match self.get_filename_and_type(&file) {
					Some(result) => result,
					None => continue,
				};
				let material_str = match fs::read_to_string(file.path().as_path()) {
					Ok(str) => str,
					Err(_e) => {
						println!("Could not read material {}", name);
						continue;
					}
				};

				let material = match self.parse_material(&name, &material_str) {
					Some(mat) => mat,
					None => continue
				};

				materials.push(material);
			}
		}
		materials
	}

	fn get_filename_and_type(&self, file: &DirEntry) -> Option<(String, String)> {
		let filename = file.file_name().to_str()?.to_string();
		let filename_parts: Vec<&str> = filename.split('.').collect();

		if filename_parts.len() != 2 {
			println!("Could not use {}", filename);
			return None;
		}

		let name = filename_parts[0].to_string();
		let file_type = filename_parts[1].to_string();
		Some((name, file_type))
	}

	fn read_file_to_bytes(&self, file: &DirEntry) -> Option<Vec<u8>> {
		Some(match fs::read(file.path().as_path()) {
			Ok(content) => content,
			Err(e) => {
				println!("Could not read shader file! Error {}", e);
				return None;
			}
		})
	}

	fn parse_material(&self, name: &str, material_str: &str) -> Option<LoadedMaterial> {
		let rows = material_str.split("\n");
		let mut key_value_pairs: Vec<(String, String)> = Vec::new();
		for row in rows {
			if row.len() == 0 {
				continue;
			}
			let pair: Vec<&str> = row.split(":").collect();
			if pair.len() != 2 {
				continue;
			}
			key_value_pairs.push((pair[0].to_string(), pair[1].to_string()));
		}

		let mut mat = LoadedMaterial {
			name: name.to_string(),
			shader: 0,
			stage: 0,
			color: [0, 0, 0, 0],
			texture: "".to_string(),
		};

		for (key, value) in key_value_pairs {
			match key.as_str() {
				"shader" => { mat.shader = value.parse::<usize>().unwrap() }
				"stage" => { mat.stage = value.parse::<usize>().unwrap() }
				"color" => {
					let color_values: Vec<u8> = value.split(",").map(|split| split.parse::<u8>().unwrap()).collect();
					if color_values.len() != 4 {
						println!("Color value of material {} is not four byte long", name);
						return None;
					}
					mat.color = [color_values[0], color_values[1], color_values[2], color_values[3]];
				}
				"texture" => { mat.texture = value }
				_ => {}
			}
		}

		Some(mat)
	}
}
