# isaklar-maltebl-game

We have created a 1v1 Tetris game in 🦀Rust🦀, with the `ggez` library.

To run, you can either use `cargo run` or use the .exe file in ./target/release/. 

The default mode is player vs player, but if you want to play against an ai you can provide the ai-script by running the application with the file-path as an argument.
The ai-script has to be a shared library with the line ending `.so` for Unix systems or `.dll` for Windows. We have povided an example written in rust in ./ai-example/. 

Your script can be written in any language you choose as long as it can be compiled into a shared library. In rust this is simply done with `rustc --crate-type cdylib <FILENAME>.rs`
