mod engine;
use doryen_rs::{App, AppOptions};
use crate::engine::Game;

const CONSOLE_WIDTH: u32 = 90;
const CONSOLE_HEIGHT: u32 = 45;

fn main() {
    // here are all the available options.
    // better practise is to use default values (see other examples)
    let mut app = App::new(AppOptions {
        console_width: CONSOLE_WIDTH,
        console_height: CONSOLE_HEIGHT,
        screen_width: CONSOLE_WIDTH * 12,
        screen_height: CONSOLE_HEIGHT * 12,
        window_title: "Rusted Roguelike".to_owned(),
        font_path: "Buddy--graphical_10x10.png".to_owned(),
        vsync: true,
        fullscreen: false,
        show_cursor: true,
        resizable: true,
        intercept_close_request: false,
        max_fps: 0,
    });
    app.set_engine(Box::new(Game::new(CONSOLE_HEIGHT, CONSOLE_WIDTH)));
    app.run();
}