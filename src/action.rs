use {
    super::state::State,
    std::fmt::{Debug, Formatter, Result as FormatResult},
};

/// Actions to perform on the state machine at the end of the frame.
///
/// To be returned after user-defined conditions are met.
///
/// # Example
/// ```
/// impl EventHandler<Action> for ... {
///     ...
///
///     fn draw(&mut self, ctx: &mut Context) {
///         if self.player.is_active() {
///             Ok(())
///         } else {
///             Err(Action::Destroy)
///         }
///     }
/// }
/// ```
#[allow(dead_code)]
pub enum Action {
    /// Pushes a new state to the top of the stack.
    Create(Box<dyn State>),

    /// Pops the current state from the top of the stack.
    Destroy,

    /// Pops the current state from the top of the stack.
    ///
    /// Pushes a new state to the top of the stack.
    Change(Box<dyn State>),
}

impl Debug for Action {
    fn fmt(&self, fmtter: &mut Formatter<'_>) -> FormatResult {
        write!(fmtter, "Hi, Mom!")
    }
}
