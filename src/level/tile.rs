use crate::math::{Point, Rect};
use crate::renderer::palette::{Colors, MAX_COLORS};
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::renderer::color::Color;
use crate::renderer::texture_registry::TextureRegistry;
use crate::unpack_colors;

pub enum Neighbors {
    None,
    Four,
    Eight,
}

pub struct Tile {
    name: String,
    colors: Colors,
    texture_id: usize,
}

impl Tile {
    pub fn new(tile_payload: TilePayload, texture_registry: &TextureRegistry) -> Self {
        let mut colors = [None; MAX_COLORS];

        unpack_colors!(colors, tile_payload.colors);

        Self {
            name: tile_payload.name,
            colors,
            texture_id: texture_registry[tile_payload.texture_name.as_str()],
        }
    }

    pub fn blit(&self, renderer: &mut Renderer, texture_registry: &TextureRegistry, x: i32, y: i32) {
        renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
            src: &Rect {
                x: 0,
                y: 0,
                w: 24,
                h: 24,
            },
            dst: &Point { x, y },
            colors: &self.colors,
            blit_flags: &BlitFlags::empty(),
        })
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TilePayload {
    pub name: String,
    pub colors: [Option<Color>; MAX_COLORS],
    pub texture_name: String,
}