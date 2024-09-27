use {super::{as_any::AsAny, property::Property, id::Id,}, std::any::Any};

#[derive(Clone)]
pub struct SpriteSheetIndex(pub usize);

impl AsAny for SpriteSheetIndex {
	fn as_any_ref(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl Id for SpriteSheetIndex {
	fn id() -> u8 {
		27
	}
}

impl Property for SpriteSheetIndex {}