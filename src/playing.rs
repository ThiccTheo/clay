use {
    super::{action::Action, id::Id, object::Object, state::State},
    crate::player::Player,
    ggez::{
        event::EventHandler,
        graphics::{Canvas, Color, DrawParam, Image, InstanceArray},
        Context,
    },
    std::collections::HashMap,
};

#[derive(Default)]
pub struct Playing {
    objects: Vec<Box<dyn Object>>,
    batches: HashMap<Id, InstanceArray>,
}

impl State for Playing {
    fn enter(&mut self, ctx: &mut Context) {
        self.batches.insert(
            Player::ID,
            InstanceArray::new(ctx, Image::from_path(ctx, "\\player.png").unwrap()),
        );
        self.objects.push(Box::new(Player::new()));
    }
}

impl EventHandler<Action> for Playing {
    fn update(&mut self, ctx: &mut Context) -> Result<(), Action> {
        for i in 0..self.objects.len() {
            let (before, tmp) = self.objects.split_at_mut(i);
            let (this, after) = tmp.split_first_mut().unwrap();
            let others = before.iter_mut().chain(after.iter_mut());
            this.tick(others, ctx);
        }
        self.objects.retain(|obj| obj.is_active());
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Action> {
        let mut canvas = Canvas::from_frame(ctx, Color::WHITE);
        for obj in self.objects.iter().filter(|obj| obj.is_visible()) {
            let Some(batch) = self.batches.get_mut(&obj.id()) else {
                continue;
            };
            batch.push(DrawParam {
                src: obj.uv_rect().unwrap_or_default(),
                transform: obj.transform().unwrap_or_default(),
                z: obj.id().0.into(),
                ..Default::default()
            });
        }
        self.batches.values_mut().for_each(|batch| {
            canvas.draw(batch, DrawParam::default());
            batch.clear()
        });
        drop(canvas.finish(ctx));
        Ok(())
    }
}
