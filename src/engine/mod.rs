mod components;
mod systems;

use doryen_rs::{DoryenApi, Engine, UpdateEvent};
use specs::{prelude::*};
use components::{Player, move_player, Position, generate_map, Tile};
use systems::{PrintPlayerPosSystem};

pub struct Game {
    player_pos: (i32, i32),
    mouse_pos: (f32, f32),
    console_height: u32,
    console_width: u32,
    ecs: World
}

impl Game {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            player_pos: ((width / 2) as i32, (height / 2) as i32),
            mouse_pos: (0.0, 0.0),
            console_height: height,
            console_width: width,
            ecs: World::new()
        }
    }
}

impl Engine for Game {

    fn init(&mut self, api: &mut dyn DoryenApi) {
        api.con().register_color("white", (255, 255, 255, 255));
        api.con().register_color("red", (255, 92, 92, 255));
        api.con().register_color("blue", (192, 192, 255, 255));

        self.ecs.register::<Player>();
        self.ecs.register::<Position>();
        self.ecs.register::<Tile>();

        // Create player entity
        let _ = self.ecs.create_entity().with(Player{}).with(Position{ x: self.player_pos.0, y: self.player_pos.1 });
        // let _ = self.ecs.create_entity().with(Map{ })
        let _ = generate_map(self.console_width, self.console_height, &mut self.ecs);
    }

    fn update(&mut self, api: &mut dyn DoryenApi) -> Option<UpdateEvent> {

        let input = api.input();
        // Left and Right
        if input.key_pressed("Digit4") || input.key_pressed("ArrowLeft") {
            self.player_pos.0 = self.player_pos.0 - 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);

        } 
        else if input.key_pressed("Digit6") || input.key_pressed("ArrowRight") {
            self.player_pos.0 = self.player_pos.0 + 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }

        // Up and Down
        if input.key_pressed("Digit8") || input.key_pressed("ArrowUp") {
            self.player_pos.1 = self.player_pos.1 - 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        } 
        else if input.key_pressed("Digit2") || input.key_pressed("ArrowDown") {
            self.player_pos.1 = self.player_pos.1 + 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }

        // Diagonals
        if input.key_pressed("Digit7") {
            self.player_pos.0 = self.player_pos.0 - 1;
            self.player_pos.1 = self.player_pos.1 - 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }
        else if input.key_pressed("Digit9") {
            self.player_pos.0 = self.player_pos.0 + 1;
            self.player_pos.1 = self.player_pos.1 - 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }
        else if input.key_pressed("Digit1") {
            self.player_pos.0 = self.player_pos.0 - 1;
            self.player_pos.1 = self.player_pos.1 + 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }
        else if input.key_pressed("Digit3") {
            self.player_pos.0 = self.player_pos.0 + 1;
            self.player_pos.1 = self.player_pos.1 + 1;
            move_player(self.player_pos.0, self.player_pos.1, &mut self.ecs);
        }

        if input.key_pressed("Escape") {
            std::process::exit(0);
        }

        self.mouse_pos = input.mouse_pos();

        let mut dispatcher = DispatcherBuilder::new()
            .with(PrintPlayerPosSystem, "print_pos", &[])
            .build();

        dispatcher.dispatch(&self.ecs);

        None
    }

    fn render(&mut self, api: &mut dyn DoryenApi) {
        let con = api.con();

        // con.area(
        //     0,
        //     0,
        //     self.console_width,
        //     self.console_height,
        //     Some((128, 128, 128, 255)),
        //     Some((0, 0, 0, 255)),
        //     Some('.' as u16),
        // );

        let (positions, tiles) = (self.ecs.read_storage::<Position>(), self.ecs.read_storage::<Tile>());

        for (pos, tile) in (&positions, &tiles).join() {
            con.ascii(pos.x, pos.y, tile.visual);
            con.fore(pos.x, pos.y, (128, 128, 128, 255));
        }
        
        con.ascii(self.player_pos.0, self.player_pos.1, '@' as u16);
        con.fore(self.player_pos.0, self.player_pos.1, (255, 255, 255, 255));
        
        // con.back(
        //     self.mouse_pos.0 as i32,
        //     self.mouse_pos.1 as i32,
        //     (255, 255, 255, 255),
        // );
    }
}
