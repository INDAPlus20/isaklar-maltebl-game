use crate::game_data::{Player};
pub struct Game {
    players: [Player; 2]
}

impl Game {
    pub fn new() -> Game {
        Game {
            players: [Player::new(), Player::new()]
        }
    }
}