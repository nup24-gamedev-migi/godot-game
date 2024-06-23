use logic::Logic;
use macroquad::prelude::*;
use macroquad::ui::{widgets, hash, root_ui};

mod logic;
mod utils;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut logic = Logic::new();
    logic.load_level();

    let mut data0 = String::new();
    let mut data1 = String::new();

    let mut text0 = String::new();
    let mut text1 = String::new();

    let mut number0 = 0.;
    let mut number1 = 0.;

    loop {
        if is_key_down(KeyCode::A) {
        }

        logic.update();

        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text(&format!("HELLO {} & {}", &*n, flag), 20.0, 20.0, 20.0, DARKGRAY);

        logic.debug_ui();

        next_frame().await
    }
}