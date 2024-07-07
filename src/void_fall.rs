use crate::prelude::*;
use crate::tiles::{TileStorage, TilePos};

pub fn handle_void_fall(
    mut walker_q: Query<(&TilePos, &mut WalkerType)>,
    mut tile_q: Query<&mut TileType>,
    tile_st_q: Query<&TileStorage>,
) {
    let Ok(tile_st) = tile_st_q.get_single() else { return; };

    walker_q.iter_mut().for_each(|(pos, mut ty)| {
        let Some(tile) = tile_st.get_tile_at(*pos)
            else { error!("Walker at illegal pos: {pos:?}"); return; };
        let Ok(mut tile_ty) = tile_q.get_mut(tile)
            else { error!("Failed to get tile type at {pos:?}"); return; };

        if *tile_ty != TileType::Void {
            return;
        }

        *tile_ty = TileType::Floor;
        *ty = WalkerType::Null;
    })
}