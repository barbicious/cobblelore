use std::time::Instant;
use sdl3::event::Event;
use sdl3::{EventPump, Sdl};
use crate::level::Level;
use crate::level::tile_registry::TileRegistry;
use crate::math::{Point, Rect};
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::renderer::palette::Palette;
use crate::renderer::texture::Texture;
use crate::renderer::texture_registry::TextureRegistry;

const FIXED_DT: f32 = 1.0 / 60.0;

pub struct State {
    event_pump: EventPump,
    renderer: Renderer,
    level: Level,
    ticks: u32,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let sdl_context = sdl3::init()?;

        let video = sdl_context.video()?;
        let window = video.window(format!("cobblelore: v{}", env!("CARGO_PKG_VERSION")).as_str(), 1280, 720).position_centered().build()?;
        let event_pump = sdl_context.event_pump()?;
        let canvas = window.into_canvas();

        Ok(Self { event_pump, renderer: Renderer::new(canvas)?, level: Level::new(), ticks: 0 })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let texture_registry = TextureRegistry::new()?;
        let tile_registry = TileRegistry::new(&texture_registry)?;

        let mut tick_now = Instant::now();
        let mut tick_last = tick_now;

        let mut accumulator = 0.0;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }

            tick_now = Instant::now();
            let delta_time = (tick_now - tick_last).as_secs_f32();
            tick_last = tick_now;

            accumulator += delta_time;

            while accumulator >= FIXED_DT {
                self.ticks += 1;

                accumulator -= FIXED_DT;
            }

            self.renderer.flush();

            self.level.blit(&mut self.renderer, &texture_registry, &tile_registry, self.ticks);

            self.renderer.splat()?;
        }

        Ok(())
    }
}