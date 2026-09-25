use std::sync::LazyLock;
use bitflags::bitflags;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};
use crate::level::Level;
use crate::level::tile_registry::TileRegistry;
use crate::math::{Point, Rect};
use crate::renderer::palette::{Colors, MAX_COLORS};
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::renderer::color::Color;
use crate::renderer::texture_registry::TextureRegistry;
use crate::unpack_colors;

bitflags! {
    pub struct NeighborMask: u8 {
        const UP = 1 << 0;
        const RIGHT = 1 << 1;
        const DOWN = 1 << 2;
        const LEFT = 1 << 3;
        const UP_LEFT = 1 << 4;
        const UP_RIGHT = 1 << 5;
        const DOWN_LEFT = 1 << 6;
        const DOWN_RIGHT = 1 << 7;
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub enum NeighborCheck {
    Zero,
    Four,
    Eight,
}

pub struct Tile {
    id: usize,
    name: String,
    colors: Colors,
    texture_id: usize,
    neighbor_check: NeighborCheck,
    liquid: bool,
}

impl Tile {
    pub const WIDTH: i32 = 16;
    pub const HEIGHT: i32 = 16;

    pub const SUB_WIDTH: i32 = Self::WIDTH / 2;
    pub const SUB_HEIGHT: i32 = Self::HEIGHT / 2;

    pub fn new(tile_payload: TilePayload, texture_registry: &TextureRegistry, id: usize) -> Self {
        let mut colors = [None; MAX_COLORS];

        unpack_colors!(colors, tile_payload.colors);

        Self {
            id,
            name: tile_payload.name,
            colors,
            texture_id: texture_registry[tile_payload.texture_name.as_str()],
            neighbor_check: tile_payload.neighbor_check,
            liquid: tile_payload.liquid,
        }
    }

    pub fn blit(&self, renderer: &mut Renderer, texture_registry: &TextureRegistry, x: i32, y: i32, neighbor_mask: NeighborMask, ticks: u32) {
        match self.neighbor_check {
            NeighborCheck::Zero => {
                renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
                    src: &Rect {
                        x: 0,
                        y: 0,
                        w: Self::SUB_WIDTH as u32,
                        h: Self::SUB_HEIGHT as u32,
                    },
                    dst: &Point { x, y },
                    colors: &self.colors,
                    blit_flags: &BlitFlags::empty(),
                })
            }
            NeighborCheck::Four => {
                if neighbor_mask.contains(NeighborMask::LEFT) && neighbor_mask.contains(NeighborMask::UP) {
                    self.blit_center_random(renderer, texture_registry, x, y, ticks)
                } else {
                    renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
                        src: &Rect {
                            x: neighbor_mask.contains(NeighborMask::LEFT) as u32 * Self::SUB_WIDTH as u32,
                            y: neighbor_mask.contains(NeighborMask::UP) as u32 * Self::SUB_HEIGHT as u32,
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point { x, y },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    });
                }

                if neighbor_mask.contains(NeighborMask::RIGHT) && neighbor_mask.contains(NeighborMask::UP) {
                    self.blit_center_random(renderer, texture_registry, x + Self::SUB_WIDTH, y, ticks)
                } else {
                    renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
                        src: &Rect {
                            x: 16 - neighbor_mask.contains(NeighborMask::RIGHT) as u32 * Self::SUB_WIDTH as u32,
                            y: neighbor_mask.contains(NeighborMask::UP) as u32 * Self::SUB_HEIGHT as u32,
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point { x: x + Self::SUB_WIDTH , y },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    });
                }

                if neighbor_mask.contains(NeighborMask::LEFT) && neighbor_mask.contains(NeighborMask::DOWN) {
                    self.blit_center_random(renderer, texture_registry, x, y + Self::SUB_HEIGHT, ticks)
                } else {
                    renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
                        src: &Rect {
                            x: neighbor_mask.contains(NeighborMask::LEFT) as u32 * Self::SUB_WIDTH as u32,
                            y: 16 - neighbor_mask.contains(NeighborMask::DOWN) as u32 * Self::SUB_HEIGHT as u32,
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point { x, y: y + Self::SUB_HEIGHT },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    });
                }

                if neighbor_mask.contains(NeighborMask::RIGHT) && neighbor_mask.contains(NeighborMask::DOWN) {
                    self.blit_center_random(renderer, texture_registry, x + Self::SUB_WIDTH, y + Self::SUB_HEIGHT, ticks)
                } else {
                    renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
                        src: &Rect {
                            x: 16 - (neighbor_mask.contains(NeighborMask::RIGHT) as u32 * Self::SUB_WIDTH as u32),
                            y: 16 - (neighbor_mask.contains(NeighborMask::DOWN) as u32 * Self::SUB_HEIGHT as u32),
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point { x: x + Self::SUB_WIDTH , y: y + Self::SUB_HEIGHT },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    });
                }

            }
            NeighborCheck::Eight => {

            }
        }
    }

    pub fn connects(&self, level: &Level, x: i32, y: i32, tile_registry: &TileRegistry) -> bool {
        if x < 0 || x >= Level::WIDTH as i32 || y < 0 || y >= Level::HEIGHT as i32 {
            return false;
        }

        level.tile_at(x as usize, y as usize) == self.id
    }

    fn blit_center_random(&self, renderer: &mut Renderer, texture_registry: &TextureRegistry, x: i32, y: i32, ticks: u32) {
        let mut rng = StdRng::seed_from_u64(((x * y) + if self.liquid { ticks as i32 / 30 } else { 0 }) as u64);

        let do_cool_texture = rng.random_range(0..=1) == 0;

        renderer.blit_texture(&texture_registry[self.texture_id], BlitDesc {
            src: &Rect {
                x: if do_cool_texture { 8 } else { rng.random_range(0..=1) * Self::SUB_WIDTH as u32 + 24 },
                y: if do_cool_texture { 8 } else { rng.random_range(0..=1) * Self::SUB_HEIGHT as u32 },
                w: Self::SUB_WIDTH as u32,
                h: Self::SUB_HEIGHT as u32,
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
    pub neighbor_check: NeighborCheck,
    pub liquid: bool,
}