use specs::{Component, VecStorage, World, WorldExt, Builder, System, WriteStorage, ReadStorage, Join, DispatcherBuilder};
use tcod::{Console, input, Color};
use tcod::console::{Root, FontLayout};
use tcod::input::{Key, Mouse, Event, KeyCode};
use std::process::exit;

const CONSOLE_WIDTH: i32 = 90;
const CONSOLE_HEIGHT: i32 = 45;

#[derive(Component)]
#[storage(VecStorage)]
struct Position {
    x: i32,
    y: i32
}

#[derive(Component)]
#[storage(VecStorage)]
struct Player;

#[derive(Component)]
#[storage(VecStorage)]
struct Entity {
    symbol: char,
    passable: bool,
    visable: bool,
    is_wall: bool
}

#[derive(Component)]
#[storage(VecStorage)]
struct Window {
    root: Root,
}


struct MovePlayerUpSystem;
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

        for (pos, entity, player) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.y += 1;
            }
        }
    }
}

struct MovePlayerDownSystem;
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

        for (pos, entity, player) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.y -= 1;
            }
        }
    }
}

struct MovePlayerRightSystem;
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

        for (pos, entity, player) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.x -= 1;
            }
        }
    }
}

struct MovePlayerLeftSystem;
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

        for (pos, entity, player) in (&mut positions, &entities, &players).join() {
            if wall_vec.contains(&(pos.x, pos.y)) {
                pos.x += 1;
            }
        }
    }
}

struct RenderEntitiesSystem;
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

struct ClearRootSystem;
impl<'a> System<'a> for ClearRootSystem {
    type SystemData = WriteStorage<'a, Window>;

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.clear();
        }
    }
}

struct FlushRootSystem;
impl<'a> System<'a> for FlushRootSystem {
    type SystemData = WriteStorage<'a, Window>;

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.flush();
        }
    }
}

fn gen_room(x: i32, y: i32, width: i32, height: i32) -> Vec<(Entity, Position)> {
    let mut tile_vec: Vec<(Entity, Position)> = vec![];
    for row in y..=height {
        for col in x..=width {
            if (col == x || col == (x + width)) || (row == y || row == (y + height)) {
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

fn gen_dungeon(world: &mut World, map_width: i32, map_height: i32, num_rooms: i32) {
    let mut rng = rand::thread_rng();
    let room_tiles = gen_room(0, 0, 10, 5);
    for tile in room_tiles {
        world.create_entity()
            .with(Entity { symbol: tile.0.symbol, passable: tile.0.passable, visable: tile.0.visable, is_wall: tile.0.is_wall })
            .with(Position { x: tile.1.x, y: tile.1.y })
            .build();
    }
}

fn main() {
    // Initialize window
    let root = Root::initializer()
        .size(CONSOLE_WIDTH, CONSOLE_HEIGHT)
        .title("Rusted Roguelike")
        .font("fonts/terminal_12x12.png", FontLayout::AsciiInRow)
        .renderer(tcod::Renderer::SDL)
        .init();

    // Initialize player
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Entity>();
    world.register::<Player>();
    world.register::<Window>();

    world.create_entity()
        .with(Window { root: root })
        .build();

    world.create_entity()
        .with(Entity { symbol: '@', passable: false, visable: true, is_wall: false })
        .with(Position { x: 1, y: 1 })
        .with(Player)
        .build();

    gen_dungeon(&mut world, CONSOLE_WIDTH, CONSOLE_WIDTH, 1);

    loop {
        // Clear Console
        DispatcherBuilder::new()
            .with(ClearRootSystem, "clear_root", &[])
            .build()
            .dispatch(&mut world);

        // Handling user input
        let event = input::check_for_event(input::MOUSE | input::KEY_PRESS);

        let mut up_dispatcher = DispatcherBuilder::new()
            .with(MovePlayerUpSystem, "up", &[])
            .build();

        let mut down_dispatcher = DispatcherBuilder::new()
            .with(MovePlayerDownSystem, "down", &[])
            .build();

        let mut right_dispatcher = DispatcherBuilder::new()
            .with(MovePlayerRightSystem, "right", &[])
            .build();

        let mut left_dispatcher = DispatcherBuilder::new()
            .with(MovePlayerLeftSystem, "left", &[])
            .build();
        
        match event {
            Some((_, Event::Key(key_event))) => {
                // Handle key events
                match key_event {
                    Key { code: KeyCode::Escape, .. } => {
                        println!("Exiting game...");
                        exit(0);
                    }
                    
                    // Arrow Keys
                    Key { code: KeyCode::Up, .. } => {
                        up_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Down, .. } => {
                        down_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Right, .. } => {
                        right_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Left, .. } => {
                        left_dispatcher.dispatch(&mut world);
                    }

                    // Numbers
                    Key { code: KeyCode::Number8, .. } => {
                        up_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number9, .. } => {
                        up_dispatcher.dispatch(&mut world);
                        right_dispatcher.dispatch(&mut world)
                    }

                    Key { code: KeyCode::Number7, .. } => {
                        up_dispatcher.dispatch(&mut world);
                        left_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number6, .. } => {
                        right_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number4, .. } => {
                        left_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number2, .. } => {
                        down_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number3, .. } => {
                        down_dispatcher.dispatch(&mut world);
                        right_dispatcher.dispatch(&mut world);
                    }

                    Key { code: KeyCode::Number1, .. } => {
                        down_dispatcher.dispatch(&mut world);
                        left_dispatcher.dispatch(&mut world);
                    }

                    _ => {}
                }
            }
            Some((_, Event::Mouse(mouse_event))) => {
                // Handle mouse events
                match mouse_event {
                    Mouse { .. } => {
                        // Handle mouse input
                    }
                }
            }
            _ => {}
        }
       
        // Rendering the results
        DispatcherBuilder::new()
            .with(RenderEntitiesSystem, "render_entities", &[])
            .build()
            .dispatch(&mut world);



        DispatcherBuilder::new()
            .with(FlushRootSystem, "flush_root", &[])
            .build()
            .dispatch(&mut world);
    }
}