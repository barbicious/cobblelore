use crate::math::Rect;

pub struct Camera {
    bounds: Rect<i32>,
    x_offset: i32,
    y_offset: i32,
}

impl Camera {
    pub fn new(bounds: Rect<i32>, x_offset: i32, y_offset: i32) -> Self {
        Self { bounds, x_offset, y_offset }
    }

    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }
}