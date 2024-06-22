use hecs::{Entity, World};
use tile::TileConfig;
use crate::utils::*;

mod tile;
mod tile_walker;

pub const LOGIC_CFG_ENTITY : Entity = ent_from_id(0);

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

impl Direction {
    pub fn apply(&self, x: u32, y: u32) -> (u32, u32) {
        match self {
            Direction::Left => (x.wrapping_sub(1), y),
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::Right => (x.wrapping_add(1), y),
            Direction::Down => (x, y.wrapping_add(1)),
        }
    }
}

#[derive(Debug, Clone, Copy, Default)]
pub struct PlayerTag;

pub struct Logic {
    world: World,
}

impl Logic {
    pub fn new() -> Logic {
        let mut world = World::new();

        // Spawn cfg object
        world.spawn_at(LOGIC_CFG_ENTITY, ());

        Logic {
            world,
        }
    }

    pub fn move_player(&mut self, dir: Direction) {
        todo!()
    }
}