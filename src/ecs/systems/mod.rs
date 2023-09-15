use specs::{System, WriteStorage, ReadStorage, Join};
use tcod::{Color, Console};
use super::{Position, Player, Entity, Window};

pub struct MovePlayerUpSystem;
impl<'a> System<'a> for MovePlayerUpSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>, ReadStorage<'a, Entity>);

    fn run(&mut self, (mut positions, players, entities): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.y -= 1;
            println!("({:?}, {:?})", pos.x, pos.y);
        }

        let mut wall_vec: Vec<(i32, i32)> = vec![];
        for (pos, entity) in (&mut positions, &entities).join() {
            if entity.is_wall {
                wall_vec.push((pos.x, pos.y));
            }
        }

        for (pos, _, _) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.y += 1;
            }
        }
    }
}

pub struct MovePlayerDownSystem;
impl<'a> System<'a> for MovePlayerDownSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>, ReadStorage<'a, Entity>);

    fn run(&mut self, (mut positions, players, entities): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.y += 1;
            println!("({:?}, {:?})", pos.x, pos.y);
        }

        let mut wall_vec: Vec<(i32, i32)> = vec![];
        for (pos, entity) in (&mut positions, &entities).join() {
            if entity.is_wall {
                wall_vec.push((pos.x, pos.y));
            }
        }

        for (pos, _, _) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.y -= 1;
            }
        }
    }
}

pub struct MovePlayerRightSystem;
impl<'a> System<'a> for MovePlayerRightSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>, ReadStorage<'a, Entity>);

    fn run(&mut self, (mut positions, players, entities): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.x += 1;
            println!("({:?}, {:?})", pos.x, pos.y);
        }

        let mut wall_vec: Vec<(i32, i32)> = vec![];
        for (pos, entity) in (&mut positions, &entities).join() {
            if entity.is_wall {
                wall_vec.push((pos.x, pos.y));
            }
        }

        for (pos, _, _) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.x -= 1;
            }
        }
    }
}

pub struct MovePlayerLeftSystem;
impl<'a> System<'a> for MovePlayerLeftSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>, ReadStorage<'a, Entity>);

    fn run(&mut self, (mut positions, players, entities): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.x -= 1;
            println!("({:?}, {:?})", pos.x, pos.y);
        }

        let mut wall_vec: Vec<(i32, i32)> = vec![];
        for (pos, entity) in (&mut positions, &entities).join() {
            if entity.is_wall {
                wall_vec.push((pos.x, pos.y));
            }
        }

        for (pos, _, _) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.x += 1;
            }
        }
    }
}

pub struct RenderEntitiesSystem;
impl<'a> System<'a> for RenderEntitiesSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Entity>, ReadStorage<'a, Player>, WriteStorage<'a, Window>);

    fn run(&mut self, (positions, entities, players, mut window): Self::SystemData) {
        for (pos, entity) in (&positions, &entities).join() {
            // println!("{:?}", (pos.x, pos.y));
            for wind in (&mut window).join() {
                wind.root.put_char_ex(pos.x, pos.y, entity.symbol, Color { r: 170, b: 170, g: 170 }, Color { r: 0, b: 0, g: 0});
            }
        }

        for (pos, _player, entity) in (&positions, &players, &entities).join() {
            for wind in (&mut window).join() {
                wind.root.put_char_ex(pos.x, pos.y, entity.symbol, Color { r: 255, b: 255, g: 255 }, Color { r: 0, b: 0, g: 0});
            }
        }
    }
}

pub struct ClearRootSystem;
impl<'a> System<'a> for ClearRootSystem {
    type SystemData = WriteStorage<'a, Window>;

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.clear();
        }
    }
}

pub struct FlushRootSystem;
impl<'a> System<'a> for FlushRootSystem {
    type SystemData = WriteStorage<'a, Window>;

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.flush();
        }
    }
}