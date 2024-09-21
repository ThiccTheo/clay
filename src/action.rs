use {
    super::state::State,
    std::fmt::{Debug, Formatter, Result as FormatResult},
};

pub enum Action {
    Create(Box<dyn State>),
    Destroy,
    Change(Box<dyn State>),
}

impl Debug for Action {
    fn fmt(&self, _fmtter: &mut Formatter<'_>) -> FormatResult {
        todo!()
    }
}
