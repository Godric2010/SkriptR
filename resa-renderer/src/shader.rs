use glsl_to_spirv::ShaderType;

pub struct ShaderRef{
	pub name: String,
	pub(crate) vertex: Vec<u32>,
	pub(crate) fragment: Vec<u32>,
}

impl ShaderRef {
	pub fn new(name: String, shaders_to_compile: &[(String, String)]) -> Option<Self>{

		let mut vertex: Option<Vec<u32>> = None;
		let mut fragment: Option<Vec<u32>> = None;

		for (shader_type, shader_to_compile) in shaders_to_compile {
			match shader_type.as_str() {
				"vert" => {
					vertex = ShaderRef::compile_shader(shader_to_compile, ShaderType::Vertex)
				},
				"frag" => {
					fragment = ShaderRef::compile_shader(shader_to_compile, ShaderType::Fragment);
				}
				_ => {}
			}
		}

		let instance = ShaderRef{
			name,
			vertex: vertex?,
			fragment: fragment?,
		};
		Some(instance)
	}
	
	fn compile_shader(shader_file: &str, kind: ShaderType) -> Option<Vec<u32>>{
		let file = match glsl_to_spirv::compile(&shader_file, kind) {
			Ok(spirv_file) => spirv_file,
			Err(_) => return None,
		};

		match gfx_auxil::read_spirv(file) {
			Ok(spirv) => Some(spirv),
			Err(_) => None,
		}
	}
}