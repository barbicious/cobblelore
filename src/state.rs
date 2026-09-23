use sdl3::event::Event;
use sdl3::{EventPump, Sdl};
use sdl3::pixels::PixelFormat;
use sdl3::render::{ScaleMode, Texture, WindowCanvas};
use sdl3::video::{Window, WindowContext};
use crate::renderer::palette::Palette;
use crate::renderer::pixel_buffer::PixelBuffer;
use crate::renderer::Renderer;

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
        
        'running: loop {
            for event in self.event_pump.poll_iter() {
                if let Event::Quit { .. } = event {
                    break 'running;
                }
            }
            
            
            self.renderer.flush();
            
            self.renderer.blit_palette();
            
            self.renderer.splat()?;
        }

        Ok(())
    }
}