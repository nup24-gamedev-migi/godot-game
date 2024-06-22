//! This module does a stupid simple thing:
//! * Spawns the requested tiles
//! * Gives a simple mapping from positions to tiles
//! * For convinience, tags every tile with `TileTag`

use crate::utils::ent_from_id;
use super::{Entity, World, LOGIC_CFG_ENTITY};

use anyhow::Context;
use macroquad::logging::debug;

#[derive(Debug, Clone, Copy)]
pub struct TileTag;

#[derive(Debug, Clone, Copy)]
pub struct TileConfig {
    pub width: u32,
    pub height: u32,
}

fn pos_to_tile_id(cfg: TileConfig, x: u32, y: u32) -> Entity {
    ent_from_id(y*cfg.width + x + 1)
}

fn get_tile_config(game_world: &mut World) -> anyhow::Result<TileConfig> {
    Ok(*game_world.query_one_mut::<&TileConfig>(LOGIC_CFG_ENTITY)?)
}

fn tile_ent_iter(cfg: TileConfig) -> impl Iterator<Item = Entity> {
    (0..cfg.height).flat_map(move |y|
        (0..cfg.width).map(move |x| pos_to_tile_id(cfg, x, y)))
}

/// Spawns the tiles. All entity ids from 0 to width*height will be explicitly used.
pub fn spawn_tiles(game_world: &mut World) -> anyhow::Result<()> {
    debug!("Spawning tiles...");
    let cfg = get_tile_config(game_world).context("Accessing tile config to spawn tiles")?;

    debug!("Tile cfg: {:?}", cfg);
    for e in tile_ent_iter(cfg) {
        game_world.spawn_at(e, (TileTag,));
    }

    Ok(())
}

pub fn despawn_tiles(game_world: &mut World) -> anyhow::Result<()> {
    debug!("Despawning tiles...");
    let cfg = get_tile_config(game_world).context("Accessing tile config to despawn tiles")?;

    debug!("Tile cfg: {:?}", cfg);
    for e in tile_ent_iter(cfg) {
        game_world.despawn(e)
            .context("Despawning a tile")?;
    }

    Ok(())
}

pub fn get_tile_at(game_world: &mut World, x: u32, y: u32) -> Option<Entity> {
    let cfg = get_tile_config(game_world).expect("Tile config must be present");

    if x >= cfg.width {
        return None;
    }

    if y >= cfg.height {
        return None;
    }

    let ent = pos_to_tile_id(cfg, x, y);
    debug_assert!(game_world.contains(ent));

    Some(ent)
}

#[cfg(test)]
mod tests {
    use std::collections::HashSet;

    use hecs::World;

    use crate::logic::{tile::{get_tile_at, pos_to_tile_id}, LOGIC_CFG_ENTITY};

    use super::{despawn_tiles, spawn_tiles, tile_ent_iter, TileConfig, TileTag};

    const SIZES : [(u32, u32); 8] = [
        (10, 10),
        (10, 20),
        (20, 10),
        (100, 2),
        (2, 100),
        (1, 1),
        (4, 6),
        (6, 10),
    ];

    #[test]
    fn tile_id_unique() {
        for (width, height) in SIZES {
            let cfg = TileConfig { width, height };

            let set = tile_ent_iter(cfg).collect::<HashSet<_>>();

            assert_eq!(set.len(), (width * height) as usize);
        }
    }

    #[test]
    fn test_spawn_depsawn() {
        let mut world = World::new();

        for (width, height) in SIZES {
            world.spawn_at(LOGIC_CFG_ENTITY, (TileConfig { width, height },));

            spawn_tiles(&mut world).unwrap();
            despawn_tiles(&mut world).unwrap();
        }
    }

    #[test]
    fn test_spawn_depsawn_get() {
        let mut world = World::new();

        for (width, height) in SIZES {
            let cfg = TileConfig { width, height };
            world.spawn_at(LOGIC_CFG_ENTITY, (cfg,));

            spawn_tiles(&mut world).unwrap();

            for x in 0..width {
                for y in 0..height {
                    let e = get_tile_at(&mut world, x, y).expect("Tile must exist");
                    assert_eq!(e, pos_to_tile_id(cfg, x, y));
                }
            }

            despawn_tiles(&mut world).unwrap();
        }
    }

    #[test]
    fn test_spawn_depsawn_tag() {
        let mut world = World::new();

        for (width, height) in SIZES {
            let cfg = TileConfig { width, height };
            world.spawn_at(LOGIC_CFG_ENTITY, (cfg,));

            spawn_tiles(&mut world).unwrap();

            for e in tile_ent_iter(cfg) {
                world.query_one_mut::<&TileTag>(e)
                    .expect("Tiles must be tagged");
            }

            despawn_tiles(&mut world).unwrap();
        }
    }
}