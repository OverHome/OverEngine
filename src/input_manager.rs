use std::collections::HashSet;
use sdl2::event::Event;
use sdl2::{EventPump, Sdl};
use sdl2::keyboard::Keycode;

pub(crate) struct InputManager<> {
    event_pump: EventPump,
    pressed_keys: HashSet<Keycode>,
    down_keys: HashSet<Keycode>,
    up_keys: HashSet<Keycode>,
    is_quit: bool,
}

impl InputManager<> {
    pub(crate) fn new(sdl_context:&Sdl) -> Self {
        InputManager {
            event_pump: sdl_context.event_pump().unwrap(),
            pressed_keys: HashSet::new(),
            down_keys: HashSet::new(),
            up_keys: HashSet::new(),
            is_quit: false,
        }
    }

    pub(crate) fn update(&mut self) {
        self.down_keys.clear();
        self.up_keys.clear();

        for event in self.event_pump.poll_iter() {
            match event {
                Event::Quit { .. } => {self.is_quit = true},
                Event::KeyDown { repeat:false, keycode, .. } => {
                    if let Some(key) = keycode {
                        self.pressed_keys.insert(key);
                        self.up_keys.insert(key);
                    }
                },
                Event::KeyUp {keycode, .. } => {
                    if let Some(key) = keycode {
                        self.pressed_keys.remove(&key);
                        self.down_keys.insert(key);
                    }
                },
                _ => {}
            }
        }
    }

    pub(crate) fn get_key(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub(crate) fn get_key_down(&self, key: Keycode) -> bool {
        self.down_keys.contains(&key)
    }

    pub(crate) fn get_key_up(&self, key: Keycode) -> bool {
        self.up_keys.contains(&key)
    }
    pub(crate) fn get_direction(&self) -> (i32, i32) {
        let mut direction = (0, 0);
        if self.get_key(Keycode::W) || self.get_key(Keycode::Up){
            direction.1 += 1;
        }
        if self.get_key(Keycode::S) || self.get_key(Keycode::Down){
            direction.1 -= 1;
        }
        if self.get_key(Keycode::D) || self.get_key(Keycode::Right){
            direction.0 += 1;
        }
        if self.get_key(Keycode::A) || self.get_key(Keycode::Left){
            direction.0 -= 1;
        }
        direction
    }
    pub(crate) fn is_exit(&self, ) -> bool {
        self.is_quit
    }
}