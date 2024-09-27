use {
    super::{batch::Batch, object::Object},
    ggez::Context,
    hashbrown::HashMap,
};

/// Provides extra functionality when pushing/popping states via the stack.
pub trait State {
    /// Runs once, after a state is pushed onto the stack, before any updates or draws.
    ///
    /// * `ctx` - App context.
    fn enter(&mut self, _ctx: &mut Context) {}

    /// Runs once, after a state is popped off the stack, before any updates or draws.
    ///
    /// * `ctx` - App context.
    fn exit(&mut self, _ctx: &mut Context) {}

    fn objects(&mut self) -> &mut Vec<Object>;

    fn batches(&mut self) -> &mut HashMap<u8, Batch>;

    fn package(&mut self) -> (&mut Vec<Object>, &mut HashMap<u8, Batch>);
}
