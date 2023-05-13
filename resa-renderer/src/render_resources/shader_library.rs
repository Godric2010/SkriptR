use crate::shader::ShaderRef;

pub struct ShaderLibrary {
	shader_refs: Vec<ShaderRef>,
}

impl ShaderLibrary {
	pub(crate) fn new() -> Self {
		Self {
			shader_refs: vec![],
		}
	}

	/// Add a new shader to the shader library. The shaders will be stored permanently until the
	/// programm shuts down. Therefore the ids of the shader ref will be prevail for program runtime.
	///
	/// # Arguments
	///
	/// * `shader`: The shader ref instance which shall be added to the library.
	///
	/// returns: Index of the shader ref in the library as USIZE.
	pub fn add(&mut self, shader: ShaderRef) -> usize {
		let shader_id = self.shader_refs.len();
		self.shader_refs.push(shader);
		shader_id
	}

	/// Get the reference to a shader ref by its id. If the id is invalid for some reason, no shader ref will be returned.
	///
	/// # Arguments
	///
	/// * `shader_id`: The id of the shader ref inside the library.
	///
	/// returns: Option<&ShaderRef>
	pub fn get_by_id(&self, shader_id: &u32) -> Option<&ShaderRef> {
		let shader_id = shader_id.clone() as usize;
		if shader_id > self.shader_refs.len() {
			return None;
		}

		Some(&self.shader_refs[shader_id.clone()])
	}


	/// Get the reference to a shader ref by its name. If no shader with the given name is found, the function will return None.
	///
	/// # Arguments
	///
	/// * `shader_name`: The name of the requested shader.
	///
	/// returns: Option<&ShaderRef>
	///
	/// # Examples
	pub fn get_by_name(&self, shader_name: &str) -> Option<&ShaderRef> {
		for shader_ref in self.shader_refs.iter() {
			if shader_ref.name == shader_name {
				return Some(shader_ref);
			}
		}
		return None;
	}
}