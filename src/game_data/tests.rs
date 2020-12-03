use super::{Color, Piece, Player, ROWS, SHAPES};
use std::io;
use std::thread;

#[test]
fn rotation() {
    let mut player = Player::new();
    player.current_piece = Piece::new(SHAPES[1], Color::Color1, [2, 2]);
    assert_eq!(
        [[1, 2], [2, 2], [2, 1], [3, 2]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[2, 3], [2, 2], [1, 2], [2, 1]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[3, 2], [2, 2], [2, 3], [1, 2]],
        player.current_piece.pos_on_board()
    );
    player.current_piece.rotate(true);
    assert_eq!(
        [[2, 1], [2, 2], [3, 2], [2, 3]],
        player.current_piece.pos_on_board()
    );
}

#[test] //not real test!
fn console_debug() {
    let mut player = Player::new();
    loop {
        let mut loop_var = 0;
        println!("-------------------------------------------");
        for line in &player.get_board() {
            loop_var += 1;
            if loop_var > (ROWS - 4) {
                break;
            }
            print!("|");
            for point in line {
                if *point == 0 {
                    print!(" ");
                } else {
                    print!("*");
                }
            }
            println!("|");
        }
        println!("-------------------------------------------");
        thread::sleep_ms(100);
        player.rotate_current(true);
        player.move_tick();
        if player.lost {
            break;
        }
        print!("{}[2J", 27 as char);
    }
    println!("game lost!");
    assert_eq!(true, player.lost);
}
