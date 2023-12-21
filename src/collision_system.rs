use std::cell::RefCell;
use std::collections::hash_map::DefaultHasher;
use std::collections::HashMap;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::ptr;
use sdl2::sys::abs;
use specs::{Join, ReadStorage, System, WriteStorage};
use crate::components::{Collider, Position};

pub struct CollisionSystem;
fn calculate_hash<T: Hash>(t: &T) -> u64 {
    let mut s = DefaultHasher::new();
    t.hash(&mut s);
    s.finish()
}
impl<'a> System<'a> for CollisionSystem {
    type SystemData = (WriteStorage<'a, Collider>, ReadStorage<'a, Position>);
    fn run(&mut self, (mut colliders1, position): Self::SystemData) {
        let ct1 = (&mut colliders1, &position).join();
        let mut book_reviews = HashMap::new();
        for (collider1, position1) in (&colliders1, &position).join() {
            for (collider2, position2) in (&colliders1, &position).join() {
                if !ptr::eq(collider1, collider2)
                    && (position1.x - position2.x).abs()*2 <= (collider1.x+ collider2.x)
                    && (position1.y - position2.y).abs()*2 <= (collider1.y+ collider2.y)
                {
                    book_reviews.insert(calculate_hash(position1), true);
                } else {
                }
            }
        }

            for (collider1, position1) in (&mut colliders1, &position).join() {
                collider1.collision_end = false;
                if(!collider1.collision_start && !collider1.is_collision) {
                    collider1.collision_start = *book_reviews.get(&calculate_hash(position1)).unwrap_or(&false);
                    collider1.is_collision = collider1.collision_start;
                }
                else if(collider1.collision_start){
                    collider1.collision_start = false;
                    collider1.is_collision = true;
                }else if collider1.is_collision {
                    collider1.is_collision = *book_reviews.get(&calculate_hash(position1)).unwrap_or(&false);
                    if(!collider1.is_collision){
                        collider1.collision_end = true;
                    }
                }
            }

    }
}