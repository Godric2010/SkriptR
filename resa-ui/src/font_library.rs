use crate::font::Font;

pub struct FontLibrary{
	fonts: Vec<Font>
}

impl FontLibrary {
	pub fn new() -> Self {
		Self{
			fonts: vec![],
		}
	}

	pub fn add_new_font(&mut self, name: &str, font_bytes: &[u8]){
		let font = Font::new(name, 200.0, font_bytes);
		self.fonts.push(font);
	}

	pub fn get_font_atlas_by_name(&self, name: &str) -> Option<(Vec<u8>,(u32,u32))>{
		let font_atlas = self.fonts.iter().find(|font| font.name == name)?.atlas.clone();
		Some((font_atlas.pixels, (font_atlas.width, font_atlas.height)))
	}
}