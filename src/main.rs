use specs::{World, WorldExt, Builder, DispatcherBuilder};
use tcod::input;
use tcod::console::{Root, FontLayout};
use tcod::input::{Key, Mouse, Event, KeyCode};
use std::process::exit;
mod ecs;
use ecs::gen_dungeon;
use ecs::entities::{Position, Entity, Player, Window};
use ecs::systems::{RenderEntitiesSystem, FlushRootSystem, ClearRootSystem, MovePlayerUpSystem, MovePlayerDownSystem, MovePlayerLeftSystem, MovePlayerRightSystem};

const CONSOLE_WIDTH: i32 = 90;
const CONSOLE_HEIGHT: i32 = 45;
const NUM_ROOMS: i32 = 9;



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

    gen_dungeon(&mut world, CONSOLE_WIDTH, CONSOLE_HEIGHT, NUM_ROOMS);

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