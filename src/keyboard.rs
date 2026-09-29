use sdl3::EventPump;
use sdl3::keyboard::Scancode;

pub struct Keyboard {
    last_keys: Vec<Scancode>,
    current_keys: Vec<Scancode>,
}

impl Keyboard {
    pub fn new(event_pump: &EventPump) -> Self {
        let last_keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .collect::<Vec<Scancode>>();
        let current_keys = last_keys.clone();

        Self {
            last_keys,
            current_keys,
        }
    }

    pub fn tick(&mut self, event_pump: &EventPump) {
        self.last_keys = self.current_keys.clone();

        self.current_keys = event_pump
            .keyboard_state()
            .pressed_scancodes()
            .collect::<Vec<Scancode>>();
    }

    pub fn is_key_down(&self, key: Scancode) -> bool {
        self.current_keys.contains(&key)
    }

    pub fn is_key_pressed(&self, key: Scancode) -> bool {
        self.current_keys.contains(&key) && !self.last_keys.contains(&key)
    }
}
