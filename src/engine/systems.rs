use std::io::Read;

use crate::engine::components::*;
use specs::{System, ReadStorage, WriteStorage, Join};

pub struct PrintPlayerPosSystem;
impl<'a> System<'a> for PrintPlayerPosSystem {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Position>);

    fn run(&mut self, (player, mut pos): Self::SystemData) {
        for (_, pos) in (&player, &mut pos).join() {
            println!("Player Pos: ({:?}, {:?})", pos.x, pos.y);
        }
    }
}
