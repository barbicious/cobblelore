use std::ops::Index;

macro_rules! apply_palette {
    ($luminance:expr, $($color:expr),+ $(,)?) => {{
        $(
            $color += $luminance;
            $color /= 2.0;
            $color *= (230.0 / u8::MAX as f32);
            $color += 10.0;
        )+
    }};
}

const MAX_COLORS: usize = 4;

pub type Colors = [Option<usize>; MAX_COLORS];

pub struct Palette {
    shades: [u32; Self::CHANNELS.pow(3)]
}

impl Palette {
    const CHANNELS: usize = 6;

    pub fn new() -> Self {
        let mut shades = [0; Self::CHANNELS.pow(3)];

        let mut i = 0;
        for r in 0..Self::CHANNELS {
            for g in 0..Self::CHANNELS {
                for b in 0..Self::CHANNELS {
                    let mut rr = Self::color_space(r as i32);
                    let mut gg = Self::color_space(g as i32);
                    let mut bb = Self::color_space(b as i32);

                    let luminance = (rr * 30.0 + gg * 59.0 + bb * 11.0) / 100.0;

                    apply_palette!(luminance, rr, gg, bb);

                    shades[i] =
                        (0xFF << 24 | ((rr as u32) << 16) | ((gg as u32) << 8) | (bb as u32))
                            as u32;

                    i += 1
                }
            }
        }

        Self { shades }
    }

    const fn color_space(color: i32) -> f32 {
        ((color * u8::MAX as i32) / (Self::CHANNELS as i32 - 1)) as f32
    }

    pub const fn palettize(r: i32, g: i32, b: i32) -> usize {
        Self::color_to_palette_idx(
            (r as f32 * u8::MAX as f32 / (Palette::CHANNELS - 1) as f32) as i32,
        ) * Palette::CHANNELS.pow(2)
            + Self::color_to_palette_idx(
            (g as f32 * u8::MAX as f32 / (Palette::CHANNELS - 1) as f32) as i32,
        ) * Palette::CHANNELS
            + Self::color_to_palette_idx(
            (b as f32 * u8::MAX as f32 / (Palette::CHANNELS - 1) as f32) as i32,
        )
    }

    const fn color_to_palette_idx(c: i32) -> usize {
        if c < 0 {
            return 0;
        }

        (((c as f32 * 100.0) % 10.0) + ((c as f32 * 10.0) % 10.0) + (c as f32 % 10.0)) as usize
    }
}

impl Index<usize> for Palette {
    type Output = u32;

    fn index(&self, index: usize) -> &Self::Output {
        &self.shades[index]
    }
}