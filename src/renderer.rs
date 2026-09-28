use crate::math::{Point, Rect};
use crate::renderer::palette::{Colors, Palette};
use crate::renderer::pixel_buffer::PixelBuffer;
use crate::renderer::texture::Texture;
use bitflags::bitflags;
use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, WindowCanvas};
use crate::renderer::camera::Camera;

pub mod pixel_buffer;
pub mod palette;
pub mod texture;
pub mod camera;
pub mod texture_registry;
pub mod color;

bitflags! {
    pub struct BlitFlags: u8 {
        const FLIP_H = 1 << 0;
        const FLIP_V = 1 << 1;
    }
}

pub struct BlitDesc<'a> {
    pub src: &'a Rect<u32>,
    pub dst: &'a Point<i32>,
    pub colors: &'a Colors,
    pub blit_flags: &'a BlitFlags,
}

pub struct Renderer {
    canvas: WindowCanvas,
    screen: sdl3::render::Texture,
    pixel_buffer: PixelBuffer,
    palette: Palette,
    camera: Camera,
}

impl Renderer {
    pub fn new(canvas: WindowCanvas, camera: Camera) -> anyhow::Result<Self> {
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
            camera,
        })
    }

    pub fn blit_texture(&mut self, texture: &Texture, blit_desc: BlitDesc) {
        for y in 0..blit_desc.src.h {
            let y_dst = if !blit_desc.blit_flags.contains(BlitFlags::FLIP_V) { y as i32 + blit_desc.dst.y - self.camera.y_offset() } else { blit_desc.dst.y + blit_desc.src.h as i32 - y as i32 - 1 - self.camera.y_offset() };

            if y_dst < 0 || y_dst >= PixelBuffer::HEIGHT as i32 {
                continue
            }

            for x in 0..blit_desc.src.w {
                let x_dst = if !blit_desc.blit_flags.contains(BlitFlags::FLIP_H) { x as i32 + blit_desc.dst.x - self.camera.x_offset() } else { blit_desc.dst.x + blit_desc.src.w as i32 - x as i32 - 1 - self.camera.x_offset() };

                if x_dst < 0 || x_dst >= PixelBuffer::WIDTH as i32 {
                    continue
                }

                let pixel = texture.pixel_at((x + blit_desc.src.x) as usize, (y + blit_desc.src.y) as usize) as usize;

                if pixel == Texture::TRANSPARENT_PIXEL as usize {
                    continue
                }

                self.pixel_buffer.set_pixel(x_dst as usize, y_dst as usize, self.palette[blit_desc.colors[pixel].unwrap()]);
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
    
    pub fn camera_mut(&mut self) -> &mut Camera {
        &mut self.camera
    }
}