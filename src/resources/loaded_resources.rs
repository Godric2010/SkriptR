use resa_renderer::material::TextureFormat;

#[derive(Clone)]
pub struct LoadedImage{
	pub image_name: String,
	pub image_data: Vec<u8>,
	pub image_format: TextureFormat,
}

#[derive(Clone)]
pub struct LoadedFont{
	pub font_name: String,
	pub font_data: Vec<u8>,
}