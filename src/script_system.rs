use std::cell::RefCell;
use std::fs::ReadDir;
use std::iter::Copied;
use std::ops::Deref;
use std::rc::Rc;
use std::sync::Arc;
use rhai::{Array, Dynamic, Engine, Scope};
use sdl2::keyboard::Keycode;
use specs::{System, WriteStorage, Join, ReadStorage, Entities, Entity, Read, World, WorldExt, Write, ReadExpect};
use crate::components::{Position, Script};
use crate::input_manager::InputComponent;

pub type SharedPosition = Rc<RefCell<Position>>;
pub type SharedInput = Rc<RefCell<InputComponent>>;

pub struct ScriptSystem {}

impl ScriptSystem {}

#[cfg(not(feature = "no_index"))]
#[cfg(not(feature = "no_object"))]
impl<'a> System<'a> for ScriptSystem {
    type SystemData = (
        Entities<'a>,
        ReadStorage<'a, Script>,
        WriteStorage<'a, Position>,
        Read<'a, InputComponent>
    );

    fn run(&mut self, (e, s, mut p, i): Self::SystemData) {
        let mut scope = Scope::new();
        for (ent, pos, scr) in (&*e, &mut p, &s).join() {
            let mut engine = Engine::new();

            let clone_pose = Rc::new(RefCell::new(pos.clone()));

            let p = clone_pose.clone();
            engine.register_fn("set_pos", move |x: i32, y: i32| {
                p.borrow_mut().x = x;
                p.borrow_mut().y = y;
            });
            let p = clone_pose.clone();
            engine.register_fn("add_pos", move |x: i32, y: i32| {
                p.borrow_mut().x += x;
                p.borrow_mut().y += y;
            });
            let inp = i.clone();
            engine.register_fn("get_dir", move|| {
                Dynamic::from_iter(inp.get_direction())
            });
            let inp = i.clone();
            engine.register_fn("get_key_down", move|key:&str| {
                inp.get_key_down(&key)
            });

            let inp = i.clone();
            engine.register_fn("get_key_up", move|key:&str| {
                inp.get_key_up(&key)
            });

            let ast = engine.compile_file(scr.code.parse().unwrap());
            match ast {
                Ok(a) => {
                    let a = engine.call_fn::<()>(&mut scope, &a, "update", ());
                    Position::clone_from(pos, clone_pose.borrow().deref());
                }
                Err(err) => { println!("{:?}", err) }
            }
        }
    }
}