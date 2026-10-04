use crate::level::Level;
use crate::level::tile_registry::TileRegistry;
use crate::math::{Point, Rect};
use crate::renderer::color::Color;
use crate::renderer::palette::{Colors, MAX_COLORS};
use crate::renderer::texture_registry::TextureRegistry;
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::unpack_colors;
use bitflags::bitflags;
use rand::rngs::StdRng;
use rand::{RngExt, SeedableRng};

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
pub enum BlitType {
    Dirt,
    Grass,
    Stone,
    Tree,
}

pub struct Tile {
    id: usize,
    name: String,
    colors: Colors,
    texture_id: usize,
    blit_type: BlitType,
    liquid: bool,
    solid: bool,
    sub_tile_name: Option<String>,
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
            sub_tile_name: tile_payload.sub_tile,
            solid: tile_payload.solid,
            id,
            name: tile_payload.name,
            colors,
            texture_id: texture_registry[tile_payload.texture_name.as_str()],
            blit_type: tile_payload.blit_type,
            liquid: tile_payload.liquid,
        }
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn sub_tile(&self) -> &Option<String> {
        &self.sub_tile_name
    }

    pub fn blit(
        &self,
        renderer: &mut Renderer,
        texture_registry: &TextureRegistry,
        tile_registry: &TileRegistry,
        x: i32,
        y: i32,
        neighbor_mask: &NeighborMask,
        ticks: u32,
    ) {

        match self.blit_type {
            BlitType::Dirt => renderer.blit_texture(
                &texture_registry[self.texture_id],
                BlitDesc {
                    src: &Rect {
                        x: 0,
                        y: 0,
                        w: Self::SUB_WIDTH as u32,
                        h: Self::SUB_HEIGHT as u32,
                    },
                    dst: &Point { x, y },
                    colors: &self.colors,
                    blit_flags: &BlitFlags::empty(),
                },
            ),
            BlitType::Grass => {
                if neighbor_mask.contains(NeighborMask::LEFT)
                    && neighbor_mask.contains(NeighborMask::UP)
                {
                    self.blit_center_random(renderer, texture_registry, x, y, ticks)
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: neighbor_mask.contains(NeighborMask::LEFT) as u32
                                    * Self::SUB_WIDTH as u32,
                                y: neighbor_mask.contains(NeighborMask::UP) as u32
                                    * Self::SUB_HEIGHT as u32,
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point { x, y },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::RIGHT)
                    && neighbor_mask.contains(NeighborMask::UP)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x + Self::SUB_WIDTH,
                        y,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: 16
                                    - neighbor_mask.contains(NeighborMask::RIGHT) as u32
                                        * Self::SUB_WIDTH as u32,
                                y: neighbor_mask.contains(NeighborMask::UP) as u32
                                    * Self::SUB_HEIGHT as u32,
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x: x + Self::SUB_WIDTH,
                                y,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::LEFT)
                    && neighbor_mask.contains(NeighborMask::DOWN)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x,
                        y + Self::SUB_HEIGHT,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: neighbor_mask.contains(NeighborMask::LEFT) as u32
                                    * Self::SUB_WIDTH as u32,
                                y: 16
                                    - neighbor_mask.contains(NeighborMask::DOWN) as u32
                                        * Self::SUB_HEIGHT as u32,
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x,
                                y: y + Self::SUB_HEIGHT,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::RIGHT)
                    && neighbor_mask.contains(NeighborMask::DOWN)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x + Self::SUB_WIDTH,
                        y + Self::SUB_HEIGHT,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: 16
                                    - (neighbor_mask.contains(NeighborMask::RIGHT) as u32
                                        * Self::SUB_WIDTH as u32),
                                y: 16
                                    - (neighbor_mask.contains(NeighborMask::DOWN) as u32
                                        * Self::SUB_HEIGHT as u32),
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x: x + Self::SUB_WIDTH,
                                y: y + Self::SUB_HEIGHT,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }
            }
            BlitType::Stone => {
                if neighbor_mask.contains(NeighborMask::LEFT)
                    && neighbor_mask.contains(NeighborMask::UP)
                    && neighbor_mask.contains(NeighborMask::UP_LEFT)
                {
                    self.blit_center_random(renderer, texture_registry, x, y, ticks)
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: if !neighbor_mask.contains(NeighborMask::UP_LEFT)
                                    && neighbor_mask.contains(NeighborMask::LEFT)
                                    && neighbor_mask.contains(NeighborMask::UP)
                                {
                                    24
                                } else {
                                    neighbor_mask.contains(NeighborMask::LEFT) as u32
                                        * Self::SUB_WIDTH as u32
                                },
                                y: if !neighbor_mask.contains(NeighborMask::UP_LEFT)
                                    && neighbor_mask.contains(NeighborMask::LEFT)
                                    && neighbor_mask.contains(NeighborMask::UP)
                                {
                                    16
                                } else {
                                    neighbor_mask.contains(NeighborMask::UP) as u32
                                        * Self::SUB_HEIGHT as u32
                                },
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point { x, y },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::RIGHT)
                    && neighbor_mask.contains(NeighborMask::UP)
                    && neighbor_mask.contains(NeighborMask::UP_RIGHT)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x + Self::SUB_WIDTH,
                        y,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: if !neighbor_mask.contains(NeighborMask::UP_RIGHT)
                                    && neighbor_mask.contains(NeighborMask::RIGHT)
                                    && neighbor_mask.contains(NeighborMask::UP)
                                {
                                    32
                                } else {
                                    16
                                        - neighbor_mask.contains(NeighborMask::RIGHT) as u32
                                        * Self::SUB_WIDTH as u32
                                },
                                y: if !neighbor_mask.contains(NeighborMask::UP_RIGHT)
                                    && neighbor_mask.contains(NeighborMask::RIGHT)
                                    && neighbor_mask.contains(NeighborMask::UP)
                                {
                                    16
                                } else {
                                    neighbor_mask.contains(NeighborMask::UP) as u32
                                        * Self::SUB_HEIGHT as u32
                                },
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x: x + Self::SUB_WIDTH,
                                y,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::LEFT)
                    && neighbor_mask.contains(NeighborMask::DOWN) && neighbor_mask.contains(NeighborMask::DOWN_LEFT)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x,
                        y + Self::SUB_HEIGHT,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: if !neighbor_mask.contains(NeighborMask::DOWN_LEFT)
                                    && neighbor_mask.contains(NeighborMask::LEFT)
                                    && neighbor_mask.contains(NeighborMask::DOWN)
                                {
                                    24
                                } else {
                                    neighbor_mask.contains(NeighborMask::LEFT) as u32
                                        * Self::SUB_WIDTH as u32
                                },
                                y: if !neighbor_mask.contains(NeighborMask::DOWN_LEFT)
                                    && neighbor_mask.contains(NeighborMask::LEFT)
                                    && neighbor_mask.contains(NeighborMask::DOWN)
                                {
                                    24
                                } else {
                                    16
                                        - neighbor_mask.contains(NeighborMask::DOWN) as u32
                                        * Self::SUB_HEIGHT as u32
                                },
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x,
                                y: y + Self::SUB_HEIGHT,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }

                if neighbor_mask.contains(NeighborMask::RIGHT)
                    && neighbor_mask.contains(NeighborMask::DOWN) && neighbor_mask.contains(NeighborMask::DOWN_RIGHT)
                {
                    self.blit_center_random(
                        renderer,
                        texture_registry,
                        x + Self::SUB_WIDTH,
                        y + Self::SUB_HEIGHT,
                        ticks,
                    )
                } else {
                    renderer.blit_texture(
                        &texture_registry[self.texture_id],
                        BlitDesc {
                            src: &Rect {
                                x: if !neighbor_mask.contains(NeighborMask::DOWN_RIGHT)
                                    && neighbor_mask.contains(NeighborMask::RIGHT)
                                    && neighbor_mask.contains(NeighborMask::DOWN)
                                {
                                    32
                                } else {
                                    16
                                        - (neighbor_mask.contains(NeighborMask::RIGHT) as u32
                                        * Self::SUB_WIDTH as u32)
                                },
                                y: if !neighbor_mask.contains(NeighborMask::DOWN_RIGHT)
                                    && neighbor_mask.contains(NeighborMask::RIGHT)
                                    && neighbor_mask.contains(NeighborMask::DOWN)
                                {
                                    24
                                } else {
                                    16
                                        - neighbor_mask.contains(NeighborMask::DOWN) as u32
                                        * Self::SUB_HEIGHT as u32
                                },
                                w: Self::SUB_WIDTH as u32,
                                h: Self::SUB_HEIGHT as u32,
                            },
                            dst: &Point {
                                x: x + Self::SUB_WIDTH,
                                y: y + Self::SUB_HEIGHT,
                            },
                            colors: &self.colors,
                            blit_flags: &BlitFlags::empty(),
                        },
                    );
                }
            }
            BlitType::Tree => {
                renderer.blit_texture(
                    &texture_registry[self.texture_id],
                    BlitDesc {
                        src: &Rect {
                            x: if neighbor_mask.contains(NeighborMask::UP_LEFT)
                                && neighbor_mask.contains(NeighborMask::LEFT)
                                && neighbor_mask.contains(NeighborMask::UP)
                            {
                                8
                            } else {
                                0
                            },
                            y: if neighbor_mask.contains(NeighborMask::UP_LEFT)
                                && neighbor_mask.contains(NeighborMask::LEFT)
                                && neighbor_mask.contains(NeighborMask::UP)
                            {
                                8
                            } else {
                                0
                            },
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point { x, y },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    },
                );

                renderer.blit_texture(
                    &texture_registry[self.texture_id],
                    BlitDesc {
                        src: &Rect {
                            x: if neighbor_mask.contains(NeighborMask::UP_RIGHT)
                                && neighbor_mask.contains(NeighborMask::RIGHT)
                                && neighbor_mask.contains(NeighborMask::UP)
                            {
                                16
                            } else {
                                24
                            },
                            y: if neighbor_mask.contains(NeighborMask::UP_RIGHT)
                                && neighbor_mask.contains(NeighborMask::RIGHT)
                                && neighbor_mask.contains(NeighborMask::UP)
                            {
                                8
                            } else {
                                0
                            },
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point {
                            x: x + Self::SUB_WIDTH,
                            y,
                        },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    },
                );

                renderer.blit_texture(
                    &texture_registry[self.texture_id],
                    BlitDesc {
                        src: &Rect {
                            x: if neighbor_mask.contains(NeighborMask::DOWN_LEFT)
                                && neighbor_mask.contains(NeighborMask::LEFT)
                                && neighbor_mask.contains(NeighborMask::DOWN)
                            {
                                8
                            } else {
                                0
                            },
                            y: if neighbor_mask.contains(NeighborMask::DOWN_LEFT)
                                && neighbor_mask.contains(NeighborMask::LEFT)
                                && neighbor_mask.contains(NeighborMask::DOWN)
                            {
                                16
                            } else {
                                24
                            },
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point {
                            x,
                            y: y + Self::SUB_HEIGHT,
                        },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    },
                );

                renderer.blit_texture(
                    &texture_registry[self.texture_id],
                    BlitDesc {
                        src: &Rect {
                            x: if neighbor_mask.contains(NeighborMask::DOWN_RIGHT)
                                && neighbor_mask.contains(NeighborMask::RIGHT)
                                && neighbor_mask.contains(NeighborMask::DOWN)
                            {
                                16
                            } else {
                                24
                            },
                            y: if neighbor_mask.contains(NeighborMask::DOWN_RIGHT)
                                && neighbor_mask.contains(NeighborMask::RIGHT)
                                && neighbor_mask.contains(NeighborMask::DOWN)
                            {
                                16
                            } else {
                                24
                            },
                            w: Self::SUB_WIDTH as u32,
                            h: Self::SUB_HEIGHT as u32,
                        },
                        dst: &Point {
                            x: x + Self::SUB_WIDTH,
                            y: y + Self::SUB_HEIGHT,
                        },
                        colors: &self.colors,
                        blit_flags: &BlitFlags::empty(),
                    },
                );

            }
        }
    }

    pub fn solid(&self) -> bool {
        self.solid
    }

    pub fn connects(&self, level: &Level, x: i32, y: i32, tile_registry: &TileRegistry) -> bool {
        if x < 0 || x >= Level::WIDTH as i32 || y < 0 || y >= Level::HEIGHT as i32 {
            return false;
        }

        level.tile_at(x as usize, y as usize) == self.id
            || if let Some(sub_tile_name) = tile_registry[level.tile_at(x as usize, y as usize)].sub_tile() { *sub_tile_name == self.name } else { false }
    }

    fn blit_center_random(
        &self,
        renderer: &mut Renderer,
        texture_registry: &TextureRegistry,
        x: i32,
        y: i32,
        ticks: u32,
    ) {
        let mut rng = StdRng::seed_from_u64(
            ((x * y) + if self.liquid { ticks as i32 / 30 } else { 0 }) as u64,
        );

        let do_cool_texture = rng.random_range(0..=1) == 0;

        renderer.blit_texture(
            &texture_registry[self.texture_id],
            BlitDesc {
                src: &Rect {
                    x: if do_cool_texture {
                        8
                    } else {
                        rng.random_range(0..=1) * Self::SUB_WIDTH as u32 + 24
                    },
                    y: if do_cool_texture {
                        8
                    } else {
                        rng.random_range(0..=1) * Self::SUB_HEIGHT as u32
                    },
                    w: Self::SUB_WIDTH as u32,
                    h: Self::SUB_HEIGHT as u32,
                },
                dst: &Point { x, y },
                colors: &self.colors,
                blit_flags: &BlitFlags::empty(),
            },
        )
    }
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct TilePayload {
    pub name: String,
    pub colors: [Option<Color>; MAX_COLORS],
    pub texture_name: String,
    pub blit_type: BlitType,
    #[serde(default)]
    pub liquid: bool,
    pub solid: bool,
    #[serde(default)]
    pub sub_tile: Option<String>
}
