use specs::prelude::*;
use sdl2::rect::{Point, Rect};
use sdl2::pixels::Color;
use sdl2::render::{WindowCanvas, Texture};

use crate::components::*;

pub type SystemData<'a> = (
    ReadStorage<'a, Position>,
    ReadStorage<'a, Sprite>,
);

pub fn render(
    canvas: &mut WindowCanvas,
    background: Color,
    textures: &[Texture],
    data: SystemData,
) -> Result<(), String> {
    canvas.set_draw_color(background);
    canvas.clear();

    let (width, height) = canvas.output_size()?;

    for (pos, sprite) in (&data.0, &data.1).join() {
        let screen_position = Point::new(pos.x, -pos.y) + Point::new(width as i32 / 2, height as i32 / 2);
        let screen_rect = Rect::from_center(screen_position, sprite.w, sprite.h);
        canvas.copy(&textures[sprite.spritesheet], Rect::new(sprite.x, sprite.y, sprite.h, sprite.w), screen_rect)?;
    }
    canvas.present();
    Ok(())
}