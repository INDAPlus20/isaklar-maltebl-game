use crate::game_data::Player;

use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, MeshBuilder, Rect};
use ggez::nalgebra::Point2;
use ggez::{Context, ContextBuilder, GameResult};

/// size of the window
pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

/// A tetris board is 10x20 blocks
pub const GRID_SIZE: (i32, i32) = (10, 20);
const GRID_LINE_WIDTH: f32 = 1.0;

/// Size of each block
pub const BLOCK_SIZE: (f32, f32) = (20.0, 20.0);

// The top-left corner of the boards
pub const P1_BOARD_PLACEMENT: (f32, f32) = (50.0, 50.0);
pub const P2_BOARD_PLACEMENT: (f32, f32) = (SCREEN_SIZE.0 / 2.0 + 50.0, 50.0);

/// The x y w h of the boards
pub const P1_BOARD: (f32, f32, f32, f32) = (
    P1_BOARD_PLACEMENT.0,                // x
    P1_BOARD_PLACEMENT.1,                // y
    (GRID_SIZE.0 as f32) * BLOCK_SIZE.0, // width
    (GRID_SIZE.1 as f32) * BLOCK_SIZE.0, // height
);
pub const P2_BOARD: (f32, f32, f32, f32) = (
    P2_BOARD_PLACEMENT.0,                // x
    P2_BOARD_PLACEMENT.1,                // y
    (GRID_SIZE.0 as f32) * BLOCK_SIZE.0, // width
    (GRID_SIZE.1 as f32) * BLOCK_SIZE.0, // height
);

const BACKGROUND_COLOR: Color = Color::new(25.0 / 255.0, 172.0 / 255.0, 244.0 / 255.0, 1.0);
const BOARD_BACKGROUND: Color = Color::new(0.0, 0.0, 0.0, 0.8);
const GRID_COLOR: Color = Color::new(100.0 / 255.0, 100.0 / 255.0, 100.0 / 255.0, 1.0);

pub const PALETTE: [Color; 8] = [
    Color::new(0.0 / 255.0, 255.0 / 255.0, 255.0 / 255.0, 1.0), // Cyan
    Color::new(255.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 1.0), // Yellow
    Color::new(128.0 / 255.0, 0.0 / 255.0, 128.0 / 255.0, 1.0), // Purple
    Color::new(0.0 / 255.0, 255.0 / 255.0, 0.0 / 255.0, 1.0),   // Green
    Color::new(255.0 / 255.0, 0.0 / 255.0, 0.0 / 255.0, 1.0),   // Red
    Color::new(0.0 / 255.0, 0.0 / 255.0, 255.0 / 255.0, 1.0),   // Blue
    Color::new(255.0 / 255.0, 127.0 / 255.0, 0.0 / 255.0, 1.0), // Orange
    Color::new(127.0 / 255.0, 127.0 / 255.0, 127.0 / 255.0, 1.0), // Grey
];

// contains fields like the game struct, ai-script, etc. Basically stores the game-state + resources
pub struct AppState {
    players: [Player; 2],
    block_palatte: [Mesh; 8],
    grid_mesh: Mesh,
}

impl AppState {
    pub fn new(ctx: &mut Context) -> AppState {
        let state = AppState {
            // Load/create resources here: images, fonts, sounds, etc.
            players: [Player::new(), Player::new()], 
            block_palatte: generate_blocks(ctx),
            grid_mesh: generate_grid_mesh(ctx).expect("grid mesh err"),
        };

        state
    }
}

