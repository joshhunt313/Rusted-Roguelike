use specs::{prelude::*, Component};

#[derive(Component)]
struct Tile {
    visual: u16,
    passable: bool,
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
