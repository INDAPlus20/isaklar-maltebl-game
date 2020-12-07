use crate::game_data::{Player, COLS, ROWS};
use ggez::event::KeyCode;
pub struct Game {
    players: [Player; 2],
}

impl Game {
    pub fn new() -> Game {
        Game {
    pub fn update(&mut self) {
        // update game tick for players
        for p in 0..self.players.len() {
            //self.players[p].time_tick();
            self.players[p].move_tick();
        }
    }

    pub fn get_boards(&mut self) -> [[[u32; COLS]; ROWS]; 2] {
        [self.players[0].get_board(), self.players[1].get_board()]
    }
    pub fn key_down(&mut self, key: KeyCode) {
        match key {
            // P1 controlls
            KeyCode::A => self.players[0].move_current(-1, 0),
            KeyCode::W => self.players[0].rotate_current(true),
            KeyCode::D => self.players[0].move_current(1, 0),
            KeyCode::S => self.players[0].rotate_current(false),
            KeyCode::Space => self.players[0].move_current(0, -1),
            KeyCode::E => self.players[0].save_piece(),
            // P2 controlls
            KeyCode::Left => self.players[1].move_current(-1, 0),
            KeyCode::Up => self.players[1].rotate_current(true),
            KeyCode::Right => self.players[1].move_current(1, 0),
            KeyCode::Down => self.players[1].rotate_current(false),
            KeyCode::NumpadEnter => self.players[1].move_current(0, -1),
            KeyCode::RShift => self.players[1].save_piece(),
            _ => (),
        }
    }
}