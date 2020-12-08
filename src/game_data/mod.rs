use rand::Rng;
use std::time::Instant;

#[cfg(test)]
mod tests;

#[derive(Copy, Clone)]
enum Color {
    Void = 0,
    Color1 = 1,
    Color2 = 2,
    Color3 = 3,
    Color4 = 4,
    Color5 = 5,
    Color6 = 6,
    Color7 = 7,
    Fixed = 8,
}

type Point = [i32; 2];
type Shape = [Point; 4];

pub const ATTACK_DELAY: u8 = 6; //Osäker på denna. nu processeras även attacks med move_tick.

pub const ROWS: usize = 24;
pub const COLS: usize = 10;

pub const SHAPES: [Shape; 7] = [
    //I
    [[-2, 0], [-1, 0], [0, 0], [1, 0]],
    //O
    [[-1, 0], [-1, -1], [0, -1], [0, 0]],
    //T
    [[-1, 0], [0, 0], [0, -1], [1, 0]],
    //S
    [[-1, 0], [0, 0], [0, -1], [1, -1]],
    //Z
    [[-1, -1], [0, -1], [0, 0], [1, 0]],
    //J
    [[-1, -1], [-1, 0], [0, 0], [1, 0]],
    //L
    [[-1, 0], [0, 0], [1, 0], [1, -1]],
];

pub const TIME_LEVELS: [f64; 20] = [
    1.0, 0.79300, 0.61780, 0.47273, 0.35520, 0.26200, 0.18968, 0.13473, 0.09388, 0.06415, 0.04298,
    0.02822, 0.01815, 0.01144, 0.00706, 0.00426, 0.00252, 0.00146, 0.00082, 0.00046,
];

pub struct Player {
    board: [[u32; COLS]; ROWS],
    incoming: Vec<(u8, u8)>,
    outgoing: Option<(u8, u8)>,
    current_piece: Piece,
    saved_piece: Option<Piece>,
    next_piece: Piece,
    score: usize,
    lost: bool,
    gravity: f64,
    update_timer: Instant,
}

impl Player {
    pub fn new() -> Player {
        Player {
            board: [[0; COLS]; ROWS],
            incoming: Vec::new(),
            outgoing: None,
            current_piece: Piece::random_piece(),
            saved_piece: None,
            next_piece: Piece::random_piece(),
            score: 0,
            lost: false,
            gravity: TIME_LEVELS[0],
            update_timer: Instant::now(),
        }
    }

    pub fn update(&mut self) {
        if !self.lost && self.update_timer.elapsed().as_secs_f64() >= self.gravity {
            self.process_attacks();
            self.move_tick();
            self.update_timer = Instant::now();
        }
    }

    pub fn move_tick(&mut self) {
        if !self.lost {
            self.current_piece.mov(0, -1);
            if !self.valid_pos(&self.current_piece) {
                self.current_piece.mov(0, 1);
                self.place_piece(None);
                self.process_lines();
                self.next_piece();
            }
        }
    }

    fn process_lines(&mut self) {
        let mut full_rows: Vec<usize> = Vec::new();
        for i in 0..self.board.len() {
            if !self.board[i].contains(&0) {
                full_rows.push(i);
            }
        }
        if !full_rows.is_empty() {
            let mut board = [[0; COLS]; ROWS];
            for (r, row) in board.iter_mut().enumerate() {
                if r >= ROWS {
                    break;
                }
                if !full_rows.contains(&r) {
                    *row = self.board[r];
                }
            }
            self.process_score(full_rows.len());
            self.board = board;
        }
    }

    fn process_attacks(&mut self) {
        let mut rows = 0;
        for (attack, count) in &mut self.incoming {
            *count -= 1;
            if *count == 0 {
                rows += *attack;
            }
        }
        if rows > 0 {
            let mut i = 0;
            let mut board = [[0; COLS]; ROWS];
            let rng = rand::thread_rng().gen_range(0, COLS);
            for row in &mut board {
                if rows > 0 {
                    *row = [1; COLS];
                    row[rng] = 0;
                    rows -= 1;
                } else {
                    *row = self.board[i];
                    i += 1;
                }
            }
            let mut lost = false;
            for point in &self.board[i + 1] {
                if *point != 0 {
                    lost = true;
                }
            }
            if lost {
                self.lose_game();
            }
            self.board = board;
        }
    }

    pub fn save_piece(&mut self) {
        if let Some(piece) = &mut self.saved_piece {
            let p = piece.clone();
            *piece = self.current_piece.clone();
            self.current_piece = p;
        } else {
            self.saved_piece = Some(self.current_piece.clone());
            self.next_piece();
        }
    }

    fn next_piece(&mut self) {
        self.current_piece = self.next_piece.clone();
        self.next_piece = Piece::random_piece();
    }

