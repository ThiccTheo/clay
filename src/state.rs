use {
    super::action::Action,
    ggez::{event::EventHandler, Context},
};

pub trait State: EventHandler<Action> {
    fn enter(&mut self, _ctx: &mut Context) {}

    fn exit(&mut self, _ctx: &mut Context) {}
}
