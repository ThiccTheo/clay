use {
    super::{batch::Batch, object::Object},
    ggez::Context,
    hashbrown::HashMap,
    std::any::TypeId,
};

pub trait State {
    fn enter(&mut self, _ctx: &mut Context) {}

    fn exit(&mut self, _ctx: &mut Context) {}

    fn objects(&mut self) -> &mut Vec<Object>;

    fn batches(&mut self) -> &mut HashMap<TypeId, Batch>;

    fn package(&mut self) -> (&mut Vec<Object>, &mut HashMap<TypeId, Batch>);
}
