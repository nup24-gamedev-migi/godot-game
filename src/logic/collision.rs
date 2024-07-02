use super::{Direction, World, Entity, tile_walker, CommandBuffer};
use tile_walker::TileWalkerPos;


#[derive(Clone, Copy, Debug)]
struct NextPos(Entity);

#[derive(Clone, Copy, Debug)]
struct PushedByInternal {
    dir: Direction,
    by: Entity,
}

#[derive(Clone, Copy, Debug)]
pub struct PushedBy {
    content: Option<PushedByInternal>,
}

/* Ensures all internal components are in-place  */
pub fn pre_update(world: &World, cmds: &mut CommandBuffer) {
    todo!()
}

pub fn update(world: &World, _cmds: &mut CommandBuffer) {
    todo!()
}