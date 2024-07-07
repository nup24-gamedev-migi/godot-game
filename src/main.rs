use bevy::log::{Level, LogPlugin};
use bundles::WalkerBundle;
use player::PlayerState;
use prelude::*;

// mod view;
// mod logic;
// mod utils;
mod tiles;
mod player;
mod prelude;
mod collision;
mod bundles;
mod game_state;
mod debugging;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins
            .set(LogPlugin {
                level: Level::DEBUG,
                filter: "info,hunted_thief::player=trace,hunted_thief::collision=trace,hunted_thief::game_state=trace".to_string(),
                update_subscriber: None,
            }))
        .add_plugins(EguiPlugin);

    setup(&mut app);

    app.run();
}

fn setup_sys(mut cmds: Commands) {
    use tiles::*;

    spawn_tiles(&mut cmds, 10, 10);

    cmds
        .insert_resource(game_state::InGameState::new());
    cmds
        .spawn(WalkerBundle::new(
            TilePos(0, 0),
            WalkerType::Player
        ))
        .insert(PlayerState::new());

    cmds
        .spawn(WalkerBundle::new(
            TilePos(1, 0),
            WalkerType::Box
        ));
}

fn setup(app: &mut App) {
    app
        .add_systems(Update, debugging::egui_debug_level)
        .add_systems(Startup, setup_sys)
        .add_systems(PreUpdate, player::player_input)
        .add_systems(PreUpdate, game_state::react_to_input.after(player::player_input))
        .add_systems(Update, collision::solve_collisions);
}

// #[macroquad::main("BasicShapes")]
// async fn main() {
//     let mut view = GameView::new();
//     let mut logic = Logic::new();
//     if let Err(e) = logic.load_level() {
//         error!("Err: {}", e);
//         return;
//     }

//     let mut last_frame = Instant::now();
//     loop {
//         let curr_frame = Instant::now();
//         let dt = curr_frame.duration_since(last_frame);

//         if is_key_down(KeyCode::A) {
//             logic.move_player(logic::Direction::Left);
//         }
//         if is_key_down(KeyCode::W) {
//             logic.move_player(logic::Direction::Up);
//         }
//         if is_key_down(KeyCode::D) {
//             logic.move_player(logic::Direction::Right);
//         }
//         if is_key_down(KeyCode::S) {
//             logic.move_player(logic::Direction::Down);
//         }

//         logic.update(dt);
//         view.draw_logic(&logic);

//         logic.debug_ui();
//         view.debug_ui();

//         last_frame = curr_frame;
//         next_frame().await
//     }
// }