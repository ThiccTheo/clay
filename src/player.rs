use ggez::{glam::Vec2, mint::{Point2, Vector2}};

use crate::prelude::*;

pub struct Player {
    is_active: bool,
    xform: Transform,
}

impl Player {
    pub const ID: Id = Id(1);

    pub fn new() -> Self {
        Self {
            is_active: true,
            xform: Transform::Values {
                dest: Vec2::ZERO.into(),
                rotation: 0.,
                scale: Vec2::splat(1.).into(),
                offset: Vec2::ZERO.into(),
            },
        }
    }
}

impl Object for Player {
    fn id(&self) -> Id {
        Self::ID
    }

    fn tick(&mut self, others: World, ctx: &mut Context) {
        
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn transform(&self) -> Option<Transform> {
        Some(self.xform)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
