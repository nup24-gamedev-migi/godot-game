//! This module is responsible for providing basic
//! functions to work with entities that walk on the tiles.
//! This module does not do any collision checks and doesn't
//! even check that the walker is trying to go in the valid
//! direction (all it does is update the position if the
//! direction points at a valid tile).

use super::{World, Entity, Direction};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct TileWalkerPos(pub Entity);

#[derive(Clone, Copy, Debug)]
pub struct TileWalkerMovement(pub Option<Direction>);

pub fn get_future_walker_tile(world: &mut World, walker: Entity) -> Option<Entity> {
    todo!()
}

pub fn update_walkers(world: &mut World) {
    todo!()
}
