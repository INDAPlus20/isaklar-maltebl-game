use crate::game_data::{self, Player, COLS, ROWS};
use ggez::event::KeyCode;
use std::env;

use libloading::{Library, Symbol};

pub const PLAYER_AMOUNT: usize = 2;

type AIFunc = unsafe fn([[u32; COLS]; ROWS]) -> u32;

#[cfg(test)]
mod tests;

pub struct Game {
    players: [Player; PLAYER_AMOUNT],
    ai_lib: Option<Library>,
}

impl Game {
    pub fn new(init_level: usize) -> Game {
        let library: Option<Library>;
        if let Some(lib_path) = env::args().nth(1) {
            if cfg!(windows) && !lib_path.ends_with(".dll") {
                panic!("Must use .dll if running an AI in windows!")
            } else if cfg!(unix) && !lib_path.ends_with(".so") {
                panic!("Must use .so if running an AI in a 'nix-system!")
            }
            if let Ok(lib) = Library::new(lib_path) {
                library = Some(lib);
            } else {
                library = None;
            }
        } else {
            library = None;
        }

        Game {
            players: [Player::new(init_level), Player::new(init_level)],
            ai_lib: library,
        }
    }

    pub fn update(&mut self) {
        // update game tick for players
        let mut target_mod: i32 = 1; //Pairs, you attack the one next to you
        for p in 0..self.players.len() {
            self.players[p].update();
            if p < PLAYER_AMOUNT {
                //attack handling
                if let Some(attack) = self.players[p].take_outgoing() {
                    self.players[(p as i32 + target_mod) as usize].add_incoming(attack);
                }
            }
            target_mod *= -1;
        }

        if let Some(_) = self.ai_lib {
            let ai_output = self.call_ai_script();
            self.parse_ai_output(ai_output);
        }
    }

    pub fn get_boards(&self) -> [[[u32; COLS]; ROWS]; PLAYER_AMOUNT] {
        [self.players[0].get_board(), self.players[1].get_board()]
    }

    pub fn get_next_pieces(&self) -> [[[u32; 4]; 4]; PLAYER_AMOUNT] {
        let mut next_pieces = [[[0; 4]; 4]; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            if p < PLAYER_AMOUNT {
                next_pieces[p] = self.players[p].get_next_piece().get_display_shape();
            }
        }
        next_pieces
    }

    pub fn get_saved_pieces(&self) -> [[[u32; 4]; 4]; PLAYER_AMOUNT] {
        let mut saved_pieces = [[[0; 4]; 4]; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            if p < PLAYER_AMOUNT {
                if let Some(piece) = self.players[p].get_saved_piece() {
                    saved_pieces[p] = piece.get_display_shape();
                }
            }
        }
        saved_pieces
    }

    pub fn get_attackbars(&self) -> [u32; PLAYER_AMOUNT] {
        let mut attackbars = [0; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            if p < PLAYER_AMOUNT {
                for (attack, _) in self.players[p].get_incoming() {
                    attackbars[p] += *attack as u32;
                }
            }
        }
        attackbars
    }

    pub fn get_scores(&self) -> [u32; PLAYER_AMOUNT] {
        let mut scores = [0; PLAYER_AMOUNT];
        for p in 0..self.players.len() {
            if p < PLAYER_AMOUNT {
                scores[p] = self.players[p].get_score() as u32;
            }
        }
        scores
    }

    //Only set for 2 players
    pub fn key_down(&mut self, key: KeyCode) {
        match key {
            // P1 controlls
            KeyCode::A => self.players[0].move_current(-1, 0),
            KeyCode::E => self.players[0].rotate_current(true),
            KeyCode::D => self.players[0].move_current(1, 0),
            KeyCode::Q => self.players[0].rotate_current(false),
            KeyCode::S => self.players[0].move_current(0, -1),
            KeyCode::W => self.players[0].drop_current(),
            KeyCode::Space => self.players[0].save_piece(),
            // P2 controlls
            KeyCode::J => self.players[1].move_current(-1, 0),
            KeyCode::O => self.players[1].rotate_current(true),
            KeyCode::L => self.players[1].move_current(1, 0),
            KeyCode::U => self.players[1].rotate_current(false),
            KeyCode::K => self.players[1].move_current(0, -1),
            KeyCode::RShift => self.players[1].save_piece(),
            KeyCode::I => self.players[1].drop_current(),
            _ => (),
        }
    }

    fn call_ai_script(&mut self) -> u32 {
        let mut output = 0;

        unsafe {
            if let Some(lib) = &self.ai_lib {
                let func: Symbol<AIFunc> = lib.get(b"ai").expect("Couldn't find ai function");
                output = func(self.get_boards()[1]);
            }
        }
        output
    }

    fn parse_ai_output(&mut self, output: u32) {
        match output {
            1 => self.players[1].move_current(-1, 0),
            2 => self.players[1].rotate_current(true),
            3 => self.players[1].move_current(1, 0),
            4 => self.players[1].rotate_current(false),
            5 => self.players[1].move_current(0, -1),
            6 => self.players[1].save_piece(),
            7 => self.players[1].drop_current(),
            _ => (),
        }
    }
}
