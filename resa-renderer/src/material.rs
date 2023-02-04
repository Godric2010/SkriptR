
#[derive(Hash)]
#[derive(Copy, Clone)]
pub struct Color{
	r: u8,
	g: u8,
	b: u8,
	a: u8,
}

#[derive(Hash)]
pub struct Material{
	pub color: Color,

}
