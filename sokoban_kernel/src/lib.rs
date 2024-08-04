mod table;

use std::collections::VecDeque;

use bevy::{prelude::*, utils::HashMap};

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
    BumpedIntoWall {
        x: usize,
        y: usize,
        dir: Direction,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
#[repr(u8)]
pub enum ThingKind {
    Player = 0,
    #[default]
    Box = 1,
    Chest = 2,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, Default)]
pub struct ThingEntry {
    pub kind: ThingKind,
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
    things: HashMap<(usize, usize), usize>,
    thing_table: HashMap<usize, ThingEntry>,
    player_hist: Vec<(usize, usize, usize, usize)>,
}

impl State {
    pub fn all_things(&self) -> impl Iterator<Item = (usize, usize, usize)> + '_ {
        self.things.iter()
            .map(|((x, y), t)| (*x, *y, *t))
    }

    pub fn all_things_with_metadata(&self) -> impl Iterator<Item = (usize, usize, usize, &'_ ThingEntry)> + '_ {
        self.all_things()
            .map(|(x, y, idx)| (x, y, idx, &self.thing_table[&idx]))
    }

    pub fn player_thing(&self) -> Option<(usize, usize, usize, &'_ ThingEntry)> {
        self.all_things_with_metadata().find(|(_, _, _, t)| {
            t.kind == ThingKind::Player
        })
    }

    pub fn treasure_exists(&self) -> bool {
        self.all_things_with_metadata().find(|(_, _, _, t)| {
            t.kind == ThingKind::Chest
        })
        .is_some()
    }

    pub fn field_width(&self) -> usize {
        self.tiles.width()
    }

    pub fn field_height(&self) -> usize {
        self.tiles.height()
    }

    pub fn tile_at(&self, x: usize, y: usize) -> Option<Tile> {
        self.tiles.get(x, y).map(|x| *x)
    }
}

#[derive(Debug)]
struct Buffers {
    push_log: Vec<(usize, usize, Direction, usize, usize, usize)>,
    push_queue: VecDeque<(usize, usize, Direction, usize)>,
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
                things: HashMap::new(),
                thing_table: HashMap::new(),
                player_hist: Vec::new(),
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
        Things: IntoIterator<Item = (usize, usize, usize, ThingEntry)>,
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
        things: Things,
    )
    where
        TileMap: FnMut(usize, usize) -> Tile,
        Things: IntoIterator<Item = (usize, usize, usize, ThingEntry)>,
    {
        self.state.tiles.resize_with(width, height, || Tile::Void);
        self.state.things.clear();
        self.state.thing_table.clear();

        for x in 0..width {
            for y in 0..height {
                self.state.tiles.set(x, y, tiles(x, y));
            }
        }

        for (x, y, idx, entry) in things {
            self.state.things.insert((x, y), idx);
            self.state.thing_table.insert(idx, entry);
        }
    }

    pub fn state(&self) -> &State {
        &self.state
    }

    // FIXME: rollback isn't really possible here
    pub fn move_player(&mut self, dir: Direction) -> Result<(), SokobanError> {
        let (px, py, pt, _) = self.state().player_thing()
            .ok_or(SokobanError::NoPlayer)?;
        let (npx, npy) = dir.apply(px, py);

        self.solve_collisions((px, py, dir, pt))?;

        let to_fall = self.state().all_things()
            .filter(|(x, y, _)| *self.state().tiles.get(*x, *y).unwrap() == Tile::Void)
            .collect::<Vec<_>>();

        for (x, y, t) in to_fall {
            self.state.things.remove(&(x, y));

            match self.state.thing_table[&t].kind {
                ThingKind::Box => {
                    info!("Box {t:} dropped at ({x:}, {y:})");
                    self.state.tiles.set(x, y, Tile::Floor)
                },
                _ => (),
            }
        }

        /* Everything has been accepted. We can record the player move */
        self.state.player_hist.push((px, py, npx, npy));

        Ok(())
    }

    fn solve_collisions(&mut self, first_push: (usize, usize, Direction, usize)) -> Result<(), SokobanError> {
        // TODO: if this code survives -- use push_log only
        self.buffers.push_table.resize(
            self.state.tiles.width(),
            self.state.tiles.height(),
        );
        self.buffers.push_log.clear();
        self.buffers.push_queue.clear();
        self.buffers.push_table.reset();

        self.buffers.push_queue.push_back(first_push);

        // Loop invariant: no collision errors
        // Loop exit guarantee: no unresolved collisions
        while let Some((x, y, dir, pusher)) = self.buffers.push_queue.pop_front() {
            self.buffers.push_table.set(x, y, Some(dir));
            let (nx, ny) = dir.apply(x, y);

            /* Range check */
            let Some(entry) = self.buffers.push_table.get(nx, ny)
                else { return Err(SokobanError::MovedOutOfRange { x, y, dir }); };

            /* Wall check */
            if self.state.tiles.get(nx, ny).map(|x| *x) == Some(Tile::Wall) {
                return Err(SokobanError::BumpedIntoWall { x, y, dir });
            }

            /* Ah dang it, we looped */
            if entry.is_some() {
                return Err(SokobanError::CollisionLoop);
            }

            /* We reached the point where the push is log-worthy */
            self.buffers.push_log.push((x, y, dir, nx, ny, pusher));

            /* Check if we need to push someone */
            let Some(occupier) = self.state.things.get(&(nx, ny))
                else { continue; };

            /* Add the occupier to the queue */
            self.buffers.push_queue.push_back((nx, ny, dir, *occupier));
        }

        /* Apply changes */
        self.buffers.push_log.iter()
            .for_each(|(x, y, _, _, _, _)| {
                self.state.things.remove(&(*x, *y));
            });

        self.state.things.extend(
            self.buffers.push_log.iter()
                .map(|(_, _, _, nx, ny, pusher)| ((*nx, *ny), *pusher))
        );

        Ok(())
    }
}

impl Default for SokobanKernel {
    fn default() -> Self {
        Self::new()
    }
}