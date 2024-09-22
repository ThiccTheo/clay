use ggez::{glam::Vec2, graphics::Rect};

pub struct SpriteSheet {
    img_size: Vec2,
    rows: usize,
    cols: usize,
    sub_img_size: Vec2,
}

impl SpriteSheet {
    pub fn new(img_size: Vec2, rows: usize, cols: usize) -> Self {
        Self {
            img_size,
            rows,
            cols,
            sub_img_size: Vec2::new(img_size.x / cols as f32, img_size.y / rows as f32),
        }
    }

    pub fn uv_rect(&self, idx: usize) -> Rect {
        let (x, y) = (idx % self.cols, idx / self.cols);
        Rect::new(
            x as f32 / self.img_size.x,
            y as f32 / self.img_size.y,
            self.sub_img_size.x / self.img_size.x,
            self.sub_img_size.y / self.img_size.y,
        )
    }
}
