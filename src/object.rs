use {
    super::world::World,
    ggez::{graphics::InstanceArray, Context},
    std::any::Any,
};

pub trait Object {
    fn update(&mut self, others: World, ctx: &mut Context);

    fn draw(&self, batch: &mut InstanceArray);

    fn is_active(&self) -> bool;

    fn id(&self) -> u8;

    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&self) -> &mut dyn Any;
}

impl dyn Object {
    pub fn cast<T: 'static + Object>(&self) -> Option<&T> {
        self.as_any().downcast_ref()
    }

    pub fn cast_mut<T: 'static + Object>(&mut self) -> Option<&mut T> {
        self.as_any_mut().downcast_mut()
    }
}
