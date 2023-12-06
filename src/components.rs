use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{Arc, Mutex, RwLock};
use specs::prelude::*;
use specs_derive::Component;
use sdl2::rect::{Point, Rect};
#[derive(Component, Debug, Clone,)]
#[storage(VecStorage)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Velocity {
    pub x: i32,
    pub y: i32,
}
#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub region: Rect,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Move {
    pub direct:Point,
    pub speed:i32,
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Script {
    pub code:String,
}