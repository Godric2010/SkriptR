use std::env::current_exe;
use std::ops::Range;
use rusttype::{point, Point, Scale};

#[derive(Clone)]
pub struct Atlas {
	pub pixels: Vec<u8>,
	pub width: u32,
	pub height: u32,
}

pub struct Font {
	pub name: String,
	pub size: f32,
	pub atlas: Atlas,
}

impl Font {
	pub fn new(name: &str, font_size: f32, font_bytes: &[u8]) -> Self {
		let characters = String::from("Hgllo"/*"AaBbCcDdEeFfGgHhIiJjKkLlMmNnOoPpQqRrSsTtUuVvWwXxYyZz0123456789.,:;!?'<>"*/);

		let font = rusttype::Font::try_from_bytes(&font_bytes).unwrap();

		let height = font_size as i32;
		// scale the font initially
		let scale = Scale::uniform(font_size);

		// Collect the glyph data
		let mut glyph_data: Vec<(Vec<u8>, i32)>	= Vec::new();
		for char in characters.chars(){
			let glyph = font.glyph(char).scaled(scale).positioned(point(0., 0.));
			let glyph_bb = glyph.pixel_bounding_box().unwrap();

			let mut pixel_data = vec![255; (glyph_bb.width() * height * 4) as usize];
			glyph.draw(|x, y, v| {
				let index = (y * glyph_bb.width() as u32 + x) as usize * 4;
				pixel_data[index] = 255;                    // red
				pixel_data[index + 1] = 255;                // green
				pixel_data[index + 2] = 255;                // blue
				pixel_data[index + 3] = (v * 255.0) as u8;  // alpha
			});

			glyph_data.push((pixel_data, glyph_bb.width()))
		}

		let total_width: i32 = glyph_data.iter().map(|(_, width)| width).sum();
		let atlas_size: i32  = total_width * height * 4;

		let mut atlas:Vec<u8> = vec![0; atlas_size as usize];
		let mut current_width_offset = 0;
		for (pixel_data, width) in glyph_data.iter(){
			for y in 0..height{
				let src_offset = (y * width * 4) as usize;
				let dest_offset = (y * total_width * 4 + current_width_offset * 4) as usize;
				atlas[dest_offset..(dest_offset + *width as usize * 4)].copy_from_slice(&pixel_data[src_offset..(src_offset + *width as usize *4)]);
			}
			current_width_offset += width;

		}


		// genereate atlas
		// let mut atlas = Vec::with_capacity(characters.len());
		// let mut atlas_width = 0;
		// let mut atlas_height = 0;
		//
		// for char in characters.chars() {
		// 	let glyph = font.glyph(char).scaled(scale).positioned(point(0., 0.));
		// 	let glyph_bb = glyph.pixel_bounding_box().unwrap();
		//
		// 	atlas_width += glyph_bb.width() as u32;
		// 	atlas_height = glyph_bb.height() as u32;
		// 	let mut pixel_data = vec![0; (glyph_bb.width() * glyph_bb.height() * 4) as usize];
		// 	glyph.draw(|x, y, v| {
		// 		let index = (y * glyph_bb.width() as u32 + x) as usize * 4;
		// 		pixel_data[index] = 255;                    // red
		// 		pixel_data[index + 1] = 255;                // green
		// 		pixel_data[index + 2] = 255;                // blue
		// 		pixel_data[index + 3] = (v * 255.0) as u8;  // alpha
		// 	});
		// 	atlas.push(pixel_data);
		// }


		Self {
			name: name.to_string(),
			size: font_size,
			atlas: Atlas {
				pixels: atlas,
				width: current_width_offset as u32,
				height: height as u32,
			},
		}
	}
}

