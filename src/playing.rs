use {
    super::{object::Object, Action},
    ggez::{event::EventHandler, graphics::InstanceArray, Context}, std::collections::HashMap,
};



#[derive(Default)]
pub struct Playing {
    objects: Vec<Box<dyn Object>>,
	batches: HashMap<u8, InstanceArray>,
}

impl EventHandler<Action> for Playing {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        for i in 0..self.objects.len() {
            let (before, tmp) = self.objects.split_at_mut(i);
            let (this, after) = tmp.split_first_mut().unwrap();
            let others = before.iter_mut().chain(after.iter_mut());
            this.update(others, ctx);
        }
        self.objects.retain(|obj| obj.is_active());

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
		for obj in &self.objects {
			let batch = InstanceArray::new(todo!(), todo!());
			batch.resize(gfx, new_capacity);
		}
        Ok(())
    }
}
