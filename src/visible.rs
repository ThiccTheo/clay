use {
    super::{as_any::AsAny, id::Id, property::Property},
    std::any::Any,
};

pub struct Visible;

impl AsAny for Visible {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Id for Visible {}

impl Property for Visible {}
