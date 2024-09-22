use {
    super::{id::Id, object::Object, state::State},
    crate::{batch::Batch, player::Player},
    ggez::{
        glam::Vec2,
        graphics::{Image, InstanceArray},
        Context,
    },
    std::collections::HashMap,
};

#[derive(Default)]
pub struct Playing {
    objects: Vec<Box<dyn Object>>,
    batches: HashMap<Id, Batch>,
}

impl State for Playing {
    fn enter(&mut self, ctx: &mut Context) {
        self.batches.insert(
            Player::ID,
            Batch::from(InstanceArray::new(
                ctx,
                Image::from_path(ctx, "\\player.png").unwrap(),
            ))
            .with_sprite_sheet(Vec2::splat(128.), 2, 2),
        );
        self.objects.push(Box::new(Player::new()));
    }

    fn objects(&mut self) -> &mut Vec<Box<dyn Object>> {
        &mut self.objects
    }

    fn batches(&mut self) -> &mut HashMap<Id, Batch> {
        &mut self.batches
    }

    fn package(&mut self) -> (&mut Vec<Box<dyn Object>>, &mut HashMap<Id, Batch>) {
        (&mut self.objects, &mut self.batches)
    }
}
