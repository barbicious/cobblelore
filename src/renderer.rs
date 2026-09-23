use bitflags::bitflags;
use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, WindowCanvas};
use crate::math::{Point, Rect};
use crate::renderer::palette::{Colors, Palette};
use crate::renderer::pixel_buffer::PixelBuffer;
use crate::renderer::texture::Texture;

pub mod pixel_buffer;
pub mod palette;
pub mod texture;

bitflags! {
    pub struct BlitFlags: u8 {
        const FLIP_H = 1 << 0;
        const FLIP_V = 1 << 1;
    }
}

pub struct BlitDesc<'a> {
    pub src: &'a Rect<u32>,
    pub dst: &'a Point<u32>,
    pub colors: &'a Colors,
    pub blit_flags: &'a BlitFlags,
}

pub struct Renderer {
    canvas: WindowCanvas,
    screen: sdl3::render::Texture,
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

    pub fn blit_texture(&mut self, texture: &Texture, blit_desc: BlitDesc) {
        for y in 0..texture.height() {
            for x in 0..texture.width() {
                let pixel = texture.pixel_at(x as usize, y as usize) as usize;

                if pixel == Texture::TRANSPARENT_PIXEL as usize {
                    continue
                }

                self.pixel_buffer.set_pixel(x as usize, y as usize, self.palette[blit_desc.colors[pixel].unwrap()]);
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