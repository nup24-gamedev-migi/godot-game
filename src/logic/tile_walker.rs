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
