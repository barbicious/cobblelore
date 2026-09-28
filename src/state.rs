use crate::level::Level;
use crate::level::tile_registry::TileRegistry;
use crate::renderer::texture_registry::TextureRegistry;
use crate::renderer::Renderer;
use sdl3::event::Event;
use sdl3::EventPump;
use std::time::Instant;
use log::__private_api::Key;
use sdl3::keyboard::Scancode;
use crate::keyboard::Keyboard;
use crate::level::tile::Tile;
use crate::math::Rect;
use crate::renderer::camera::Camera;

const FIXED_DT: f32 = 1.0 / 60.0;

pub struct State {
    event_pump: EventPump,
    renderer: Renderer,
    level: Level,
    ticks: u32,
    keyboard: Keyboard,
}

impl State {
    pub fn new() -> anyhow::Result<Self> {
        let sdl_context = sdl3::init()?;

        let video = sdl_context.video()?;
        let window = video.window(format!("cobblelore: v{}", env!("CARGO_PKG_VERSION")).as_str(), 1280, 720).position_centered().build()?;
        let event_pump = sdl_context.event_pump()?;
        let canvas = window.into_canvas();
        let keyboard = Keyboard::new(&event_pump);

        Ok(Self { event_pump, renderer: Renderer::new(canvas, Camera::new(Rect {
            x: 0,
            y: 0,
            w: Level::WIDTH as i32 * Tile::WIDTH,
            h: Level::HEIGHT as i32 * Tile::HEIGHT,
        }, 4, 0))?, level: Level::new(), ticks: 0, keyboard })
    }

    pub fn run(mut self) -> anyhow::Result<()> {
        let texture_registry = TextureRegistry::new()?;
        let tile_registry = TileRegistry::new(&texture_registry)?;

        let mut tick_now = Instant::now();
        let mut tick_last = tick_now;

        let mut accumulator = 0.0;
        let mut x = 0;
        let mut y = 0;

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

                if self.keyboard.is_key_down(Scancode::A) {
                    x -= 1
                }

                if self.keyboard.is_key_down(Scancode::D) {
                    x += 1
                }

                self.renderer.camera_mut().set_x_offset(&mut x);

                if self.keyboard.is_key_down(Scancode::W) {
                    y -= 1
                }

                if self.keyboard.is_key_down(Scancode::S) {
                    y += 1
                }

                self.renderer.camera_mut().set_y_offset(&mut y);


                accumulator -= FIXED_DT;
            }

            self.renderer.flush();

            self.level.blit(&mut self.renderer, &texture_registry, &tile_registry, self.ticks);

            self.renderer.splat()?;
        }

        Ok(())
    }
}