//! This module is responsible for providing basic
//! functions to work with entities that walk on the tiles.
//! This module does not do any collision checks and doesn't
//! even check that the walker is trying to go in the valid
//! direction (all it does is update the position if the
//! direction points at a valid tile).

use anyhow::Context;
use macroquad::logging::error;

use super::{tile, transactions::{self, Mutation}, Direction, Entity, World};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileWalkerPos(pub Entity);

pub fn get_walker_pos(world: &World, walker: Entity) -> anyhow::Result<Entity> {
    let mut pos = world.query_one::<(&TileWalkerPos,)>(walker)
        .context("Quering waker")?;

    Ok(pos.get().unwrap().0.0)
}

pub fn move_walker(world: &World, walker: Entity, dir: Direction) -> anyhow::Result<()> {
    let curr_pos = get_walker_pos(world, walker)?;
    let new_pos = tile::get_tile_neighbor(world, curr_pos, dir)
                            .ok_or_else(|| anyhow::anyhow!("No neighbor"))?;

    transactions::add_mutation(world, Mutation::new(
        walker,
        transactions::MutationTy::TileWalkerPos {
            from: TileWalkerPos(curr_pos),
            to: TileWalkerPos(new_pos),
        },
    ));

    Ok(())
}

pub fn any_walkers_at(world: &World, tile: Entity) -> bool {
    world.query::<(&TileWalkerPos,)>().into_iter()
        .filter(|(_, (pos,))| pos.0 == tile)
        .next()
        .is_some()
}

#[cfg(test)]
mod tests {
    // use rand::{seq::SliceRandom, Rng};

    // use crate::logic::{Direction, LOGIC_CFG_ENTITY};

    // use super::{World, tile, update_walkers, TileWalkerPos, TileWalkerMovement};

    // const TILE_SIDE: u32 = 100;
    // const TILE_SIDE_TIGHT: u32 = 5;
    // const TEST_GEN_COUNT: usize = 1000;

    // const DIRS: [Direction; 4] = [
    //     Direction::Left,
    //     Direction::Up,
    //     Direction::Right,
    //     Direction::Down,
    // ];

    // fn rand_walker_pos() -> (u32, u32) {
    //     let mut rng = rand::thread_rng();
    //     (rng.gen_range(0..TILE_SIDE), rng.gen_range(0..TILE_SIDE))
    // }

    // fn rand_walker_pos_tight() -> (u32, u32) {
    //     let mut rng = rand::thread_rng();
    //     (rng.gen_range(0..TILE_SIDE_TIGHT), rng.gen_range(0..TILE_SIDE_TIGHT))
    // }

    // #[test]
    // fn test_update_walkers_good() {
    //     let mut world = World::new();

    //     world.spawn_at(
    //         LOGIC_CFG_ENTITY,
    //         (tile::TileConfig { width: TILE_SIDE, height: TILE_SIDE },),
    //     );
    //     tile::spawn_tiles(&mut world).expect("Failed to init tiles");

    //     let (start_x, start_y) = rand_walker_pos();
    //     let e = world.spawn(
    //         (
    //             TileWalkerMovement(None),
    //             TileWalkerPos(tile::get_tile_at(&world, start_x, start_y).expect("Must be valid pos"))
    //         )
    //     );

    //     let (mut curr_x, mut curr_y) = (start_x, start_y);
    //     let mut rng = rand::thread_rng();

    //     for _test_it in 0..TEST_GEN_COUNT {
    //         let dir = *DIRS.as_slice().choose(&mut rng).unwrap();
    //         let (new_x, new_y) = dir.apply(curr_x, curr_y);
    //         let Some(new_pos) = tile::get_tile_at(&world, new_x, new_y) else {
    //             continue;
    //         };

    //         world.query_one_mut::<&mut TileWalkerMovement>(e)
    //             .expect("Walker must exist").0 = Some(dir);
    //         update_walkers(&world);
    //         assert_eq!(
    //             new_pos,
    //             world.query_one_mut::<&TileWalkerPos>(e)
    //                 .expect("Walker must exist").0
    //         );

    //         (curr_x, curr_y) = (new_x, new_y);
    //     }
    // }

    // #[test]
    // fn test_update_walkers_tight_chaos() {
    //     let mut world = World::new();

    //     world.spawn_at(
    //         LOGIC_CFG_ENTITY,
    //         (tile::TileConfig { width: TILE_SIDE_TIGHT, height: TILE_SIDE_TIGHT },),
    //     );
    //     tile::spawn_tiles(&mut world).expect("Failed to init tiles");

    //     let (start_x, start_y) = rand_walker_pos_tight();
    //     let e = world.spawn(
    //         (
    //             TileWalkerMovement(None),
    //             TileWalkerPos(tile::get_tile_at(&world, start_x, start_y).expect("Must be valid pos"))
    //         )
    //     );

    //     let mut rng = rand::thread_rng();

    //     for _test_it in 0..TEST_GEN_COUNT {
    //         let dir = *DIRS.as_slice().choose(&mut rng).unwrap();
    //         let curr_pos = world.query_one_mut::<&TileWalkerPos>(e).expect("Walker must exist").0;
    //         let new_pos = tile::get_tile_neighbor(&world, curr_pos, dir)
    //             .unwrap_or(curr_pos);

    //         world.query_one_mut::<&mut TileWalkerMovement>(e)
    //             .expect("Walker must exist").0 = Some(dir);
    //         update_walkers(&world);
    //         assert_eq!(
    //             new_pos,
    //             world.query_one_mut::<&TileWalkerPos>(e)
    //                 .expect("Walker must exist").0
    //         );
    //     }
    // }
}