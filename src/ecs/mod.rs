use rand::Rng;
use specs::{World, WorldExt, Builder};
pub mod systems;
pub mod entities;
use entities::{Entity, Player, Position, Window};

pub fn gen_room(x: i32, y: i32, width: i32, height: i32) -> Vec<(Entity, Position)> {
    let mut tile_vec: Vec<(Entity, Position)> = vec![];
    for row in y..y+height {
        for col in x..x+width {
            if (col == x || col == (x + width - 1)) || (row == y || row == (y + height - 1)) {
                let tile = Entity { symbol: '#', passable: false, visable: true, is_wall: true };
                let pos = Position { x: col, y: row };
                tile_vec.push((tile, pos));
            }
            else {
                let tile = Entity { symbol: '.', passable: true, visable: true, is_wall: false };
                let pos = Position { x: col, y: row };
                tile_vec.push((tile, pos));
            }
        }
    }
    tile_vec
}

pub fn gen_dungeon(world: &mut World, map_width: i32, map_height: i32, num_rooms: i32) {
    let mut rng = rand::thread_rng();
    for _ in 0..num_rooms {
        let width = rng.gen_range(5..15);
        let height = rng.gen_range(5..10);
        let x = rng.gen_range(0..map_width - width);
        let y = rng.gen_range(0..map_height - height);

        // println!("{:?}: ({:?}, {:?}) -- {} {}", room_num, x, y, map_width - width, map_height - height);
        let room_tiles = gen_room(x, y, width, height);
        for tile in room_tiles {
            world.create_entity()
                .with(Entity { symbol: tile.0.symbol, passable: tile.0.passable, visable: tile.0.visable, is_wall: tile.0.is_wall })
                .with(Position { x: tile.1.x, y: tile.1.y })
                .build();
        }
    }
}