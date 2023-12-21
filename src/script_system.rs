use std::cell::RefCell;
use std::ops::Deref;
use std::rc::Rc;
use rhai::{Array, Dynamic, Engine, Scope};
use specs::{System, WriteStorage, Join, ReadStorage, Entities, Entity, Read, World, WorldExt, Write, ReadExpect, LendJoin};
use crate::components::{Collider, Position, Script, Statik};
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
        WriteStorage<'a, Script>,
        WriteStorage<'a, Position>,
        Read<'a, InputComponent>,
        Write<'a, Statik>,
        ReadStorage<'a, Collider>
    );

    fn run(&mut self, (e, mut s, mut p, i, mut statik, c): Self::SystemData) {
        let mut scope = Scope::new();

        for (ent, pos, scr, colid) in (&*e, &mut p, &mut s, (&c).maybe()).join() {
            let mut engine = Engine::new();
            scope.push("buffer",  Dynamic::from_iter(scr.buffer.clone()));
            scope.push("statik_buffer",  Dynamic::from_iter(statik.buffer.clone()));
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

            let p = clone_pose.clone();
            engine.register_fn("get_pos", move || {
                Dynamic::from_iter([p.borrow().x, p.borrow().y])
            });
            let inp = i.clone();
            engine.register_fn("get_dir_wasd", move || {
                Dynamic::from_iter(inp.get_direction_wasd())
            });
            let inp = i.clone();
            engine.register_fn("get_dir_arrows", move || {
                Dynamic::from_iter(inp.get_direction_arrows())
            });
            let inp = i.clone();
            engine.register_fn("get_key_down", move |key: &str| {
                inp.get_key_down(&key)
            });

            let inp = i.clone();
            engine.register_fn("get_key_up", move |key: &str| {
                inp.get_key_up(&key)
            });

            if colid.is_some() {
                let col = colid.unwrap().clone();
                engine.register_fn("is_collision", move || {
                    col.is_collision
                });
                let col = colid.unwrap().clone();
                engine.register_fn("collision_start", move || {
                    col.collision_start
                });
                let col = colid.unwrap().clone();
                engine.register_fn("collision_end", move || {
                    col.collision_end
                });
            }

            let mut buf1 = Rc::new(RefCell::new(scr.buffer.clone()));
            let mut b = buf1.clone();
            engine.register_fn("save_buff", move | arr: Array|{
                for i in 0..arr.len(){
                    b.borrow_mut()[i] = arr[i].clone().cast();
                }
            });

            let mut buf1_statik = Rc::new(RefCell::new(statik.buffer.clone()));
            let mut b_statik = buf1_statik.clone();
            engine.register_fn("save_statik_buff", move | arr: Array|{
                for i in 0..arr.len(){
                    b_statik.borrow_mut()[i] = arr[i].clone().cast();
                }
            });

            let ast = engine.compile_file(scr.code.parse().unwrap());
            match ast {
                Ok(a) => {
                    let a = engine.call_fn::<()>(&mut scope, &a, "update", ());
                    match a {
                        Ok(_) => {}
                        Err(err) => { println!("{:?}{}", err, scr.code) }
                    }
                    Position::clone_from(pos, clone_pose.borrow().deref());
                    let b1 = buf1.take();
                    let bs = buf1_statik.take();
                    for i in 0..8{
                        scr.buffer[i] = b1[i].clone();
                        statik.buffer[i] = bs[i].clone();
                    }
                }
                Err(err) => { println!("{:?}{}", err, scr.code) }
            }
        }
    }
}