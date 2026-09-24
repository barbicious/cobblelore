use sdl3::event::Event;
use sdl3::{EventPump, Sdl};
use crate::level::tile_registry::TileRegistry;
use crate::math::{Point, Rect};
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::renderer::palette::Palette;
use crate::renderer::texture::Texture;
use crate::renderer::texture_registry::TextureRegistry;

pub struct State {
    event_pump: EventPump,
    renderer: Renderer,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let sdl_context = sdl3::init()?;

        let video = sdl_context.video()?;
        let window = video.window(format!("cobblelore: v{}", env!("CARGO_PKG_VERSION")).as_str(), 1280, 720).position_centered().build()?;
        let event_pump = sdl_context.event_pump()?;
        let canvas = window.into_canvas();

        Ok(Self { event_pump, renderer: Renderer::new(canvas)? })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let texture_registry = TextureRegistry::new()?;
        let tile_registry = TileRegistry::new(&texture_registry)?;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }


            self.renderer.flush();

            for y in 0..30 {
                for x in 0..30 {
                    tile_registry[0].blit(&mut self.renderer, &texture_registry, x * 24, y * 24);
                }
            }

            self.renderer.splat()?;
        }

        Ok(())
    }
}