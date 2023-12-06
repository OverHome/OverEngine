use std::collections::HashSet;
use std::sync::{Arc, Mutex};
use sdl2::event::Event;
use sdl2::{EventPump, Sdl, SdlDrop};
use sdl2::keyboard::Keycode;
use sdl2::sys::KeyCode;
use specs::{Entities, Join, System, WriteStorage};
use specs_derive::Component;
use specs::prelude::*;

#[derive(Component, Debug, Clone)]
#[derive(Default)]
pub struct InputComponent<> {
    pub pressed_keys: HashSet<Keycode>,
    pub down_keys: HashSet<Keycode>,
    pub up_keys: HashSet<Keycode>,
    pub is_quit: bool,
}


impl InputComponent<> {
    pub(crate) fn get_key(&self, key: Keycode) -> bool {
        self.pressed_keys.contains(&key)
    }

    pub fn get_key_down(&self, key: &str) -> bool {
        let a = Keycode::from_name(key);
        match a {
            None => {false}
            Some(k) => {self.down_keys.contains(&k)}
        }

    }

    pub fn get_key_up(&self,  key: &str) -> bool {
        let a = Keycode::from_name(key);
        match a {
            None => {false}
            Some(k) => {self.up_keys.contains(&k)}
        }
    }
    pub fn get_direction(&self) -> [i32; 2] {
        let mut direction = [0, 0];
        if self.get_key(Keycode::W) || self.get_key(Keycode::Up) {
            direction[1] += 1;
        }
        if self.get_key(Keycode::S) || self.get_key(Keycode::Down) {
            direction[1] -= 1;
        }
        if self.get_key(Keycode::D) || self.get_key(Keycode::Right) {
            direction[0] += 1;
        }
        if self.get_key(Keycode::A) || self.get_key(Keycode::Left) {
            direction[0] -= 1;
        }
        direction
    }
    pub fn is_exit(&self) -> bool {
        self.is_quit
    }
}

// pub struct InputManager{
// }
//
// impl<'a> System<'a> for InputManager {
//     type SystemData = (Entities<'a>, WriteStorage<'a, InputComponent>,  Option<Read<'a, EventPump>>,);
//
//     fn run(&mut self, mut data: Self::SystemData) {
//         // for (ent, inp) in (&*data.0, &mut data.1, ).join() {
//         //     inp.down_keys.clear();
//         //     inp.up_keys.clear();
//
//
//         }
//     }