use bevy::log::{Level, LogPlugin};
use bundles::{TileBundle, WalkerBundle};
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

    spawn_tiles(
        &mut cmds,
        10,
        10,
        |_x, y| TileBundle::new(
            if y < 9 {
                TileType::Floor
            } else {
                TileType::Void
            }
        )
    );

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