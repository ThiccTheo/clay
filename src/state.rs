use {
    super::action::Action,
    ggez::{event::EventHandler, Context},
};

/// Provides extra functionality when pushing/popping states via the stack.
pub trait State: EventHandler<Action> {
    /// Runs once, after a state is pushed onto the stack, before any updates or draws.
    ///
    /// * `ctx` - App context.
    fn enter(&mut self, _ctx: &mut Context) {}

    /// Runs once, after a state is popped off the stack, before any updates or draws.
    ///
    /// * `ctx` - App context.
    fn exit(&mut self, _ctx: &mut Context) {}
}
