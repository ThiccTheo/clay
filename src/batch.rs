use ggez::{
    glam::Vec2,
    graphics::{DrawParam, InstanceArray, Rect},
};

pub struct Batch {
    instance_arr: InstanceArray,
    img_size: Vec2,
    rows: usize,
    cols: usize,
}

impl Batch {
    pub fn with_sprite_sheet(mut self, img_size: Vec2, rows: usize, cols: usize) -> Self {
        self.img_size = img_size;
        self.rows = rows;
        self.cols = cols;
        self
    }

    pub fn uv_rect(&self, idx: usize) -> Rect {
        if self.rows <= 1 && self.cols <= 1 {
            Rect::one()
        } else {
            let (x, y) = (idx % self.cols, idx / self.cols);
            let sub_img_size = self.img_size / Vec2::new(self.cols as f32, self.rows as f32);
            self.instance_arr.image().uv_rect(
                x as u32 * sub_img_size.x as u32,
                y as u32 * sub_img_size.y as u32,
                sub_img_size.x as u32,
                sub_img_size.y as u32,
            )
        }
    }

    pub fn instance_arr(&self) -> &InstanceArray {
        &self.instance_arr
    }

    pub fn push(&mut self, draw_param: DrawParam) {
        self.instance_arr.push(draw_param);
    }

    pub fn clear(&mut self) {
        self.instance_arr.clear();
    }
}

impl From<InstanceArray> for Batch {
    fn from(instance_arr: InstanceArray) -> Self {
        Self {
            instance_arr,
            img_size: Vec2::ONE,
            rows: 1,
            cols: 1,
        }
    }
}
