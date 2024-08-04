use bevy::log::{Level, LogPlugin};
// use bundles::{TileBundle, WalkerBundle};
// use player::PlayerState;
use prelude::*;
use sokoban_kernel::{Direction, SokobanError, SokobanKernel, ThingEntry, ThingKind, Tile};
// use tiles::TileStorage;
// use treasure_steal::TreasureState;

// mod view;
// mod logic;
// mod utils;
// mod tiles;
// mod player;
// mod shadow;
mod prelude;
// mod collision;
// mod bundles;
// mod game_state;
// mod debugging;
// mod void_fall;
// mod treasure_steal;

fn main() {
    let mut app = App::new();

    app
        .add_plugins(DefaultPlugins
            .set(LogPlugin {
                level: Level::INFO,
                filter: "".to_string(),
                // filter: "info,hunted_thief::player=trace,hunted_thief::collision=trace,hunted_thief::game_state=trace".to_string(),
                custom_layer: |_| None,
            }))
        .add_plugins(EguiPlugin);

    setup(&mut app);

    app.run();
}

fn setup_sys(
    mut cmds: Commands,
    server: Res<AssetServer>,
    sokoban: Res<SokobanKernel>,
) {

    cmds.spawn(Camera2dBundle::default());

    for (x, y, id, thing) in sokoban.state().all_things_with_metadata() {
        let texture = match thing.kind {
            ThingKind::Player => server.load("char.png"),
            ThingKind::Box => server.load("box.png"),
            ThingKind::Chest => server.load("chest.png"),
        };

        cmds.spawn((
            SokobanThing(id),
            SpriteBundle {
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(
                    (x as f32) * 32.0,
                    (y as f32) * -32.0,
                    0.0
                )),
                texture,
                ..default()
            }
        ));
    }

    let tile_iter = (0..sokoban.state().field_width())
        .flat_map(|y| (0..sokoban.state().field_height()).map(move |x| (x, y)))
        .map(|(x, y)| (x, y, sokoban.state().tile_at(x, y).unwrap()));

    for (x, y, tile) in tile_iter {
        let texture = match tile {
            Tile::Void => default(),
            Tile::Wall => server.load("boxy.png"),
            Tile::Floor => server.load("grass1.png"),
            Tile::Exit => server.load("trapdoor.png"),
        };

        cmds.spawn((
            SpriteBundle {
                transform: Transform::from_translation(Vec3::new(
                    (x as f32) * 32.0,
                    (y as f32) * -32.0,
                    0.0
                )),
                sprite: Sprite {
                    custom_size: Some(Vec2::new(32.0, 32.0)),
                    ..default()
                },
                texture,
                ..default()
            },
        ));
    }
//     use tiles::*;

//     spawn_tiles(
//         &mut cmds,
//         10,
//         10,
//         |x, y| TileBundle::new(
//             if x == 5 && y == 5 {
//                 TileType::Treasure
//             } else if y < 9 {
//                 TileType::Floor
//             } else {
//                 TileType::Void
//             }
//         )
//     );

//     cmds
//         .insert_resource(game_state::InGameState::new());
//     cmds
//         .insert_resource(shadow::PlayerMoveHistory::new());
//     cmds
//         .spawn(WalkerBundle::new(
//             TilePos(0, 0),
//             WalkerType::Player
//         ))
//         .insert(PlayerState::new());

//     cmds
//         .spawn(WalkerBundle::new(
//             TilePos(1, 0),
//             WalkerType::Box
//         ));

//     cmds
//         .spawn(WalkerBundle::new(
//             TilePos(2, 2),
//             WalkerType::Box
//         ));

//     cmds
//         .spawn(WalkerBundle::new(
//             TilePos(3, 3),
//             WalkerType::Null
//         ));
}

fn control(
    mut kernel: ResMut<SokobanKernel>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    let mut caller = || {
        if keyboard_input.just_pressed(KeyCode::KeyA) {
            kernel.move_player(Direction::Left)?;
        }

        if keyboard_input.just_pressed(KeyCode::KeyW) {
            kernel.move_player(Direction::Up)?;
        }

        if keyboard_input.just_pressed(KeyCode::KeyD) {
            kernel.move_player(Direction::Right)?;
        }

        if keyboard_input.just_pressed(KeyCode::KeyS) {
            kernel.move_player(Direction::Down)?;
        }

        Ok::<_, SokobanError>(())
    };

    if let Err(e) = caller() {
        error!("{e:?}");
    }
}

#[derive(Clone, Copy, Debug, Component)]
struct SokobanThing(usize);

fn update_entities(
    sokoban: Res<SokobanKernel>,
    mut things: Query<(&mut Transform, &SokobanThing)>,
) {
    for (x, y, thing) in sokoban.state().all_things() {
        for (mut tf, thing_handle) in things.iter_mut() {
            if thing != thing_handle.0 { continue; }

            tf.translation = Vec3::new(
                32.0 * (x as f32),
                -32.0 * (y as f32),
                1.0,
            );
        }
    }
}

fn setup(app: &mut App) {
    app
        .insert_resource(SokobanKernel::from_map(
            10,
            10,
            |x, y| {
                if x == 0 && y == 0 {
                    return Tile::Exit;
                }

                if y > 8 {
                    return Tile::Void;
                }

                if x >= 9 {
                    return Tile::Wall;
                }

                Tile::Floor
            },
            [
                (0, 0, 0, ThingEntry { kind: ThingKind::Player }),
                (3, 3, 1, ThingEntry { kind: ThingKind::Box }),
            ],
        ))
        .add_systems(Update, control)
        .add_systems(Update, update_entities)
        .add_systems(Startup, setup_sys);
        // .add_systems(PostUpdate, player::player_reset)
        // .add_systems(Update, debugging::egui_debug_level)
        // .add_systems(Startup, setup_sys)
        // .add_systems(Startup, spawn_treasure.after(setup_sys))
        // .add_systems(PreUpdate, player::player_input)
        // .add_systems(PreUpdate, game_state::react_to_input.after(player::player_input))
        // .add_systems(Update, collision::solve_collisions)
        // .add_systems(Update, void_fall::handle_void_fall.after(collision::solve_collisions))
        // .add_systems(Update, treasure_steal::treasure_stealing.after(void_fall::handle_void_fall))
        // .add_systems(Update, shadow::process_player_move.after(treasure_steal::treasure_stealing))
        // .add_systems(Update, shadow::do_shadow_walk.after(shadow::process_player_move))
        // .add_systems(Update, debugging::egui_debug_shadow.after(shadow::do_shadow_walk));
}