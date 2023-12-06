mod input_manager;
mod render_system;
mod components;
mod script_system;

use std::cmp::min;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::{Arc, Mutex};

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::keyboard::Keycode;
use sdl2::rect::{Point, Rect};
use specs::prelude::*;
use specs::rayon::iter::Positions;
use specs_derive::Component;
use crate::components::*;

use rhai::Engine;
use sdl2::event::Event;
use sdl2::SdlDrop;
use crate::input_manager::InputComponent;
use crate::script_system::ScriptSystem;

fn main() {
    let sdl_context = sdl2::init().expect("sdl error");
    let video_subsystem = sdl_context.video().expect("sdl video error");
    let window = video_subsystem.window("OverEngine", 800, 600)
        .position_centered()
        .build()
        .expect("window error");
    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let textures = [
        texture_creator.load_texture("assets/мем4.png").expect("texture error")
    ];

    let mut dispatcher = DispatcherBuilder::new()
        // .with(InputManager{}, "script_system", &[])
        .with(ScriptSystem {}, "script_system", &[])
        .build();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<InputComponent>();
    world.register::<Sprite>();
    world.register::<Script>();

    let a = world.create_entity()
        .with(Position { x: 0, y: 0 })
        .with(Sprite { spritesheet: 0, region: Rect::new(400, 0, 200, 200) })
        .with(Script { code: "assets/scripts/main.rhai".to_string() })
        .build();

    render_system::SystemData::setup(&mut world);

    world.insert(InputComponent {
        pressed_keys: Default::default(),
        down_keys: Default::default(),
        up_keys: Default::default(),
        is_quit: false,
    });
    world.maintain();

    // world.write_resource::<InputComponent>();

    let mut a = sdl_context.event_pump().unwrap();
    'running: loop {
        {
            let mut inp = world.write_resource::<InputComponent>();
            inp.down_keys.clear();
            inp.up_keys.clear();
            for event in a.poll_iter() {
                match event {
                    Event::Quit { .. } => {
                        inp.is_quit = true;
                        if inp.is_exit() {
                            break 'running;
                        }
                    }
                    Event::KeyDown { repeat: false, keycode, .. } => {
                        if let Some(key) = keycode {
                            inp.pressed_keys.insert(key);
                            inp.down_keys.insert(key);
                        }
                    }
                    Event::KeyUp { keycode, .. } => {
                        if let Some(key) = keycode {
                            inp.pressed_keys.remove(&key);
                            inp.up_keys.insert(key);
                        }
                    }
                    _ => {}
                }
            }
        }
        render_system::render(&mut canvas, Color::RGB(0, 255, 255),
                              &textures, world.system_data())
            .expect("render error");
        dispatcher.dispatch(&world);
        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 30));
    }
}