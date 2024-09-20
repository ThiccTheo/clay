use {
    super::Action,
    ggez::{event::EventHandler, glam::Vec2, graphics::InstanceArray, mint::Point2, Context},
};

pub struct Playing;

impl EventHandler<Action> for Playing {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
		let dt = ctx.time.remaining_update_time()
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
		Ok(())
    }
}
