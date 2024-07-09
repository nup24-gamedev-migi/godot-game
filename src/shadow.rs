use std::collections::VecDeque;

use crate::prelude::*;

use crate::player::PlayerState;
use crate::tiles::{TilePos, TileStorage};
use crate::treasure_steal::TreasureState;

#[derive(Clone, Copy, Debug)]
#[derive(Reflect)]
pub struct PlayerMoveHistoryEntry {
    pub from: TilePos,
    pub dir: MoveDirection,
}

#[derive(Clone, Debug)]
#[derive(Reflect, Resource)]
pub struct PlayerMoveHistory {
    pub list: Vec<PlayerMoveHistoryEntry>,
}

impl PlayerMoveHistory {
    pub fn new() -> Self {
        Self {
            list: Vec::new(),
        }
    }

    pub fn iter_vst_poses(&'_ self) -> impl Iterator<Item = TilePos> + '_ {
        let last = self.list.last()
            .map(|e| e.from.apply_direction(e.dir));

        self.list.iter().map(|e| e.from)
            .chain(last.into_iter())
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug, Default)]
#[derive(Reflect, Component)]
pub enum TileShadowState {
    #[default]
    Free = 0,
    Occupied = 1,
    Cycle = 2,
}

pub fn process_player_move(
    mut hist: ResMut<PlayerMoveHistory>,
    treasure_q: Query<&TreasureState>,
    player_q: Query<(&PlayerState, &TilePos)>,
) {
    let Ok((st, pos)) = player_q.get_single()
        else { return; };
    let record = treasure_q.iter()
        .all(|state| state.0);
    let Some(dir) = st.new_direction
        else { return; };

    if record {
        hist.list.push(PlayerMoveHistoryEntry {
            dir,
            from: *pos,
        });
        return;
    }
}

pub fn do_shadow_walk(
    hist: Res<PlayerMoveHistory>,
    tile_st_q: Query<&TileStorage>,
    mut tile_q: Query<&mut TileShadowState>,
) {
    /* Do nothing when tiles are not available */
    let Ok(tile_st) = tile_st_q.get_single() else { return; };

    /* Reset everything */
    tile_q.iter_mut().for_each(|mut st| *st = TileShadowState::Free);

    /* The traversal algo */
    let mut stack = VecDeque::new();
    for pos in hist.iter_vst_poses() {
        let Some(tile_e) = tile_st.get_tile_at(pos)
            else { error!("Invalid tile pos {pos:?}"); return; };
        let mut st = match tile_q.get_mut(tile_e) {
            Ok(x) => x,
            Err(e) => {
                error!("Error fetching tile state at {pos:?}: {e:?}");
                return;
            },
        };

        trace!("{pos:?}\t{st:?}");
        match *st {
            TileShadowState::Free => {
                *st = TileShadowState::Occupied
            },
            TileShadowState::Cycle => continue,
            TileShadowState::Occupied => {
                *st = TileShadowState::Cycle;

                let res = stack.iter().take_while(|x| **x != pos)
                    .try_for_each(|pos| {
                        trace!("\t>rlbck: {pos:?}");

                        let Some(tile_e) = tile_st.get_tile_at(*pos)
                            else { anyhow::bail!("Invalid tile pos {pos:?}"); };
                        let mut st = tile_q.get_mut(tile_e)
                            .map_err(|e|
                                anyhow::anyhow!("Error fetching tile state at {pos:?}: {e:?}")
                            )?;

                        *st = TileShadowState::Cycle;

                        Ok(())
                    });

                if let Err(e) = res {
                    error!("Rollback error: {e}");
                }

                continue;
            },
        }

        stack.push_front(pos);
    }
}