mod table;

use std::collections::VecDeque;

use bevy::prelude::*;

use table::*;

#[derive(Clone, Copy, Debug)]
pub enum SokobanError {
    NoPlayer,
    CollisionLoop,
    MovedOutOfRange {
        x: usize,
        y: usize,
        dir: Direction,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum ThingKind {
    Player = 0,
    Box = 1,
    Chest = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Thing {
    pub kind: ThingKind,
    pub id: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Tile {
    Void = 0,
    Wall = 1,
    Floor = 2,
    Exit = 3,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Direction {
    Left = 0,
    Up = 1,
    Right = 2,
    Down = 3,
}

impl Direction {
    pub fn apply(self, x: usize, y: usize) -> (usize, usize) {
        match self {
            Direction::Left => (x.wrapping_sub(1), y),
            Direction::Up => (x, y.wrapping_sub(1)),
            Direction::Right => (x.wrapping_add(1), y),
            Direction::Down => (x, y.wrapping_add(1)),
        }
    }
}

#[derive(Debug)]
pub struct State {
    tiles: Table<Tile>,
    things: Table<Option<Thing>>,
}

impl State {
    pub fn player_thing(&self) -> Option<(usize, usize, &Thing)> {
        let (px, py, pt) = self.things.find(|x| {
            x.map(|t| t.kind) == Some(ThingKind::Player)
        })?;

        Some((px, py, pt.as_ref()?))
    }

    pub fn treasure_exists(&self) -> bool {
        self.things.find(|x| {
            x.map(|t| t.kind) == Some(ThingKind::Chest)
        }).is_some()
    }
}

#[derive(Debug)]
struct Buffers {
    push_log: Vec<(usize, usize, Direction, usize, usize, Thing)>,
    push_queue: VecDeque<(usize, usize, Direction, Thing)>,
    push_table: Table<Option<Direction>>,
}

/*
    Invariants:
    * At most one player
    * At most one chest
*/
#[derive(Resource, Debug)]
pub struct SokobanKernel {
   state: State,
   buffers: Buffers,
}

impl SokobanKernel {
    pub fn new() -> Self {
        Self {
            state: State {
                tiles: Table::new(),
                things: Table::new(),
            },
            buffers: Buffers {
                push_log: Vec::new(),
                push_queue: VecDeque::new(),
                push_table: Table::new(),
            },
        }
    }

    pub fn from_map<TileMap, Things>(
        width: usize,
        height: usize,
        tiles: TileMap,
        things: Things,
    ) -> Self
    where
        TileMap: FnMut(usize, usize) -> Tile,
        Things: FnMut(usize, usize) -> Option<ThingKind>,
    {
        let mut me = Self::new();
        me.load_map(width, height, tiles, things);

        me
    }

    pub fn load_map<TileMap, Things>(
        &mut self,
        width: usize,
        height: usize,
        mut tiles: TileMap,
        mut things: Things,
    )
    where
        TileMap: FnMut(usize, usize) -> Tile,
        Things: FnMut(usize, usize) -> Option<ThingKind>,
    {
        let mut thing_idx = 0;

        self.state.tiles.resize_with(width, height, || Tile::Void);
        self.state.things.resize(width, height);

        for x in 0..width {
            for y in 0..height {
                self.state.tiles.set(x, y, tiles(x, y));

                if let Some(kind) = things(x, y) {
                    self.state.things.set(x, y, Some(Thing { kind, id: thing_idx }));

                    thing_idx += 1;
                }
            }
        }

        info!("{:?}", self.state.tiles);
        info!("{:?}", self.state.things);
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    pub fn move_player(&mut self, dir: Direction) -> Result<(), SokobanError> {
        let (px, py, pt) = self.state().player_thing()
            .ok_or(SokobanError::NoPlayer)?;
        let pt = *pt;

        // TODO: if this code survives -- use push_log only
        self.buffers.push_table.resize(
            self.state.tiles.width(),
            self.state.tiles.height(),
        );
        self.buffers.push_log.clear();
        self.buffers.push_queue.clear();
        self.buffers.push_table.reset();

        /* The algorithm */
        self.buffers.push_queue.push_back((px, py, dir, pt));

        // Loop invariant: no collision errors
        // Loop exit guarantee: no unresolved collisions
        while let Some((x, y, dir, pusher)) = self.buffers.push_queue.pop_front() {
            self.buffers.push_table.set(x, y, Some(dir));
            let (nx, ny) = dir.apply(x, y);

            /* Range check */
            let Some(entry) = self.buffers.push_table.get(nx, ny)
                else { return Err(SokobanError::MovedOutOfRange { x, y, dir }); };

            /* Ah dang it, we looped */
            if entry.is_some() {
                return Err(SokobanError::CollisionLoop);
            }

            /* We reached the point where the push is log-worthy */
            self.buffers.push_log.push((x, y, dir, nx, ny, pusher));

            /* Check if we need to push someone */
            let entry = self.state.things.get(nx, ny).unwrap();
            let Some(occupier) = *entry
                else { continue; };

            /* Add the occupier to the queue */
            self.buffers.push_queue.push_back((nx, ny, dir, occupier));
        }

        /* Apply changes */
        self.buffers.push_log.iter()
            .for_each(|(x, y, _, _, _, _)| {
                self.state.things.set(*x, *y, None);
            });
        self.buffers.push_log.iter()
            .for_each(|(_, _, _, nx, ny, pusher)| {
                self.state.things.set(*nx, *ny, Some(*pusher));
            });

        info!("{:?}", self.state.tiles);
        info!("{:?}", self.state.things);

        Ok(())
    }
}

impl Default for SokobanKernel {
    fn default() -> Self {
        Self::new()
    }
}