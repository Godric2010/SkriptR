use gfx_hal::Backend;

pub mod graphics_pipeline;

#[derive(Hash)]
#[derive(Copy, Clone)]
#[derive(Eq, PartialEq)]
pub enum PipelineType{
	Opaque,
	Transparent,
	UI,
}

pub trait Pipeline<B: Backend>{
	fn build_pipeline();


}
