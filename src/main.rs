use specs::{Component, VecStorage, World, WorldExt, Builder, System, WriteStorage, ReadStorage, Join, DispatcherBuilder, ReadExpect};
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
    visable: bool
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
        for (pos, _) in (&mut positions, &players).join() {
            pos.y -= 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerDownSystem;
impl<'a> System<'a> for MovePlayerDownSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.y += 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerRightSystem;
impl<'a> System<'a> for MovePlayerRightSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
            pos.x += 1;
            println!("({:?}, {:?})", pos.x, pos.y)
        }
    }
}

struct MovePlayerLeftSystem;
impl<'a> System<'a> for MovePlayerLeftSystem {
    type SystemData = (WriteStorage<'a, Position>, ReadStorage<'a, Player>);

    fn run(&mut self, (mut positions, players): Self::SystemData) {
        for (pos, _) in (&mut positions, &players).join() {
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

    // let tcod = Window { root: root };
    world.create_entity()
        .with(Window {root: root})
        .build();

    world.create_entity()
        .with(Entity { symbol: '@', passable: false, visable: true })
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