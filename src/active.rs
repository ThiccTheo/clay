use {
    super::{as_any::AsAny, id::Id, property::Property},
    std::any::Any,
};

pub struct Active;

impl AsAny for Active {
    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl Id for Active {}

impl Property for Active {}
