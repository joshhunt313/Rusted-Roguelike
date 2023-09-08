use tcod::{Console, input};
use tcod::console::{Root, FontLayout};
use tcod::input::{Key, Mouse, Event, KeyCode};
use std::process::exit;

const CONSOLE_WIDTH: u32 = 120;
const CONSOLE_HEIGHT: u32 = 70;

fn main() {
    let mut root = Root::initializer()
        .size(CONSOLE_WIDTH as i32, CONSOLE_HEIGHT as i32)
        .title("Rusted Roguelike")
        .font("fonts/terminal_8x8.png", FontLayout::AsciiInCol)
        .renderer(tcod::Renderer::SDL)
        .init();

    while !root.window_closed() {
        // Clear Console
        root.clear();

        // Handling user input
        let event = input::check_for_event(input::MOUSE | input::KEY_PRESS);
        
        match event {
            Some((_, Event::Key(key_event))) => {
                // Handle key events
                match key_event {
                    Key { code: KeyCode::Escape, .. } => {
                        // Handle enter key press
                        println!("Escape key pressed");
                        exit(0);
                    }
                    // Handle other key events as needed
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
       
        // Updating the gamestate
        // Rendering the results
        

        root.flush();
    }
}