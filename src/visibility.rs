use {super::{as_any::AsAny, property::Property, id::Id, }, std::any::Any};

pub enum Visibility {
	Visible,
	Invisible,
}

impl AsAny for Visibility {
	fn as_any_ref(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl Id for Visibility {
	fn id() -> u8 {
		20
	}
}

impl Property for Visibility {}