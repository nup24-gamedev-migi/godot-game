use std::collections::VecDeque;

use bevy::utils::HashMap;

use crate::{player::PlayerState, prelude::*, tiles::{TilePos, TileStorage}};

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[derive(Component, Reflect)]
pub struct JustMoved(pub bool);

pub fn solve_collisions(
    mut force_map: Local<HashMap<TilePos, (Entity, MoveDirection)>>,
    mut queue: Local<VecDeque<(Entity, MoveDirection, TilePos)>>,
    tile_st_q: Query<&TileStorage>,
    external_force_q: Query<(Entity, &PlayerState)>,
    mut walker_q: Query<(Entity, &mut TilePos, &mut JustMoved, &WalkerType)>,
) {
    /* Reset to clean state */
    walker_q.iter_mut().for_each(|mut x| {
        *x.2 = JustMoved(false);
    });

    /* Do not process collisions if the tilemap does not exist */
    let Ok(tile_st) = tile_st_q.get_single() else { return; };

    /* The algorithm */
    force_map.clear();
    queue.clear();
    external_force_q.iter()
        .filter_map(|(e, player)| {
            player.new_direction.map(|dir| (e, dir))
        })
        .filter_map(|(e, dir)| {
            let (_, pos, _, ty) = walker_q.get(e).ok()?;
            if ty.is_null() {
                return None;
            }

            Some((e, dir, *pos))
        })
        .for_each(|(e, dir, pos)| {
            queue.push_back((e, dir, pos))
        });

    if queue.is_empty() {
        return;
    }

    debug!("Root applied. Resolving collisions");
    while let Some((e, dir, pos_from)) = queue.pop_front() {
        let pos_to = pos_from.apply_direction(dir);
        trace!("{e:?}:\t{pos_from:?} -> {pos_to:?}");

        /* Ah dang it, we looped */
        if force_map.insert(pos_to, (e, dir)).is_some() {
            trace!("\t> Resolver looped");
            return;
        }

        /* Geometry checks */
        if pos_to.0 >= tile_st.width() {
            trace!("\t> Width check reject");
            return;
        }
        if pos_to.1 >= tile_st.height() {
            trace!("\t> Height check reject");
            return;
        }

        trace!("\t> Geometry checks passed");

        /* Check if we need to push someone */
        let occupier = walker_q.iter()
            .find_map(|(e, walker_pos, _, ty)| {
                if ty.is_null() {
                    return None;
                }

                if *walker_pos != pos_to {
                    return None;
                }

                Some(e)
            });
        if let Some(occupier) = occupier {
            trace!("\t> Found occupier: {occupier:?}");
            queue.push_back((occupier, dir, pos_to));
        }
    }

    /* Apply changes */
    debug!("Collisions resolved. Applying changes");
    force_map.values().for_each(|(e, dir)| {
        let Ok((_, mut pos, mut moved, ty)) = walker_q.get_mut(*e)
            else { error!("Entity {e:?} not a walker"); return; };

        *pos = pos.apply_direction(*dir);
        *moved = JustMoved(true);

        debug!("{e:?}\tpos:{pos:?}\tmoved:{moved:?}\tty:{ty:?}");
    });
}