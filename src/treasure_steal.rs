use crate::prelude::*;
use crate::tiles::{TileStorage, TilePos};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[derive(Component)]
pub struct TreasureState(pub bool);

pub fn treasure_stealing(
    mut treasure_q: Query<&mut TreasureState>,
    tile_st_q: Query<&TileStorage>,
    tile_walker_q: Query<(&TilePos, &WalkerType)>,
) {
    let Ok(tile_st) = tile_st_q.get_single() else { return; };

    let player_iter = tile_walker_q.iter()
        .filter_map(|(pos, ty)| {
            (*ty == WalkerType::Player).then_some(*pos)
        });

    for pos in player_iter {
        let Some(tile) = tile_st.get_tile_at(pos)
            else { error!("Walker at illegal pos: {pos:?}"); return; };
        let Ok(mut treasure) = treasure_q.get_mut(tile)
            else { continue; };

        if treasure.0 {
            info!("Treasure picked up");
            treasure.0 = false;
        }
    }
}