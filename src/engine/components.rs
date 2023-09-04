use specs::{prelude::*, Component};
use rand::Rng;


struct Tile {
    visual: u16,
    passable: bool,
}

pub struct Room {
    width: i8,
    height: i8,
    x: i32,
    y: i32
}

pub fn gen_room(width: i8, height: i8) -> Room {
    let mut rng = rand::thread_rng();
    let room = Room { width: width, height: height, x: rng.gen_range(0..=90), y: rng.gen_range(0..=50) };
    room
}


#[derive(Component)]
#[storage(VecStorage)]
struct Map {
    tiles: Vec<Vec<Tile>>,
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32,
}


#[derive(Component)]
#[storage(VecStorage)]
pub struct Player {}

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