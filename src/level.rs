use crate::level::tile::{NeighborMask, Tile};
use crate::level::tile_registry::TileRegistry;
use crate::renderer::Renderer;
use crate::renderer::texture_registry::TextureRegistry;

pub mod tile;
pub mod tile_registry;

pub struct Level {
    tiles: [usize; Self::WIDTH * Self::HEIGHT],
}

impl Level {
    pub const WIDTH: usize = 64;
    pub const HEIGHT: usize = 32;

    pub fn new() -> Self {
        let mut tiles = [0; Self::WIDTH * Self::HEIGHT];

        for y in 3..7 {
            for x in 5..12 {
                tiles[Self::idx(x, y)] = 1;
            }
        }

        Self { tiles }
    }

    pub fn blit(&self, renderer: &mut Renderer, texture_registry: &TextureRegistry, tile_registry: &TileRegistry, ticks: u32) {
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let mut neighbor_mask = NeighborMask::empty();

                let tile = &tile_registry[self.tiles[Self::idx(x, y)]];

                if tile.connects(self, x as i32 - 1, y as i32 - 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::UP_LEFT, true)
                }

                if tile.connects(self, x as i32 + 1, y as i32 - 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::UP_RIGHT, true)
                }

                if tile.connects(self, x as i32 - 1, y as i32 + 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::DOWN_LEFT, true)
                }

                if tile.connects(self, x as i32 + 1, y as i32 + 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::DOWN_RIGHT, true)
                }

                if tile.connects(self, x as i32, y as i32 - 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::UP, true)
                }

                if tile.connects(self, x as i32, y as i32 + 1, tile_registry) {
                    neighbor_mask.set(NeighborMask::DOWN, true)
                }

                if tile.connects(self, x as i32 - 1, y as i32, tile_registry) {
                    neighbor_mask.set(NeighborMask::LEFT, true)
                }

                if tile.connects(self, x as i32 + 1, y as i32, tile_registry) {
                    neighbor_mask.set(NeighborMask::RIGHT, true)
                }

                tile_registry[self.tiles[Self::idx(x, y)]].blit(renderer, texture_registry, x as i32 * Tile::WIDTH, y as i32 * Tile::HEIGHT, neighbor_mask, ticks);
            }
        }
    }
    
    #[inline]
    pub fn tile_at(&self, x: usize, y: usize) -> usize {
        self.tiles[Self::idx(x, y)]
    }

    #[inline]
    const fn idx(x: usize, y: usize) -> usize {
        y * Self::WIDTH + x
    }
}