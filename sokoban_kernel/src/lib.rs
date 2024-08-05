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
    shadow: Table<bool>,
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

#[derive(Clone, Copy, PartialEq, Eq, Default)]
enum Vst {
    #[default]
    Empty,
    Vst,
    Cycle,
}

/*
    Invariants:
    * At most one player
    * At most one chest
*/
#[derive(Resource, Debug)]
pub struct SokobanKernel {
   state: State,
}

impl SokobanKernel {
    pub fn new() -> Self {
        Self {
            state: State {
                shadow: Table::new(),
                tiles: Table::new(),
                things: HashMap::new(),
                thing_table: HashMap::new(),
                player_hist: Vec::new(),
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
        self.state.shadow.resize(width, height);
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

    fn update_shadow(&mut self) {
        self.state.shadow.reset();

        let count_entries =  self.state.player_hist.len().saturating_sub(3);
        let hist_iter = self.state.player_hist.iter()
            .map(|v| (v.2, v.3))
            .take(count_entries);
        let mut log = Vec::new();

        info!("begin");
        for (x, y) in hist_iter {
            log.push((x, y));
            // info!("{log:?}");

            // Rollback
            if *self.state.shadow.get(x, y).unwrap() {
                log.pop();
                while let Some((px, py)) = log.pop() {
                    if px == x && py == y {
                        break;
                    }

                    self.state.shadow.set(px, py, false);
                }

                // continue;
                log.push((x, y));
            }

            self.state.shadow.set(x, y, true);
        }

        info!("Final: {log:?}");

        self.state.shadow.reset();
        log.into_iter().for_each(|(x, y)| self.state.shadow.set(x, y, true));
    }

    // FIXME: rollback isn't really possible here
    pub fn move_player(&mut self, dir: Direction) -> Result<(), SokobanError> {
        let have_treasure = self.state.treasure_exists();
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
        self.update_shadow();

        /* Check treasure stuff */
        if have_treasure != self.state.treasure_exists() {
            // self.apply_shadow();
        }

        Ok(())
    }

    fn even_odd_check(table: &Table<bool>) -> bool {
        for y in 0..table.height() {
            for x in 0..table.width() {
                print!("{}\t", *table.get(x, y).unwrap());
            }
            print!("\n");
        }

        for y in 0..table.height() {
            let mut touched = false;
            let mut last = false;
            let mut change_count = 1;
            for x in 0..table.width() {
                let curr = *table.get(x, y).unwrap();

                if curr {
                    touched = true;
                }

                if last != curr && !curr {
                    change_count += 1;
                }

                last = curr;
            }

            if last {
                change_count += 1;
            }

            info!("scanline: {y} change_count: {change_count}");

            if change_count % 2 == 1 && touched {
                info!("Scanline {y} detected");

                return true;
            }
        }

        false
    }

    fn check_cycle(&mut self, cyc: impl Iterator<Item = (usize, usize)>) -> bool {
        info!("Begin cycle check");

        let mut vst = Table::<bool>::new_filled(
            self.state.tiles.width(),
            self.state.tiles.height()
        );

        cyc.for_each(|(x, y)| {
            info!("Mark {x} {y}");
            vst.set(x, y, true);
        });

        Self::even_odd_check(&vst)
    }

    // pub fn apply_shadow(&mut self) {
    //     let mut vst = Table::<Vst>::new_filled(
    //         self.state.tiles.width(),
    //         self.state.tiles.height()
    //     );
    //     let hist = std::mem::replace(&mut self.state.player_hist, Vec::new());
    //     let Some(start) = hist.first().map(|x| *x).map(|x| (x.0, x.1))
    //         else { return; };
    //     let mut it = std::iter::once(start).chain(
    //         hist.iter().map(|v| (v.2, v.3))
    //     );

    //     let mut buff = VecDeque::new();

    //     // FIXME: make a buff to store the path in
    //     while let Some((x, y)) = it.next() {
    //         buff.push_front((x, y));
    //         info!("{x}, {y}");
    //         match vst.get(x, y).unwrap() {
    //             Vst::Empty => {
    //                 vst.set(x, y, Vst::Vst);
    //             },
    //             Vst::Vst => {
    //                 let cyc = buff.iter()
    //                     .map(|x| *x)
    //                     .skip(1)
    //                     .take_while(|pos| *pos != (x, y))
    //                     .chain(std::iter::once(buff.front().map(|x| *x).unwrap()));
    //                 let is_cyc = self.check_cycle(cyc.clone());

    //                 if is_cyc {
    //                     cyc.for_each(|(x, y)| vst.set(x, y, Vst::Cycle));
    //                 }
    //             },
    //             Vst::Cycle => continue,
    //         }
    //     }

    //     let it = std::iter::once(start).chain(
    //         hist.iter().map(|v| (v.2, v.3))
    //     );
    //     for (x, y) in it {
    //         self.state.shadow.set(x, y, vst.get(x, y).map(|x| *x).unwrap() == Vst::Vst);
    //     }
    // }

    pub fn get_shadow(&self, x: usize, y: usize) -> Option<bool> {
        self.state.shadow.get(x, y).map(|x| *x)
    }

    fn solve_collisions(&mut self, first_push: (usize, usize, Direction, usize)) -> Result<(), SokobanError> {
        let mut push_log = Vec::new();
        let mut push_queue = VecDeque::<(usize, usize, Direction, usize)>::new();
        let mut push_table = Table::<bool>::new_filled(
            self.state.tiles.width(),
            self.state.tiles.height()
        );

        push_queue.push_back(first_push);

        // Loop invariant: no collision errors
        // Loop exit guarantee: no unresolved collisions
        while let Some((x, y, dir, pusher)) = push_queue.pop_front() {
            push_table.set(x, y, true);
            let (nx, ny) = dir.apply(x, y);

            /* Range check */
            let Some(entry) = push_table.get(nx, ny)
                else { return Err(SokobanError::MovedOutOfRange { x, y, dir }); };

            /* Wall check */
            if self.state.tiles.get(nx, ny).map(|x| *x) == Some(Tile::Wall) {
                return Err(SokobanError::BumpedIntoWall { x, y, dir });
            }

            /* Ah dang it, we looped */
            if *entry {
                return Err(SokobanError::CollisionLoop);
            }

            /* We reached the point where the push is log-worthy */
            push_log.push((x, y, dir, nx, ny, pusher));

            /* Check if we need to push someone */
            let Some(occupier) = self.state.things.get(&(nx, ny))
                else { continue; };

            /* Player can collide into the chest */
            let pusher_kind = self.state.thing_table[&pusher].kind;
            let pushed_kind = self.state.thing_table[occupier].kind;
            if pusher_kind == ThingKind::Player && pushed_kind == ThingKind::Chest {
                continue;
            }

            /* Add the occupier to the queue */
            push_queue.push_back((nx, ny, dir, *occupier));
        }

        /* Apply changes */
        push_log.iter()
            .for_each(|(x, y, _, _, _, _)| {
                self.state.things.remove(&(*x, *y));
            });

        self.state.things.extend(
            push_log.iter()
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