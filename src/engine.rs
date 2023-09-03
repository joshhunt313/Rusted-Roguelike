use doryen_rs::{DoryenApi, Engine, UpdateEvent};

pub struct Game {
    player_pos: (i32, i32),
    mouse_pos: (f32, f32),
    console_height: u32,
    console_width: u32
}

impl Game {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            player_pos: ((width / 2) as i32, (height / 2) as i32),
            mouse_pos: (0.0, 0.0),
            console_height: height,
            console_width: width
        }
    }
}

impl Engine for Game {

    fn init(&mut self, api: &mut dyn DoryenApi) {
        api.con().register_color("white", (255, 255, 255, 255));
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {

        let input = api.input();
        // Left and Right
        if input.key_pressed("Digit4") || input.key_pressed("ArrowLeft") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
        } 
        else if input.key_pressed("Digit6") || input.key_pressed("ArrowRight") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(self.console_width as i32 - 2);
        }

        // Up and Down
        if input.key_pressed("Digit8") || input.key_pressed("ArrowUp") {
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        } 
        else if input.key_pressed("Digit2") || input.key_pressed("ArrowDown") {
            self.player_pos.1 = (self.player_pos.1 + 1).min(self.console_height as i32 - 2);
        }

        // Diagonals
        if input.key_pressed("Digit7") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        }
        else if input.key_pressed("Digit9") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(self.console_width as i32 - 2);
            self.player_pos.1 = (self.player_pos.1 - 1).max(1);
        }
        else if input.key_pressed("Digit1") {
            self.player_pos.0 = (self.player_pos.0 - 1).max(1);
            self.player_pos.1 = (self.player_pos.1 + 1).min(self.console_height as i32 - 2);
        }
        else if input.key_pressed("Digit3") {
            self.player_pos.0 = (self.player_pos.0 + 1).min(self.console_width as i32 - 2);
            self.player_pos.1 = (self.player_pos.1 + 1).min(self.console_height as i32 - 2);
        }

        self.mouse_pos = input.mouse_pos();

        None
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();
        con.rectangle(
            0,
            0,
            self.console_width,
            self.console_height,
            Some((128, 128, 128, 255)),
            Some((0, 0, 0, 255)),
            Some('.' as u16),
        );
        
        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        con.back(
            self.mouse_pos.0 as i32,
            self.mouse_pos.1 as i32,
            (255, 255, 255, 255),
        );
    }
}