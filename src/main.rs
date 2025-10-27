mod dim2d;
mod verletcore;
mod test2d;
mod dim3d;
mod test3d;

use macroquad::prelude::*;
//put "cargo add macroquad" in terminal to install lib

#[macroquad::main("VerletRopes")]
async fn main() {
    let mut currently_running = Test::NotAnything;
    let text1 = "Press 2 for 2d test, 3 for 3d test";
    let text2 = "Press esc while running to return this screen";

    loop {
        if is_key_pressed(KeyCode::Escape) {
            currently_running = Test::NotAnything;
        }

        match currently_running {
            Test::NotAnything => {
                let text1_x = (screen_width()-measure_text(text1,None,20u16,1f32).width)/2f32;
                let text2_x = (screen_width()-measure_text(text2,None,20u16,1f32).width)/2f32;
                let text1_y = (screen_height()+20f32)/2f32;
                let text2_y = text1_y+20f32;

                clear_background(WHITE);

                draw_text(text1,text1_x,text1_y,20f32,BLACK);
                draw_text(text2,text2_x,text2_y,20f32,BLACK);

                if is_key_down(KeyCode::Key2) {
                    currently_running = Test::Dim2D;
                } else if is_key_down(KeyCode::Key3) {
                    currently_running = Test::Dim3D;
                }
                next_frame().await
            }
            Test::Dim2D => {
                test2d::main().await
            }
            Test::Dim3D => {
                test3d::main().await
            }
        }
    }
}

enum Test {
    NotAnything,
    Dim2D,
    Dim3D
}