use ggez::{
    glam::Vec2,
    graphics::{DrawParam, InstanceArray, Rect},
};

pub struct Batch {
    instance_arr: InstanceArray,
    sub_img_size: Vec2,
    rows: usize,
    cols: usize,
}

impl Batch {
    pub fn with_sprite_sheet(mut self, img_size: Vec2, rows: usize, cols: usize) -> Self {
        self.rows = rows;
        self.cols = cols;
        self.sub_img_size = img_size / Vec2::new(self.cols as f32, self.rows as f32);
        self
    }

    pub fn uv_rect(&self, idx: usize) -> Rect {
        if self.rows <= 1 && self.cols <= 1 {
            Rect::one()
        } else {
            let (x, y) = (idx % self.cols, idx / self.cols);
            self.instance_arr.image().uv_rect(
                x as u32 * self.sub_img_size.x as u32,
                y as u32 * self.sub_img_size.y as u32,
                self.sub_img_size.x as u32,
                self.sub_img_size.y as u32,
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

    pub fn sub_img_size(&self) -> Vec2 {
        self.sub_img_size
    }
}

impl From<InstanceArray> for Batch {
    fn from(instance_arr: InstanceArray) -> Self {
        Self {
            instance_arr,
            sub_img_size: Vec2::ONE,
            rows: 1,
            cols: 1,
        }
    }
}
