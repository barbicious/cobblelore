#[derive(Copy, Clone, serde::Serialize, serde::Deserialize)]
pub struct Color {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

#[macro_export]
macro_rules! unpack_colors {
    ($dst_colors:expr, $src_colors:expr) => {
        for i in 0..crate::renderer::pixel_buffer::PixelBuffer::CHANNELS {
            if let Some(color) = &($src_colors)[i as usize] {
                ($dst_colors)[i as usize] = Some(crate::renderer::palette::Palette::palettize(
                    color.r as i32,
                    color.g as i32,
                    color.b as i32,
                ))
            }
        }
    };
}