impl event::EventHandler for AppState {
    // update the game logic
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }

    // update the graphics
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        // Clear screen with a the background color
        graphics::clear(ctx, BACKGROUND_COLOR);

        // draw boards
        let rectangle = Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, P1_BOARD.2 as i32, P1_BOARD.3 as i32),
            BOARD_BACKGROUND,
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            (ggez::mint::Point2 {
                x: P1_BOARD.0,
                y: P1_BOARD.1,
            },),
        )?;

        graphics::draw(
            ctx,
            &rectangle,
            (ggez::mint::Point2 {
                x: P2_BOARD.0,
                y: P2_BOARD.1,
            },),
        )?;

        let p1_board = self.players[0].get_board();
        let p2_board = self.players[1].get_board();

        // draw blocks
        for y in 0..(p1_board.len() - 4) {
            for x in 0..p1_board[y].len() {
                if p1_board[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.block_palatte[p1_board[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: P1_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                            y: P1_BOARD.1 + P1_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1,
                        },),
                    )
                    .expect("msg");
                }
            }
        }

        for y in 0..(p2_board.len() - 4) {
            for x in 0..p2_board[y].len() {
                if p2_board[y][x] > 0 {
                    graphics::draw(
                        ctx,
                        &self.block_palatte[p2_board[y][x] as usize - 1],
                        (ggez::mint::Point2 {
                            x: P2_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                            y: P2_BOARD.1 + P2_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1,
                        },),
                    )
                    .expect("msg");
                }
            }
        }

        // draw grids
        graphics::draw(
            ctx,
            &self.grid_mesh,
            (ggez::mint::Point2 {
                x: P1_BOARD.0,
                y: P1_BOARD.1,
            },),
        )?;
        graphics::draw(
            ctx,
            &self.grid_mesh,
            (ggez::mint::Point2 {
                x: P2_BOARD.0,
                y: P2_BOARD.1,
            },),
        )?;
        graphics::present(ctx)?;

        Ok(())
    }
}

fn generate_blocks(ctx: &mut Context) -> [Mesh; 8] {
    [
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[0],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[1],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[2],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[3],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[4],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[5],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[6],
        )
        .expect("Failed creating blocks"),
        Mesh::new_rectangle(
            ctx,
            DrawMode::fill(),
            Rect::new_i32(0, 0, BLOCK_SIZE.0 as i32, BLOCK_SIZE.1 as i32),
            PALETTE[7],
        )
        .expect("Failed creating blocks"),
    ]
}

fn generate_grid_mesh(ctx: &mut Context) -> GameResult<Mesh> {
    let mut mesh = MeshBuilder::new();
    for x in 0..(GRID_SIZE.0 + 1) {
        mesh.line(
            &[
                ggez::mint::Point2 {
                    x: (x as f32) * BLOCK_SIZE.0,
                    y: 0.0,
                },
                ggez::mint::Point2 {
                    x: (x as f32) * BLOCK_SIZE.0,
                    y: P1_BOARD.3,
                },
            ],
            GRID_LINE_WIDTH,
            GRID_COLOR,
        ).expect("msg");
    }
    for y in 0..(GRID_SIZE.1 + 1) {
        mesh.line(
            &[
                ggez::mint::Point2 {
                    x: 0.0,
                    y: (y as f32) * BLOCK_SIZE.1,
                },
                ggez::mint::Point2 {
                    x: P1_BOARD.2,
                    y: (y as f32) * BLOCK_SIZE.1,
                },
            ],
            GRID_LINE_WIDTH,
            GRID_COLOR,
        ).expect("msg");
    }

    mesh.build(ctx)

}

mod tests {
    use super::{AppState, SCREEN_SIZE};
    use ggez::event::{self, EventHandler};
    use ggez::graphics;
    use ggez::{Context, ContextBuilder, GameResult};

    #[test]
    fn window_test() {
        let context_builder = ggez::ContextBuilder::new("tetris", "malte och isak")
            .window_setup(ggez::conf::WindowSetup::default().title("Test goes brrr"))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(SCREEN_SIZE.0, SCREEN_SIZE.1) // Set window dimenstions
                    .resizable(true), // Fixate window size
            );

        let (contex, event_loop) = &mut context_builder.build().expect("context builder error");

        let state = &mut AppState::new(contex);

        event::run(contex, event_loop, state);
    }
}
