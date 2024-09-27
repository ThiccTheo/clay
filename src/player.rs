use crate::{activity::Activity, prelude::*, visibility::Visibility};

pub struct Player;

impl Id for Player {
    fn id() -> u8 {
        99
    }
}

fn spawn_player() -> Object {
    Object::new(
        Player,
        [
            (Activity::id(), Box::new(Activity::Active)),
            (Visibility::id(), Box::new(Visibility::Visible)),
        ],
        |this, others, ctx, action| {
            
        },
    )
}
