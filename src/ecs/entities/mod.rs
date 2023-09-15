use specs::{Component, VecStorage};
use tcod::console::Root;


#[derive(Component)]
#[storage(VecStorage)]
pub struct Position {
    pub x: i32,
    pub y: i32
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Player;

#[derive(Component)]
#[storage(VecStorage)]
pub struct Entity {
    pub symbol: char,
    pub passable: bool,
    pub visable: bool,
    pub is_wall: bool
}

#[derive(Component)]
#[storage(VecStorage)]
pub struct Window {
    pub root: Root,
}