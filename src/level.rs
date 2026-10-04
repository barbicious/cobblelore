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
    pub const WIDTH: usize = 20;
    pub const HEIGHT: usize = 20;

    pub fn new(tile_registry: &TileRegistry) -> Self {
        let mut tiles = [tile_registry["cobblelore::grass"]; Self::WIDTH * Self::HEIGHT];

        for y in 3..7 {
            for x in 5..12 {
                tiles[Self::idx(x, y)] = tile_registry["cobblelore::stone"];
            }
        }

        for y in 0..3 {
            for x in 10..15 {
                tiles[Self::idx(x, y)] = tile_registry["cobblelore::stone"];
            }
        }

        for y in 0..6 {
            for x in 12..14 {
                tiles[Self::idx(x, y)] = tile_registry["cobblelore::stone"];
            }
        }

        for y in 5..8 {
            for x in 8..17 {
                tiles[Self::idx(x, y)] = tile_registry["cobblelore::water"];
            }
        }

        for y in 3..6 {
            for x in 3..6 {
                tiles[Self::idx(x, y)] = tile_registry["cobblelore::oak_tree"];
            }
        }

        Self { tiles }
    }

    pub fn blit(
        &self,
        renderer: &mut Renderer,
        texture_registry: &TextureRegistry,
        tile_registry: &TileRegistry,
        ticks: u32,
    ) {
        for y in 0..Self::HEIGHT {
            for x in 0..Self::WIDTH {
                let tile = &tile_registry[self.tiles[Self::idx(x, y)]];

                if let Some(sub_tile) = tile.sub_tile() {
                    let mut neighbor_mask = NeighborMask::empty();

                    let tile = &tile_registry[tile_registry[sub_tile.as_str()]];

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

                    tile.blit(
                        renderer,
                        texture_registry,
                        tile_registry,
                        x as i32 * Tile::WIDTH,
                        y as i32 * Tile::HEIGHT,
                        &neighbor_mask,
                        ticks,
                    );
                }
                
                let mut neighbor_mask = NeighborMask::empty();

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

                tile.blit(
                    renderer,
                    texture_registry,
                    tile_registry,
                    x as i32 * Tile::WIDTH,
                    y as i32 * Tile::HEIGHT,
                    &neighbor_mask,
                    ticks,
                );
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

    #[inline]
    pub fn is_tile_solid(&self, tile_registry: &TileRegistry, x: usize, y: usize) -> bool {
        tile_registry[self.tiles[Self::idx(x, y)]].solid()
    }
}
