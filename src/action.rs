use {
    super::state::State,
    std::fmt::{Debug, Formatter, Result as FormatResult},
};

#[allow(dead_code)]
pub enum Action {
    Create(Box<dyn State>),
    Destroy,
    Change(Box<dyn State>),
}

impl Debug for Action {
    fn fmt(&self, fmtter: &mut Formatter<'_>) -> FormatResult {
        write!(fmtter, "Hi, Mom!")
    }
}
