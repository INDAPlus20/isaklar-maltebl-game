pub mod app {

    use ggez::event::{self, EventHandler};
    use ggez::graphics::{self, Color, DrawMode, DrawParam, Mesh, Rect, MeshBuilder};
    use ggez::{Context, ContextBuilder, GameResult};
    use ggez::nalgebra::Point2;

    /// size of the window
    pub const SCREEN_SIZE: (f32, f32) = (800.0, 600.0);

    /// A tetris board is 10x20 blocks
    pub const GRID_SIZE: (i32, i32) = (10, 20);
    const GRID_LINE_WIDTH: f32 = 1.0;

    /// Size of each block
    pub const BLOCK_SIZE: (f32, f32) = (20.0, 20.0);

    // The top-left corner of the boards
    pub const P1_BOARD_PLACEMENT: (f32, f32) = (50.0, 50.0);
    pub const P2_BOARD_PLACEMENT: (f32, f32) = (SCREEN_SIZE.0/2.0 + 50.0, 50.0);

    /// The x y w h of the boards
    pub const P1_BOARD: (f32, f32, f32, f32) = (
        P1_BOARD_PLACEMENT.0,                   // x
        P1_BOARD_PLACEMENT.1,                   // y
        (GRID_SIZE.0 as f32) * BLOCK_SIZE.0, // width
        (GRID_SIZE.1 as f32) * BLOCK_SIZE.0, // height
    );
    pub const P2_BOARD: (f32, f32, f32, f32) = (
        P2_BOARD_PLACEMENT.0,                   // x
        P2_BOARD_PLACEMENT.1,                   // y
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

    // contains fields like the game struct, ai-script, etc. Basically stores the game-state
    pub struct AppState {
        p1_board: [[u32; 20]; 10],
        p2_board: [[u32; 20]; 10],
        block_palatte: [Mesh; 8],
        grid_mesh: Mesh

    }

    impl AppState {
        pub fn new(ctx: &mut Context) -> AppState {
            let state = AppState {
            // Load/create resources here: images, fonts, sounds, etc.
            p1_board: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 2, 2, 0, 0, 0, 0, 0],
                [7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            p2_board: [
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 1, 1, 0, 0, 0, 0],
                [1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 1, 0, 0, 0, 0, 0],
                [1, 1, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [1, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [2, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [7, 7, 7, 7, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
                [3, 3, 3, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0],
            ],
            block_palatte: generate_blocks(ctx),
            grid_mesh: generate_grid_mesh(ctx).expect("grid mesh err")
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

            // draw blocks
            for x in 0..self.p1_board.len() {
                for y in 0..self.p1_board[x].len() {
                    if self.p1_board[x][y] > 0 {
                        graphics::draw(
                            ctx,
                            &self.block_palatte[self.p1_board[x][y] as usize - 1],
                            (ggez::mint::Point2 {
                                x: P1_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                                y: P1_BOARD.1 + P1_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1
                            },),
                        )
                        .expect("msg");
                    }
                }
            }

            for x in 0..self.p2_board.len() {
                for y in 0..self.p2_board[x].len() {
                    if self.p2_board[x][y] > 0 {
                        graphics::draw(
                            ctx,
                            &self.block_palatte[self.p2_board[x][y] as usize - 1],
                            (ggez::mint::Point2 {
                                x: P2_BOARD.0 + (x as f32) * BLOCK_SIZE.0,
                                y: P2_BOARD.1 + P2_BOARD.3 - ((y as f32) + 1.0) * BLOCK_SIZE.1
                            },),
                        )
                        .expect("msg");
                    }
                }
            }


            // draw grids
            graphics::draw(ctx, &self.grid_mesh, (ggez::mint::Point2 {
                x: P1_BOARD.0 ,
                y: P1_BOARD.1
            },),)?;
            graphics::draw(ctx, &self.grid_mesh, (ggez::mint::Point2 {
                x: P2_BOARD.0 ,
                y: P2_BOARD.1
            },),)?;
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
        MeshBuilder::new()
            // Vertical lines
            .line(&[ggez::mint::Point2{x: 0.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 0.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 1.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 1.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 2.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 2.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 3.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 3.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 4.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 4.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 5.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 5.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 6.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 6.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 7.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 7.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 8.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 8.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 9.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 9.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 10.0 * BLOCK_SIZE.0, y: 0.0}, ggez::mint::Point2{x: 10.0 * BLOCK_SIZE.0, y: P1_BOARD.3}], GRID_LINE_WIDTH, GRID_COLOR)?
            // Horisontal lines
            .line(&[ggez::mint::Point2{x: 0.0 , y: 0.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 0.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 1.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 1.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 2.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 2.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 3.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 3.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 4.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 4.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 5.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 5.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 6.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 6.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 7.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 7.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 8.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 8.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 9.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 9.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 10.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 10.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 11.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 11.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 12.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 12.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 13.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 13.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 14.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 14.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 15.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 15.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 16.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 16.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 17.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 17.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 18.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 18.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 19.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 19.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .line(&[ggez::mint::Point2{x: 0.0 , y: 20.0 * BLOCK_SIZE.1}, ggez::mint::Point2{x: P1_BOARD.2 , y: 20.0 * BLOCK_SIZE.1}], GRID_LINE_WIDTH, GRID_COLOR)?
            .build(ctx)
    }
}

mod tests {
    use super::app::AppState;
    use ggez::event::{self, EventHandler};
    use ggez::graphics;
    use ggez::{Context, ContextBuilder, GameResult};

    #[test]
    fn window_test() {
        let context_builder = ggez::ContextBuilder::new("tetris", "malte och isak")
            .window_setup(ggez::conf::WindowSetup::default().title("Test goes brrr"))
            .window_mode(
                ggez::conf::WindowMode::default()
                    .dimensions(super::app::SCREEN_SIZE.0, super::app::SCREEN_SIZE.1) // Set window dimenstions
                    .resizable(true), // Fixate window size
            );

        let (contex, event_loop) = &mut context_builder.build().expect("context builder error");

        let state = &mut AppState::new(contex);

        event::run(contex, event_loop, state);
    }
}
