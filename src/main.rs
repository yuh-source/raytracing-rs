use macroquad::prelude::*;

#[macroquad::main("raytracing")]
async fn main() {
    loop {
        clear_background(BLACK);

        

        next_frame().await
    }
}