//! This module is responsible for providing basic
//! functions to work with entities that walk on the tiles.
//! This module does not do any collision checks and doesn't
//! even check that the walker is trying to go in the valid
//! direction (all it does is update the position if the
//! direction points at a valid tile).

use anyhow::Context;
use macroquad::logging::error;

use super::{World, Entity, Direction, tile};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileWalkerPos(pub Entity);

#[derive(Clone, Copy, Debug)]
pub struct TileWalkerMovement(pub Option<Direction>);

pub fn get_future_walker_tile(world: &World, walker: Entity) -> anyhow::Result<Option<Entity>> {
    let mut query = world.query_one::<(&TileWalkerPos, &TileWalkerMovement)>(walker)
                     .context("Accessing entity data")?;
    let (pos, dir) = query.get().unwrap();
    let Some(dir) = dir.0 else { return Ok(None); };
    let pos = pos.0;

    tile::get_tile_neighbor(world, pos, dir)
        .ok_or_else(|| anyhow::anyhow!("Walker is targetting a non-existent tile"))
        .map(Some)
}

pub fn update_walkers(world: &World) {
    for (e, (pos, dir)) in world.query::<(&mut TileWalkerPos, &mut TileWalkerMovement)>().iter() {
        let Some(dir) = dir.0.take() else { continue; };

        if let Some(new_pos) = tile::get_tile_neighbor(world, pos.0, dir) {
            pos.0 = new_pos;
        } else {
            error!("Entity {:?} failed to move in dir {:?}", e, dir);
        }
    }
}

#[cfg(test)]
mod tests {
    use rand::{seq::SliceRandom, Rng};

    use crate::logic::{Direction, LOGIC_CFG_ENTITY};

    use super::{World, tile, update_walkers, TileWalkerPos, TileWalkerMovement};

    const TILE_SIDE: u32 = 100;
    const TILE_SIDE_TIGHT: u32 = 5;
    const TEST_GEN_COUNT: usize = 1000;

    const DIRS: [Direction; 4] = [
        Direction::Left,
        Direction::Up,
        Direction::Right,
        Direction::Down,
    ];

    fn rand_walker_pos() -> (u32, u32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..TILE_SIDE), rng.gen_range(0..TILE_SIDE))
    }

    fn rand_walker_pos_tight() -> (u32, u32) {
        let mut rng = rand::thread_rng();
        (rng.gen_range(0..TILE_SIDE_TIGHT), rng.gen_range(0..TILE_SIDE_TIGHT))
    }

    #[test]
    fn test_update_walkers_good() {
        let mut world = World::new();

        world.spawn_at(
            LOGIC_CFG_ENTITY,
            (tile::TileConfig { width: TILE_SIDE, height: TILE_SIDE },),
        );
        tile::spawn_tiles(&mut world).expect("Failed to init tiles");

        let (start_x, start_y) = rand_walker_pos();
        let e = world.spawn(
            (
                TileWalkerMovement(None),
                TileWalkerPos(tile::get_tile_at(&world, start_x, start_y).expect("Must be valid pos"))
            )
        );

        let (mut curr_x, mut curr_y) = (start_x, start_y);
        let mut rng = rand::thread_rng();

        for _test_it in 0..TEST_GEN_COUNT {
            let dir = *DIRS.as_slice().choose(&mut rng).unwrap();
            let (new_x, new_y) = dir.apply(curr_x, curr_y);
            let Some(new_pos) = tile::get_tile_at(&world, new_x, new_y) else {
                continue;
            };

            world.query_one_mut::<&mut TileWalkerMovement>(e)
                .expect("Walker must exist").0 = Some(dir);
            update_walkers(&world);
            assert_eq!(
                new_pos,
                world.query_one_mut::<&TileWalkerPos>(e)
                    .expect("Walker must exist").0
            );

            (curr_x, curr_y) = (new_x, new_y);
        }
    }

    #[test]
    fn test_update_walkers_tight_chaos() {
        let mut world = World::new();

        world.spawn_at(
            LOGIC_CFG_ENTITY,
            (tile::TileConfig { width: TILE_SIDE_TIGHT, height: TILE_SIDE_TIGHT },),
        );
        tile::spawn_tiles(&mut world).expect("Failed to init tiles");

        let (start_x, start_y) = rand_walker_pos_tight();
        let e = world.spawn(
            (
                TileWalkerMovement(None),
                TileWalkerPos(tile::get_tile_at(&world, start_x, start_y).expect("Must be valid pos"))
            )
        );

        let mut rng = rand::thread_rng();

        for _test_it in 0..TEST_GEN_COUNT {
            let dir = *DIRS.as_slice().choose(&mut rng).unwrap();
            let curr_pos = world.query_one_mut::<&TileWalkerPos>(e).expect("Walker must exist").0;
            let new_pos = tile::get_tile_neighbor(&world, curr_pos, dir)
                .unwrap_or(curr_pos);

            world.query_one_mut::<&mut TileWalkerMovement>(e)
                .expect("Walker must exist").0 = Some(dir);
            update_walkers(&world);
            assert_eq!(
                new_pos,
                world.query_one_mut::<&TileWalkerPos>(e)
                    .expect("Walker must exist").0
            );
        }
    }
}