use {super::{as_any::AsAny, cast::Cast, property::Property, id::Id}, std::any::Any};

pub enum Activity {
	Active,
	Inactive,
}

impl AsAny for Activity {
	fn as_any_ref(&self) -> &dyn Any {
		self
	}

	fn as_any_mut(&mut self) -> &mut dyn Any {
		self
	}
}

impl Id for Activity {
	fn id() -> u8 {
		10
	}
}

impl Cast for Activity {}

impl Property for Activity {}