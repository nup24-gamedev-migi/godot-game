use std::time::Duration;

use macroquad::prelude::*;
use macroquad::ui::{hash, root_ui, widgets};
use hecs::{CommandBuffer, Entity, World};
use tile::TileConfig;
use crate::utils::*;

mod transactions;
mod tile;
mod tile_walker;
mod filtering;
mod collision;

const LOGIC_CFG_ENTITY : Entity = ent_from_id(0);

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

// TODO: part of global entity
#[derive(Debug, Clone, Copy)]
pub enum GameState {
    Ready,
    MovingPlayer(Duration),
}

const PLAYER_MOVE_TIME: Duration = Duration::from_millis(120);
const RETRY_COUNT: u32 = 10;

pub struct Logic {
    state: GameState,
    buffer: CommandBuffer,
    world: World,
}

impl Logic {
    pub fn new() -> Logic {
        let mut world = World::new();

        // Spawn cfg object
        world.spawn_at(LOGIC_CFG_ENTITY, ());

        // Setup transaction system
        transactions::init(&mut world).unwrap(); // TODO: return result
        filtering::init(&mut world, []).unwrap();

        Logic {
            state: GameState::Ready,
            buffer: CommandBuffer::new(),
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
            tile_walker::TileWalkerPos(player_tile),
            TileWalkerKind::Player,
            ControllerTag,
        ));

        Ok(())
    }

    pub fn unload_level(&mut self) {
        tile::despawn_tiles(&mut self.world).unwrap();
    }

    fn on_ready(&mut self) {
        // transactions::update(&self.world).unwrap();

        // let mut retry_cnt = RETRY_COUNT;
        // loop {
        //     if filtering::update(&self.world) {
        //         break;
        //     }

        //     if retry_cnt == 0 {
        //         transactions::update(&self.world).unwrap();
        //         transactions::drop_uncommited_mutations(&self.world);
        //         break;
        //     }
        //     retry_cnt -= 1;

        //     /* fixers called here */

        //     transactions::update(&self.world).unwrap();
        // }
    }

    pub fn update(&mut self, dt: Duration) {
        match &mut self.state {
            GameState::Ready => self.on_ready(),
            GameState::MovingPlayer(timer) => {
                *timer = timer.saturating_sub(dt);

                if timer.is_zero() {
                    self.state = GameState::Ready;
                    self.on_ready();
                }
            }
        }
    }

    pub fn move_player(&mut self, new_dir: Direction) {
        if !matches!(self.state, GameState::Ready) {
            return;
        }

        let player = self.world.query::<(&TileWalkerKind,)>()
                                                        .into_iter()
                                                        .find_map(|(e, (kind,))| match kind {
                                                            TileWalkerKind::Player => Some(e),
                                                            _ => None,
                                                        });
        let Some(player) = player else {
            error!("Player not found");
            return;
        };

        if let Err(e) = tile_walker::move_walker(&self.world, player, new_dir) {
            error!("Failed to query player movement: {}", e);
            return;
        }

        self.state = GameState::MovingPlayer(PLAYER_MOVE_TIME);
    }

    pub fn debug_ui(&mut self) {
        widgets::Window::new(hash!(), vec2(470., 50.), vec2(300., 300.))
        .label("Logic debug")
        .ui(&mut *root_ui(), |ui| {
            ui.tree_node(hash!(), "general info", |ui| {
                ui.label(None, &format!("Total number of entities: {}", self.world.len()));
                ui.label(None, &format!("State: {:?}", self.state));
            });
            ui.tree_node(hash!(), "tile walkers", |ui| {
                use tile::*;
                use tile_walker::*;

                let mut query = self.world.query::<(&TileWalkerPos, Option<&TileWalkerKind>)>();
                for (e, (pos, kind)) in query.iter() {
                    let (x, y) = get_tile_pos(&self.world, pos.0);

                    ui.label(None, &format!("ent: {:?} ({:?}) at {}, {}", e, kind, x, y));
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