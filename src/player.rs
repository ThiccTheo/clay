use std::f32::consts::{FRAC_PI_2, TAU};

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
    }

    fn is_active(&self) -> bool {
        self.is_active
    }

    fn transform(&self) -> Option<Transform> {
        Some(self.xform.clone())
    }

    fn sprite_sheet_index(&self) -> Option<usize> {
        Some(2)
    }

    fn as_any_ref(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}
