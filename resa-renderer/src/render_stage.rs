use std::fmt;
use std::fmt::Formatter;

#[derive(Copy, Clone, Hash, Debug, Eq, PartialEq)]
pub enum RenderStage {
	None,
	Opaque,
	Transparent,
	UI,
}

impl fmt::Display for RenderStage {
	fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
		match self {
			RenderStage::None => write!(f, "No rendering pass"),
			RenderStage::Opaque => write!(f, "Opaque Pass"),
			RenderStage::Transparent => write!(f, "Transparent Pass"),
			RenderStage::UI => write!(f, "UI Pass"),
		}
	}
}

impl RenderStage {
	pub fn get_stage_form_index(index: usize) -> Self {
		match index {
			0 => RenderStage::None,
			1 => RenderStage::Opaque,
			2 => RenderStage::Transparent,
			3 => RenderStage::UI,
			_ => panic!("Invalid index!"),
		}
	}
	pub fn get_stages_in_order_of_priority() -> Vec<Self> {
		vec![RenderStage::Opaque, RenderStage::Transparent, RenderStage::UI]
	}
}


