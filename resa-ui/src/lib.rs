use crate::font_library::FontLibrary;

mod font;
pub mod font_library;
pub mod ui_element;

#[allow(dead_code)]
pub struct ResaUserInterface{
	font_library: FontLibrary,
}

impl ResaUserInterface {
	pub fn new(fonts: Vec<(String, Vec<u8>)>)-> Self{

		let mut font_library = FontLibrary::new();
		for (font_name, font_data) in fonts.iter(){
			font_library.add_new_font(&font_name, &font_data);
		}


		ResaUserInterface{
			font_library,
		}
	}
}

