use {
    super::{batch::Batch, object::Object},
    ggez::Context,
    hashbrown::HashMap,
};

pub trait State {
    fn enter(&mut self, _ctx: &mut Context) {}

    fn exit(&mut self, _ctx: &mut Context) {}

    fn objects(&mut self) -> &mut Vec<Object>;

    fn batches(&mut self) -> &mut HashMap<u8, Batch>;

    fn package(&mut self) -> (&mut Vec<Object>, &mut HashMap<u8, Batch>);
}
