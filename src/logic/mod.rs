use hecs::{Entity, World};
use tile::TileConfig;
use crate::utils::*;

pub mod tile;
pub mod tile_walker;

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

#[cfg(test)]
mod tests {
    use super::Direction;

    #[test]
    fn dir_inv() {
        for x in 0..1000  {
            for y in 0..1000  {
                let (lx, ly) = Direction::Left.apply(x, y);
                let (ux, uy) = Direction::Up.apply(x, y);
                let (rx, ry) = Direction::Right.apply(x, y);
                let (dx, dy) = Direction::Down.apply(x, y);

                assert_eq!((x, y), Direction::Right.apply(lx, ly));
                assert_eq!((x, y), Direction::Down.apply(ux, uy));
                assert_eq!((x, y), Direction::Left.apply(rx, ry));
                assert_eq!((x, y), Direction::Up.apply(dx, dy));
            }
        }
    }
}