# isaklar-maltebl-game

We have created a 1v1 Tetris game in 🦀Rust🦀, with the `ggez` library.

![alt text](https://github.com/INDAPlus20/isaklar-maltebl-game/blob/main/tetris.png?raw=true)

To run, you can either use `cargo run` or compile and use the .exe file in ./target/release/. 

The default mode is player vs player, but if you want to play against an ai (or ai vs ai) you can provide one or two ai-scripts by running the application with their file-paths as an arguments, (note: player 2 will have the first script and player1 the last).
The ai-script has to be a shared library with the line ending `.so` for Unix systems or `.dll` for Windows. We have povided an example written in rust in ./ai-example/. 

Your script can be written in any language you choose as long as it can be compiled into a shared library. In rust this is simply done with `rustc --crate-type cdylib <FILENAME>.rs` (remember to do this before testing the example ai script). For exact specifications look below.

## Key-bindings

You can press `R` to restart the game at any time.

**Player 1**
| Key | Action |
|:----|:-------|
| A | Move left |
| D | Move right |
| E | Rotate clockwise |
| Q | Rotate counter-clockwise |
| S | Move down |
| W | Instant drop |
| Space | Save piece |

**Player 2**
| Key | Action |
|:----|:-------|
| J | Move left |
| L | Move right |
| O | Rotate clockwise |
| U | Rotate counter-clockwise |
| K | Move down |
| I | Instant drop |
| RShift | Save piece |

## AI-script specification

The ai script must include a function equivalent to `fn ai(*const [[u32; 10]; 24], *const [[i32; 2]; 4], *const [[i32; 2]; 4]) -> u32`. 
- The first argument in this fuction represents the board, 10 wide and 24 high (four top rows hidden) with already dropped pieces in their colours, where any value != 0 means there is a block there. 
- The second argument represents the currently controlled piece adjusted to its position on the board meaning [[3,2],[4,2],[5,2],[6,2]] represents an horizontal I-piece occupying the 3rd row from the bottom, one block from the right edge of the board. 
- The third argument represents the currently saved piece, not adjusted for position on the board, meaning an z-piece would be: [[-1,-1], [0,-1], [0,0], [1,0]]
- The output u32 designates an action for the ai to perform according to the table below.

For further information look around in the source code and ai-example or contact us.

**AI**
| Value | Action |
|:----|:-------|
| 1 | Move left |
| 2 | Move right |
| 3 | Rotate clockwise |
| 4 | Rotate counter-clockwise |
| 5 | Move down |
| 6 | Instant drop |
| 7 | Save piece |
| 0 or 8+ | Do nothing |
