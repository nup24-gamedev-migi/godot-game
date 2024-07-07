use bevy::log::{Level, LogPlugin};
use bundles::{TileBundle, WalkerBundle};
use player::PlayerState;
use prelude::*;
use tiles::TileStorage;
use treasure_steal::TreasureState;

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
mod void_fall;
mod treasure_steal;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins
            .set(LogPlugin {
                level: Level::INFO,
                filter: "".to_string(),
                // filter: "info,hunted_thief::player=trace,hunted_thief::collision=trace,hunted_thief::game_state=trace".to_string(),
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
        |x, y| TileBundle::new(
            if x == 5 && y == 5 {
                TileType::Treasure
            } else if y < 9 {
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

    cmds
        .spawn(WalkerBundle::new(
            TilePos(2, 2),
            WalkerType::Box
        ));

    cmds
        .spawn(WalkerBundle::new(
            TilePos(3, 3),
            WalkerType::Null
        ));
}

fn spawn_treasure(
    tile_st_q: Query<&TileStorage>,
    mut cmds: Commands,
) {
    let Ok(tile_st) = tile_st_q.get_single() else { return; };
    let tile = tile_st.get_tile_at_pos(5, 5).unwrap();

    cmds.entity(tile).insert(TreasureState(true));
}

fn setup(app: &mut App) {
    app
        .add_systems(Update, debugging::egui_debug_level)
        .add_systems(Startup, setup_sys)
        .add_systems(Startup, spawn_treasure.after(setup_sys))
        .add_systems(PreUpdate, player::player_input)
        .add_systems(PreUpdate, game_state::react_to_input.after(player::player_input))
        .add_systems(Update, collision::solve_collisions)
        .add_systems(Update, void_fall::handle_void_fall.after(collision::solve_collisions))
        .add_systems(Update, treasure_steal::treasure_stealing.after(collision::solve_collisions));
}