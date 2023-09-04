use crate::engine::components::*;
use specs::{System, ReadStorage, WriteStorage, Join, World, WorldExt};

pub struct PrintPlayerPosSystem;
impl<'a> System<'a> for PrintPlayerPosSystem {
    type SystemData = (ReadStorage<'a, Player>, WriteStorage<'a, Position>);

    fn run(&mut self, (player, mut pos): Self::SystemData) {
        for (_, pos) in (&player, &mut pos).join() {
            println!("Player Pos: ({:?}, {:?})", pos.x, pos.y);
        }
    }
}

pub fn move_player(new_x: i32, new_y: i32, ecs: &mut World) {
    let (mut positions, _) = (
        ecs.write_storage::<Position>(),
        ecs.read_storage::<Player>(),
    );
    for pos in (&mut positions).join() {
        pos.x = new_x;
        pos.y = new_y;
    }
}