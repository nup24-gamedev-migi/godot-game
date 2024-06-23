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

        logic.debug_ui();
        view.debug_ui();

        last_frame = curr_frame;
        next_frame().await
    }
}