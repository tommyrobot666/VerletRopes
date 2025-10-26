mod dim2d;
mod verletcore;
mod test2d;

use macroquad::color;
use macroquad::prelude::*;
//put "cargo add macroquad" in terminal to install lib

#[macroquad::main("VerletRopes")]
async fn main() {
    loop {
        // update
        clear_background(WHITE);
        next_frame().await;
    }
}
