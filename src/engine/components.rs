use doryen_rs::DoryenApi;
use specs::{prelude::*, Component};
use rand::Rng;

#[derive(Component, Debug)]
#[storage(VecStorage)]
pub struct Tile {
    pub visual: u16,
    pub passable: bool,
}

pub struct Room {
    width: i8,
    height: i8,
    x: i32,
    y: i32
}

fn gen_room (x: i32, y: i32) -> Room {
    let mut rng = rand::thread_rng();
    Room { width: rng.gen_range(5..=15), height: rng.gen_range(5..=10), x: x, y: y }
}

// fn place_room (room: Room, map: Vec<Vec<Tile>>) -> Vec<Vec<Tile>> {
    
// }

pub fn generate_map (map_width: u32, map_height: u32, ecs: &mut World) {
    // let map: Vec<Vec<Tile>> = Vec::with_capacity(map_height as usize);
    for (i, row) in (0..map_height).enumerate() {
        // let row_vec: Vec<Tile> = Vec::new();
        for (j, col) in (0..map_width).enumerate() {
            let _ = ecs.create_entity().with(Tile { visual: '#' as u16, passable: false }).with(Position {x: j as i32, y: i as i32});
        }
    }
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

pub fn move_player (new_x: i32, new_y: i32, ecs: &mut World) {
    let (mut positions, _) = (
        ecs.write_storage::<Position>(),
        ecs.read_storage::<Player>(),
    );
    for pos in (&mut positions).join() {
        pos.x = new_x;
        pos.y = new_y;
    }
}