mod input_manager;
mod components;
mod script_system;
mod collision_system;
mod render_system;

use std::io::{Read, Write};
use std::ops::Deref;

use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::image::{self, LoadTexture, InitFlag};
use specs::{
    prelude::*,
    saveload::{
        DeserializeComponents, MarkedBuilder, SerializeComponents, SimpleMarker,
        SimpleMarkerAllocator,
    },
};
use crate::components::*;
use std::any::Any;
use std::fs::File;
use sdl2::event::Event;
use crate::collision_system::CollisionSystem;
use crate::input_manager::InputComponent;
use crate::script_system::ScriptSystem;
use serde::{Serialize, Deserialize, Deserializer};
use serde_json::Value;

struct NetworkSync;

fn main() {
    let sdl_context = sdl2::init().expect("sdl error");
    let video_subsystem = sdl_context.video().expect("sdl video error");
    let window = video_subsystem.window("OverEngine", 800, 600)
        .position_centered()
        .build()
        .expect("window error");
    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();


    let mut dispatcher = DispatcherBuilder::new()
        .with(CollisionSystem, "collision_system", &[])
        .with(ScriptSystem {}, "script_system", &["collision_system"])
        .build();

    let mut world = World::new();
    world.register::<Position>();
    world.register::<InputComponent>();
    world.register::<Collider>();
    world.register::<Sprite>();
    world.register::<Script>();

    render_system::SystemData::setup(&mut world);

    world.insert(InputComponent {
        pressed_keys: Default::default(),
        down_keys: Default::default(),
        up_keys: Default::default(),
        is_quit: false,
    });
    world.insert(Statik { buffer: [0, 0, 0, 0, 0, 0, 0, 0] });
    world.maintain();

    let file_path = "assets/imgs.json";
    let mut file = File::open(file_path).expect("Error open imgs file");

    // Create a buffer to hold the file contents
    let mut data = String::new();

    // Read the file contents into the buffer
    file.read_to_string(&mut data).expect("Error read imgs file");

    let v: Value = serde_json::from_str(&*data).unwrap();

    let mut textures = vec![
        // texture_creator.load_texture("assets/image/мем4.png").expect("texture error"),
        // texture_creator.load_texture("assets/image/img.png").expect("texture error")
    ];

    for i in 0..1000 {
        if (!v[i].is_null()) {
            textures.push(texture_creator.load_texture(v[i].as_str().unwrap_or("")).expect("texture error"))
        }
    }

    let file_path = "assets/world.json";
    let mut file = File::open(file_path).expect("Error open world file");

    // Create a buffer to hold the file contents
    let mut data = String::new();

    // Read the file contents into the buffer
    file.read_to_string(&mut data).expect("Error read world file");

    let v: Value = serde_json::from_str(&*data).unwrap();

    for i in 0..1000 {
        if (!v[i].is_null()) {
            let mut e = world.create_entity();
            if (!v[i]["position"].is_null()) {
                e = e.with(Position {
                    x: v[i]["position"]["x"].as_i64().unwrap_or(0) as i32,
                    y: v[i]["position"]["y"].as_i64().unwrap_or(0) as i32
                });
            }
            if (!v[i]["sprite"].is_null()) {
                e = e.with(Sprite {
                    spritesheet: v[i]["sprite"]["spritesheet"].as_i64().unwrap_or(0) as usize,
                    x: v[i]["sprite"]["x"].as_i64().unwrap_or(0) as i32,
                    y: v[i]["sprite"]["y"].as_i64().unwrap_or(0) as i32,
                    h: v[i]["sprite"]["h"].as_i64().unwrap_or(0) as u32,
                    w: v[i]["sprite"]["w"].as_i64().unwrap_or(0) as u32,
                });
            }

            if (!v[i]["script"].is_null()) {
                e = e.with( Script {
                    code: v[i]["script"]["code"].as_str().unwrap_or("").parse().unwrap(),
                    buffer: [0,0,0,0,0,0,0,0]
                });
            }

            if (!v[i]["collider"].is_null()) {
                e = e.with( Collider {
                    x: v[i]["collider"]["size"]["x"].as_i64().unwrap_or(0) as i32,
                    y: v[i]["collider"]["size"]["y"].as_i64().unwrap_or(0) as i32,
                    is_collision: false,
                    collision_start: false,
                    collision_end: false,
                });
            }
        }
    }

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
        render_system::render(&mut canvas, Color::RGB(0, 200, 200),
                              &textures, world.system_data())
            .expect("render error");
        dispatcher.dispatch(&world);
        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 320));
    }
}