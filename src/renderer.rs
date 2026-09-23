use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, Texture, WindowCanvas};
use crate::renderer::palette::Palette;
use crate::renderer::pixel_buffer::PixelBuffer;

pub mod pixel_buffer;
pub mod palette;

pub struct Renderer {
    canvas: WindowCanvas,
    screen: Texture,
    pixel_buffer: PixelBuffer,
    palette: Palette,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas) -> anyhow::Result<Self> {
        let mut screen = canvas.texture_creator()
            .create_texture_streaming(
                Some(PixelFormat::ARGB8888),
                PixelBuffer::WIDTH,
                PixelBuffer::HEIGHT,
            )?;

        screen.set_scale_mode(ScaleMode::Nearest);

        Ok(Self {
            canvas,
            screen,
            pixel_buffer: PixelBuffer::new(),
            palette: Palette::new(),
        })
    }

    pub fn blit_palette(&mut self) {
        for y in 0..PixelBuffer::HEIGHT {
            for x in 0..PixelBuffer::WIDTH {
                self.pixel_buffer.set_pixel(x as usize, y as usize, self.palette[((y + x) % 216) as usize]);
            }
        }
    }

    pub fn flush(&mut self) {
        self.canvas.clear();

        self.pixel_buffer.clear()
    }

    pub fn splat(&mut self) -> anyhow::Result<()> {
        self.screen.with_lock(None, |buffer: &mut [u8], _pitch: usize| {
            buffer.copy_from_slice(self.pixel_buffer.bytes());
        })?;

        self.canvas.copy(&self.screen, None, None)?;

        self.canvas.present();

        Ok(())
    }
}