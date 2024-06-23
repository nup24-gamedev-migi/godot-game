use std::time::Instant;

use logic::Logic;
use macroquad::prelude::*;
use view::GameView;

mod view;
mod logic;
mod utils;

#[macroquad::main("BasicShapes")]
async fn main() {
    let mut view = GameView::new();
    let mut logic = Logic::new();
    if let Err(e) = logic.load_level() {
        error!("Err: {}", e);
        return;
    }

    let mut last_frame = Instant::now();
    loop {
        let curr_frame = Instant::now();
        let dt = curr_frame.duration_since(last_frame);

        if is_key_down(KeyCode::A) {
            logic.move_player(logic::Direction::Left);
        }
        if is_key_down(KeyCode::W) {
            logic.move_player(logic::Direction::Up);
        }
        if is_key_down(KeyCode::D) {
            logic.move_player(logic::Direction::Right);
        }
        if is_key_down(KeyCode::S) {
            logic.move_player(logic::Direction::Down);
        }

        logic.update(dt);
        view.draw_logic(&logic);

        clear_background(RED);

        draw_line(40.0, 40.0, 100.0, 200.0, 15.0, BLUE);
        draw_rectangle(screen_width() / 2.0 - 60.0, 100.0, 120.0, 60.0, GREEN);
        draw_circle(screen_width() - 30.0, screen_height() - 30.0, 15.0, YELLOW);
        // draw_text(&format!("HELLO {} & {}", &*n, flag), 20.0, 20.0, 20.0, DARKGRAY);

        logic.debug_ui();
        view.debug_ui();

        last_frame = curr_frame;
        next_frame().await
    }
}