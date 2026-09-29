use crate::components::{PlayerComponent, PositionComponent, SpriteComponent};
use crate::keyboard::Keyboard;
use crate::level::Level;
use crate::level::tile::Tile;
use crate::level::tile_registry::TileRegistry;
use crate::math::{Point, Rect};
use crate::renderer::camera::Camera;
use crate::renderer::palette::Palette;
use crate::renderer::texture_registry::TextureRegistry;
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use hecs::World;
use log::__private_api::Key;
use sdl3::EventPump;
use sdl3::event::Event;
use sdl3::keyboard::Scancode;
use std::time::Instant;

const FIXED_DT: f32 = 1.0 / 60.0;

pub struct State {
    event_pump: EventPump,
    renderer: Renderer,
    level: Level,
    ticks: u32,
    keyboard: Keyboard,
    texture_registry: TextureRegistry,
    tile_registry: TileRegistry,
    world: World,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let sdl_context = sdl3::init()?;

        let video = sdl_context.video()?;
        let window = video
            .window(
                format!("cobblelore: v{}", env!("CARGO_PKG_VERSION")).as_str(),
                1280,
                720,
            )
            .position_centered()
            .build()?;
        let event_pump = sdl_context.event_pump()?;
        let canvas = window.into_canvas();
        let keyboard = Keyboard::new(&event_pump);

        let texture_registry = TextureRegistry::new()?;
        let tile_registry = TileRegistry::new(&texture_registry)?;

        Ok(Self {
            event_pump,
            renderer: Renderer::new(
                canvas,
                Camera::new(
                    Rect {
                        x: 0,
                        y: 0,
                        w: Level::WIDTH as i32 * Tile::WIDTH,
                        h: Level::HEIGHT as i32 * Tile::HEIGHT,
                    },
                    4,
                    0,
                ),
            )?,
            level: Level::new(),
            ticks: 0,
            keyboard,
            texture_registry,
            tile_registry,
            world: World::new(),
        })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let mut tick_now = Instant::now();
        let mut tick_last = tick_now;

        let mut accumulator = 0.0;

        self.world.spawn((
            PlayerComponent,
            PositionComponent { x: 0, y: 0 },
            SpriteComponent {
                colors: [
                    Some(Palette::palettize(3, 2, 1)),
                    Some(Palette::palettize(1, 5, 1)),
                    Some(Palette::palettize(2, 2, 2)),
                    Some(Palette::palettize(1, 2, 3)),
                ],
                src: Rect {
                    x: 0,
                    y: 0,
                    w: 16,
                    h: 16,
                },
                texture_id: self.texture_registry["player"],
                flags: BlitFlags::empty(),
            },
        ));

        'running: loop {
            self.event_pump.pump_events();

            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }

            tick_now = Instant::now();
            let delta_time = (tick_now - tick_last).as_secs_f32();
            tick_last = tick_now;

            println!("{delta_time}");

            accumulator += delta_time;

            while accumulator >= FIXED_DT {
                self.ticks += 1;

                self.keyboard.tick(&self.event_pump);

                for (pos, _) in &mut self
                    .world
                    .query::<(&mut PositionComponent, &PlayerComponent)>()
                {
                    if self.keyboard.is_key_down(Scancode::A) {
                        pos.x -= 1
                    }

                    if self.keyboard.is_key_down(Scancode::D) {
                        pos.x += 1
                    }

                    self.renderer.camera_mut().set_x_offset(pos.x + 8);

                    if self.keyboard.is_key_down(Scancode::W) {
                        pos.y -= 1
                    }

                    if self.keyboard.is_key_down(Scancode::S) {
                        pos.y += 1
                    }

                    self.renderer.camera_mut().set_y_offset(pos.y + 8);
                }

                accumulator -= FIXED_DT;
            }

            self.renderer.flush();

            self.level.blit(
                &mut self.renderer,
                &self.texture_registry,
                &self.tile_registry,
                self.ticks,
            );

            for (pos, sprite) in &mut self.world.query::<(&PositionComponent, &SpriteComponent)>() {
                self.renderer.blit_texture(
                    &self.texture_registry[self.texture_registry["player"]],
                    BlitDesc {
                        src: &sprite.src,
                        dst: &Point { x: pos.x, y: pos.y },
                        colors: &sprite.colors,
                        blit_flags: &sprite.flags,
                    },
                );
            }

            self.renderer.splat()?;
        }

        Ok(())
    }
}
