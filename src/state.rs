use sdl3::event::Event;
use sdl3::{EventPump, Sdl};
use crate::math::{Point, Rect};
use crate::renderer::{BlitDesc, BlitFlags, Renderer};
use crate::renderer::palette::Palette;
use crate::renderer::texture::Texture;

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
        let texture = Texture::new("res/textures/ground.png")?;

        'running: loop {
            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }


            self.renderer.flush();

            self.renderer.blit_palette();

            self.renderer.blit_texture(&texture, BlitDesc {
                src: &Rect {
                    x: 0,
                    y: 0,
                    w: 16,
                    h: 24
                },
                dst: &Point {
                    x: 0,
                    y: 0,
                },
                colors: &[
                    Some(Palette::palettize(3 , 2, 1)),
                    Some(Palette::palettize(1 , 1, 1)),
                    Some(Palette::palettize(5 , 2, 1)),
                    Some(Palette::palettize(3 , 5, 1)),
                ],
                blit_flags: &BlitFlags::empty(),
            });

            self.renderer.splat()?;
        }

        Ok(())
    }
}