    fn process_score(&mut self, lines_cleared: usize) {
        let (mut score, mut attack) = (0, 0);
        if lines_cleared >= 4 {
            score = 8;
            attack = 4;
        } else {
            score = lines_cleared * 2 - 1;
            attack = lines_cleared as u8 - 1;
        };
        self.score += score;
        let level = self.score / 5;
        self.gravity = TIME_LEVELS[level];
        self.outgoing = Some((attack, ATTACK_DELAY));
    }

    fn lose_game(&mut self) {
        self.lost = true;
    }

    pub fn get_board(&self) -> [[u32; COLS]; ROWS] {
        let mut board = self.board;
        for [x, y] in &self.current_piece.pos_on_board() {
            let (x, y) = (*x as usize, *y as usize);
            if x < COLS && y < ROWS {
                board[y][x] = self.current_piece.color as u32;
            }
        }
        board
    }

    fn place_piece(&mut self, alt_piece: Option<&Piece>) -> Result<(), String> {
        let piece = if let Some(piece) = alt_piece {
            piece
        } else {
            &self.current_piece
        };
        if self.valid_pos(&piece) {
            let mut lost = false;
            for [x, y] in &piece.pos_on_board() {
                self.board[*y as usize][*x as usize] = piece.color as u32;
                if *y >= (ROWS as i32 - 4) {
                    lost = true;
                }
            }
            if lost {
                self.lose_game();
            }
            return Ok(());
        }
        Err("Error placing piece on board!".to_string())
    }

    pub fn move_current(&mut self, x: i32, y: i32) {
        let old_shape = self.current_piece.shape;
        self.current_piece.mov(x, y);
        if !self.valid_pos(&self.current_piece) {
            self.adjust_current();
        }
    }

    pub fn rotate_current(&mut self, clockwise: bool) {
        let old_shape = self.current_piece.shape;
        self.current_piece.rotate(clockwise);
        if !self.valid_pos(&self.current_piece) {
            self.adjust_current();
            if !self.valid_pos(&self.current_piece) {
                self.current_piece.shape = old_shape;
            }
        }
    }

    fn valid_pos(&self, piece: &Piece) -> bool {
        for [x, y] in &piece.pos_on_board() {
            if *x < 0 || *y < 0 {
                return false;
            }
            let x = *x as usize;
            let y = *y as usize;
            if y >= ROWS || x >= COLS || self.board[y][x] != 0 {
                return false;
            }
        }
        true
    }

    fn adjust_current(&mut self) {
        let mut y_adj = 0;
        let mut x_adj = 0;
        for [x, y] in &self.current_piece.pos_on_board() {
            if *y < -y_adj && *y < 0 {
                y_adj = -*y;
            } else if (ROWS as i32 - 1 - *y) < y_adj {
                y_adj = ROWS as i32 - 1 - *y;
            }
            if *x < -x_adj && *x < 0 {
                x_adj = -*x;
            } else if (COLS as i32 - 1 - *x) < x_adj {
                x_adj = COLS as i32 - 1 - *x;
            }
            loop {
                if (*y + y_adj) < ROWS as i32                                           //UNSURE IF THIS IS A GOOD BUGFIX!
                    && self.board[(*y + y_adj) as usize][(*x + x_adj) as usize] != 0
                {
                    y_adj += 1;
                } else {
                    break;
                }
            }
        }
        self.current_piece.position[0] += x_adj;
        self.current_piece.position[1] += y_adj as i32;
    }
}

#[derive(Clone)]
struct Piece {
    shape: Shape,
    color: Color,
    position: Point,
}

impl Piece {
    pub fn new(shape: Shape, color: Color, position: Point) -> Piece {
        Piece {
            shape,
            color,
            position,
        }
    }

    pub fn random_piece() -> Piece {
        let rng = rand::thread_rng().gen_range(0, SHAPES.len());
        Piece {
            shape: SHAPES[rng],
            color: match rng {
                0 => Color::Color1,
                1 => Color::Color2,
                2 => Color::Color3,
                3 => Color::Color4,
                4 => Color::Color5,
                5 => Color::Color6,
                _ => Color::Color7,
            },
            position: [COLS as i32 / 2, ROWS as i32 - 1],
        }
    }

    fn mov(&mut self, x: i32, y: i32) {
        self.position[0] += x;
        self.position[1] += y;
    }

    fn rotate(&mut self, clockwise: bool) {
        if self.shape != SHAPES[1] {
            for block in &mut self.shape {
                block.swap(0, 1);
                if clockwise {
                    block[1] *= -1
                } else {
                    block[0] *= -1
                }
            }
        }
    }

    fn pos_on_board(&self) -> Shape {
        let mut board_pos = [[0; 2]; 4];
        for i in 0..4 {
            board_pos[i][0] = self.position[0] + self.shape[i][0];
            board_pos[i][1] = self.position[1] + self.shape[i][1];
        }
        board_pos
    }
}
