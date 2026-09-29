use crate::math::Rect;
use crate::renderer::pixel_buffer::PixelBuffer;

pub struct Camera {
    bounds: Rect<i32>,
    x_offset: i32,
    y_offset: i32,
}

impl Camera {
    pub fn new(bounds: Rect<i32>, x_offset: i32, y_offset: i32) -> Self {
        Self {
            bounds,
            x_offset,
            y_offset,
        }
    }

    pub fn x_offset(&self) -> i32 {
        self.x_offset
    }

    pub fn y_offset(&self) -> i32 {
        self.y_offset
    }

    pub fn set_x_offset(&mut self, x_offset: i32) {
        self.x_offset = (x_offset - PixelBuffer::WIDTH as i32 / 2)
            .clamp(0, self.bounds.w - PixelBuffer::WIDTH as i32);
    }

    pub fn set_y_offset(&mut self, y_offset: i32) {
        self.y_offset = (y_offset - PixelBuffer::HEIGHT as i32 / 2)
            .clamp(0, self.bounds.w - PixelBuffer::HEIGHT as i32);
    }

    pub fn clamp(&self, x: i32, y: i32) -> (i32, i32) {
        (x.clamp(0, self.bounds.w), y.clamp(0, self.bounds.h))
    }
}
