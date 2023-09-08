use specs::{Component, VecStorage, World, WorldExt, Builder, System, WriteStorage, ReadStorage, Join, DispatcherBuilder, ReadExpect};
use tcod::{Console, input, Renderer, Color};
use tcod::console::{Root, FontLayout};
use tcod::input::{Key, Mouse, Event, KeyCode};
use std::process::exit;

const CONSOLE_WIDTH: u32 = 120;
const CONSOLE_HEIGHT: u32 = 70;

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
    passable: bool
}

#[derive(Component)]
#[storage(VecStorage)]
struct Window {
    root: Root
}

struct MovePlayerUpSystem;
impl<'a> System<'a> for MovePlayerUpSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, player) in (&mut positions, &players).join() {
            pos.y -= 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerDownSystem;
impl<'a> System<'a> for MovePlayerDownSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, player) in (&mut positions, &players).join() {
            pos.y += 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerRightSystem;
impl<'a> System<'a> for MovePlayerRightSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, player) in (&mut positions, &players).join() {
            pos.x += 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerLeftSystem;
impl<'a> System<'a> for MovePlayerLeftSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, player) in (&mut positions, &players).join() {
            pos.x -= 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct RenderEntitiesSystem;
impl<'a> System<'a> for RenderEntitiesSystem {
    type SystemData = (ReadStorage<'a, Position>, ReadStorage<'a, Entity>, WriteStorage<'a, Window>);

    fn run(&mut self, (positions, players, mut window): Self::SystemData) {
        for (pos, entity) in (&positions, &players).join() {
            println!("{:?}", (pos.x, pos.y));
            for wind in (&mut window).join() {
                wind.root.put_char_ex(pos.x, pos.y, entity.symbol, Color { r: 255, b: 255, g: 255 }, Color { r: 0, b: 0, g: 0});
            }
        }
    }
}

struct ClearRootSystem;
impl<'a> System<'a> for ClearRootSystem {
    type SystemData = (WriteStorage<'a, Window>);

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.clear();
        }
    }
}

struct FlushRootSystem;
impl<'a> System<'a> for FlushRootSystem {
    type SystemData = (WriteStorage<'a, Window>);

    fn run(&mut self, mut root: Self::SystemData) {
        for console in (&mut root).join() {
            console.root.flush();
        }
    }
}

fn main() {
    // Initialize window
    let mut root = Root::initializer()
        .size(CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32)
        .title("Rusted Roguelike")
        .font("fonts/terminal_8x8.png", FontLayout::AsciiInCol)
        .renderer(tcod::Renderer::SDL)
        .init();

    // Initialize player
    let mut world = World::new();
    world.register::<Position>();
    world.register::<Entity>();
    world.register::<Player>();
    world.register::<Window>();

    // let tcod = Window { root: root };
    world.create_entity()
        .with(Window {root: root})
        .build();

    world.create_entity()
        .with(Entity { symbol: 16 as char, passable: false })
        .with(Position { x: 1, y: 1 })
        .with(Player)
        .build();

    loop {
        // Clear Console
        DispatcherBuilder::new()
            .with(ClearRootSystem, "clear_root", &[])
            .build()
            .dispatch(&mut world);

        // Handling user input
        let event = input::check_for_event(input::MOUSE | input::KEY_PRESS);
        
        match event {
            Some((_, Event::Key(key_event))) => {
                // Handle key events
                match key_event {
                    Key { code: KeyCode::Escape, .. } => {
                        println!("Exiting game...");
                        exit(0);
                    }
                    
                    Key { code: KeyCode::Up, .. } => {
                        let dispatcher = DispatcherBuilder::new()
                            .with(MovePlayerUpSystem, "up", &[])
                            .build()
                            .dispatch(&mut world);
                    }

                    Key { code: KeyCode::Down, .. } => {
                        let dispatcher = DispatcherBuilder::new()
                            .with(MovePlayerDownSystem, "down", &[])
                            .build()
                            .dispatch(&mut world);
                    }

                    Key { code: KeyCode::Right, .. } => {
                        let dispatcher = DispatcherBuilder::new()
                            .with(MovePlayerRightSystem, "right", &[])
                            .build()
                            .dispatch(&mut world);
                    }

                    Key { code: KeyCode::Left, .. } => {
                        let dispatcher = DispatcherBuilder::new()
                            .with(MovePlayerLeftSystem, "left", &[])
                            .build()
                            .dispatch(&mut world);
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