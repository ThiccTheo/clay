use ggez::{
    glam::Vec2,
    mint::{Point2, Vector2},
};

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
            xform: Transform::default(),
        }
    }
}

impl Object for Player {
    fn id(&self) -> Id {
        Self::ID
    }

    fn tick(&mut self, others: World, ctx: &mut Context, action: &mut Option<Action>) {
        *action = None;
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn transform(&self) -> Option<Transform> {
        Some(self.xform)
    }

    fn sprite_sheet_index(&self) -> usize {
        3
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
