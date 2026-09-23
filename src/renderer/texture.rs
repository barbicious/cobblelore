use std::path::Path;
use image::GenericImageView;

pub struct Texture {
    pixels: Vec<u8>,
    width: u32,
    height: u32,
}

impl Texture {
    pub const TRANSPARENT_PIXEL: u8 = u8::MAX;

    pub fn new<P: AsRef<Path>>(path: P) -> anyhow::Result<Self> {
        let image = image::open(path)?;
        let (width, height) = image.dimensions();

        let mut pixels = image.to_luma8().into_raw();

        for i in &mut pixels {
            if *i == 0 {
                *i = Self::TRANSPARENT_PIXEL
            } else {
                *i /= 64
            }
        }

        Ok(Self { pixels, width, height })
    }

    pub fn width(&self) -> u32 {
        self.width
    }

    pub fn height(&self) -> u32 {
        self.height
    }

    pub fn pixel_at(&self, x: usize, y: usize) -> u8 {
        debug_assert!(x < self.width as usize && y < self.height as usize);

        self.pixels[y * self.width as usize + x]
    }
}