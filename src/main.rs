mod input_manager;

use std::cmp::min;
use input_manager::InputManager;
use sdl2::pixels::Color;
use std::time::Duration;
use sdl2::render::{Texture, WindowCanvas};
use sdl2::image::{self, LoadTexture, InitFlag};
use sdl2::rect::{Point, Rect};

use specs::prelude::*;
use specs_derive::Component;

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Position(Point);

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Velocity {
    speed: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
struct Sprite {
    spritesheet: usize,
    region: Rect,
}

struct Player {
    position: Point,
    sprite: Rect,
    directinal: Point
}

fn render(
    canvas: &mut WindowCanvas,
    texture: &Texture,
    player: &Player,
) -> Result<(), String> {
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    let screen_position = player.position + Point::new(width as i32 / 2, height as i32 / 2);
    let screen_rect = Rect::from_center(screen_position, player.sprite.width(), player.sprite.height());
    canvas.copy(texture, player.sprite, screen_rect)?;

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.present();

    Ok(())
}

fn update(){

}

fn fixed_update(player: &mut Player){

    player.position += player.directinal;
}

fn main() {
    let sdl_context = sdl2::init().expect("sdl error");
    let video_subsystem = sdl_context.video().expect("sdl video error");
    let _image_context = image::init(InitFlag::PNG | InitFlag::JPG);

    let window = video_subsystem.window("OverEngine", 800, 600)
        .position_centered()
        .build()
        .expect("window error");
    let mut canvas = window.into_canvas().build().expect("could not make a canvas");

    let texture_creator = canvas.texture_creator();
    let texture = texture_creator.load_texture("assets/мем4.png").expect("texture error");

    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();

    let mut player = Player {
        position: Point::new(0, 0),
        sprite: Rect::new(75, 130, 200, 200),
        directinal: Point::new(1, 1),
    };

    let target_fixed_updates_per_second = 30;
    let fixed_update_interval = Duration::from_millis(1000 / target_fixed_updates_per_second);
    let fixed_update_interval_ms = fixed_update_interval.as_millis() as u64;

    let mut input_manager = InputManager::new(&sdl_context);
    let sdl_timer = sdl_context.timer().unwrap();
    let mut previous_time = sdl_timer.ticks() as u64;

    'running: loop {
        input_manager.update();
        let current_time = sdl_timer.ticks() as u64;
        for _ in 0..min((current_time - previous_time) / fixed_update_interval_ms, target_fixed_updates_per_second) {
            fixed_update(&mut player);
            previous_time += fixed_update_interval_ms;
        }

        render(&mut canvas, &texture, &player).expect("render error");
        update();

        // ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60 * 2));
        if input_manager.is_exit() {
            break 'running
        }
    }
}