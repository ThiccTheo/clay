use super::{as_any::AsAny, cast::Cast};

pub trait Property: AsAny + 'static {}

impl Cast for dyn Property {}