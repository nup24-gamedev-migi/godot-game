use std::borrow::{Borrow, BorrowMut};

use anyhow::Context;
use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use hecs::{Entity, With, World};
use tile::TileConfig;
use tile_walker::TileWalkerMovement;
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
pub struct ControllerTag;

#[derive(Debug, Clone, Copy)]
pub enum TileWalkerKind {
    Player,
    Box,
}

#[derive(Debug, Clone, Copy)]
pub enum TileKind {
    Void,
    Entrance,
    Treasure,
    Floor,
    FallenBox,
}

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

    pub fn load_level(&mut self) -> anyhow::Result<()> {
        self.world.insert_one(
            LOGIC_CFG_ENTITY,
            TileConfig { width: 10, height: 10 },
        ).unwrap();

        tile::spawn_tiles(&mut self.world).unwrap();

        let top_left = tile::get_tile_at(&self.world, 0, 0).unwrap();
        self.world.insert(top_left, (TileKind::Entrance,)).unwrap();

        // Player
        let player_tile = self.world.query::<(&TileKind,)>()
            .into_iter()
            .find_map(|(e, (kind,))| match kind {
                TileKind::Entrance => Some(e),
                _ => None,
            })
            .ok_or_else(|| anyhow::anyhow!("Failed to find entrance"))?;

        self.world.spawn((
            tile_walker::TileWalkerMovement(None),
            tile_walker::TileWalkerPos(player_tile),
            TileWalkerKind::Player,
            ControllerTag,
        ));

        Ok(())
    }

    pub fn unload_level(&mut self) {
        tile::despawn_tiles(&mut self.world).unwrap();
    }

    pub fn update(&mut self) {
        tile_walker::update_walkers(&self.world);
    }

    pub fn move_player(&mut self, new_dir: Direction) {
        let Some((_, (dir,))) = self.world.query_mut::<(&mut TileWalkerMovement,)>()
            .with::<(&ControllerTag,)>()
            .into_iter()
            .next()
        else {
            error!("Player not found");
            return;
        };

        dir.0 = Some(new_dir);
    }

    pub fn debug_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Logic debug")
        .ui(&mut *root_ui(), |ui| {
            ui.tree_node(hash!(), "general info", |ui| {
                ui.label(None, &format!("Total number of entities: {}", self.world.len()));
            });
            ui.tree_node(hash!(), "tile walkers", |ui| {
                use tile::*;
                use tile_walker::*;

                let mut query = self.world.query::<(&TileWalkerPos, &mut TileWalkerMovement, Option<&TileWalkerKind>)>();
                for (e, (pos, dir, kind)) in query.iter() {
                    let (x, y) = get_tile_pos(&self.world, pos.0);

                    ui.label(None, &format!("ent: {:?} ({:?}) at {}, {}", e, kind, x, y));
                    ui.same_line(0.);
                    if ui.button(None, "L") {
                        dir.0 = Some(Direction::Left);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "U") {
                        dir.0 = Some(Direction::Up);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "R") {
                        dir.0 = Some(Direction::Right);
                    }
                    ui.same_line(0.);
                    if ui.button(None, "D") {
                        dir.0 = Some(Direction::Down);
                    }
                    ui.separator();
                }
            });
        });
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