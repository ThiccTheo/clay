use {
    super::{action::Action, object::Object, state::State},
    ggez::{
        event::EventHandler,
        graphics::{Canvas, DrawParam, InstanceArray},
        Context,
    },
    maplit::hashmap,
    std::collections::HashMap,
};

#[derive(Default)]
pub struct Playing {
    objects: Vec<Box<dyn Object>>,
    batches: HashMap<u8, InstanceArray>,
}

impl State for Playing {
    fn enter(&mut self, ctx: &mut Context) {
        self.batches = hashmap! {
            10 => InstanceArray::new(ctx, None),
        };
    }
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
        let mut canvas = Canvas::from_frame(ctx, None);
        for obj in &self.objects {
            obj.draw(self.batches.get_mut(&obj.id()));
        }
        self.batches.values_mut().for_each(|batch| {
            canvas.draw(batch, DrawParam::default());
            batch.clear()
        });
        canvas.finish(ctx);
        Ok(())
    }
}
