use {super::{as_any::AsAny, cast::Cast, property::Property, id::Id}, std::any::Any};

pub struct Active;

impl AsAny for Active {
	fn as_any_ref(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl Id for Active {
	fn id() -> u8 {
		10
	}
}

impl Cast for Active {}

impl Property for Active {}