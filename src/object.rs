use {
    ggez::{graphics::{Draw, DrawParam, InstanceArray, Rect, Transform}, Context},
    std::{iter::Chain, slice::IterMut},
};

pub trait Object {
    fn update(
        &mut self,
        others: Chain<IterMut<'_, Box<dyn Object>>, IterMut<'_, Box<dyn Object>>>,
        ctx: &mut Context,
    );

    fn draw(&self, batch: &mut InstanceArray) {
        batch.image().uv_rect(x, y, w, h)
        DrawParam::transform(self, transform)
    }

    fn is_active(&self) -> bool;

	fn id(&self) -> u8;
}