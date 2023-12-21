use specs::prelude::*;
use specs_derive::Component;
use serde::Serialize;
use serde_derive::Deserialize;

#[derive(Component, Debug, Clone, Hash)]
#[derive(Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Position{
    pub x: i32,
    pub y: i32,
}

#[derive(Component, Debug, Clone, PartialEq, Eq, Hash)]
#[derive(Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Collider {
    pub x: i32,
    pub y: i32,
    pub is_collision:bool,
    pub collision_start:bool,
    pub collision_end:bool,
}

#[derive(Component, Debug)]
#[derive(Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Sprite {
    pub spritesheet: usize,
    pub x: i32,
    pub y: i32,
    pub h: u32,
    pub w: u32,
}

#[derive(Component, Debug)]
#[derive(Serialize, Deserialize)]
#[storage(VecStorage)]
pub struct Script {
    pub code:String,
    pub buffer:[i32;8]
}

#[derive(Component, Debug)]
#[storage(VecStorage)]
#[derive(Default)]
pub struct Statik {
    pub buffer:[i32;8]
}