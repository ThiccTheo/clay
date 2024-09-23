use ggez::glam::{Vec2, Vec3};

#[derive(Clone)]
pub struct Transform {
    pub translation: Vec3,
    pub rotation: f32,
    pub scale: Vec2,
}

#[allow(dead_code)]
impl Transform {
    fn with_translation(mut self, translation: Vec3) -> Self {
        self.translation = translation;
        self
    }

    fn with_rotation(mut self, rotation: f32) -> Self {
        self.rotation = rotation;
        self
    }

    fn with_scale(mut self, scale: Vec2) -> Self {
        self.scale = scale;
        self
    }
}

impl Default for Transform {
    fn default() -> Self {
        Self {
            translation: Vec3::ZERO,
            rotation: 0.,
            scale: Vec2::ONE,
        }
    }
}
