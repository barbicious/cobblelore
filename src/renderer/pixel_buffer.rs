pub struct PixelBuffer {
    pixels: Vec<u8>,
}

impl PixelBuffer {
    pub const WIDTH: u32 = 256;
    pub const HEIGHT: u32 = 144;

    /// Channels in pixel buffer, for RGBA it would be 4, one for R, one for G, etc.
    pub const CHANNELS: u32 = 4;

    pub fn new() -> Self {
        Self { pixels: vec![0; Self::WIDTH as usize * Self::HEIGHT as usize * Self::CHANNELS as usize] }
    }

    /// Sets a pixel at the given coordinates based off of a
    /// 4 byte hexadecimal representation, such as 0xFFFF00FF
    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32) {
        let idx = (y * Self::WIDTH as usize + x) * Self::CHANNELS as usize;

        debug_assert!(x < Self::WIDTH as usize && y < Self::HEIGHT as usize);

        self.pixels[idx..idx + Self::CHANNELS as usize].copy_from_slice(&color.to_le_bytes());
    }
    
    pub fn clear(&mut self) {
        self.pixels.fill(0);
    }

    /// Returns byte representation of pixels for uploading to a given image for display
    pub fn bytes(&self) -> &[u8] {
        &self.pixels
    }
}