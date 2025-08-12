use macroquad::prelude::*;
//put "cargo add macroquad" in termanal to install lib
#[macroquad::main("VerletRopes")]
async fn main() {
    loop {
        clear_background(BLACK);

        next_frame().await
    }
}

