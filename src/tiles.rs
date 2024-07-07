use crate::prelude::*;

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
#[derive(Component, Reflect)]
pub struct TilePos(pub u32, pub u32);

impl TilePos {
    pub fn apply_direction(&self, dir: MoveDirection) -> TilePos {
        match dir {
            MoveDirection::Left => TilePos(self.0.wrapping_sub(1), self.1),
            MoveDirection::Up => TilePos(self.0, self.1.wrapping_sub(1)),
            MoveDirection::Right => TilePos(self.0.wrapping_add(1), self.1),
            MoveDirection::Down => TilePos(self.0, self.1.wrapping_add(1)),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(Component, Reflect)]
pub struct TileNeighbor {
    left: Option<Entity>,
    up: Option<Entity>,
    right: Option<Entity>,
    down: Option<Entity>,
}

impl TileNeighbor {
    pub fn left(&self) -> Option<Entity> {
        self.left
    }

    pub fn up(&self) -> Option<Entity> {
        self.up
    }

    pub fn right(&self) -> Option<Entity> {
        self.right
    }

    pub fn down(&self) -> Option<Entity> {
        self.down
    }
}

// TODO: Reflect
#[derive(Clone, Debug)]
#[derive(Component)]
pub struct TileStorage {
    width: u32,
    height: u32,
    tiles: Vec<Entity>,
}

impl TileStorage {
    pub fn new(
        width: u32,
        height: u32,
        tiles: impl Iterator<Item = Entity>,
    ) -> Self {
        let tiles = tiles.collect::<Vec<_>>();

        if tiles.len() != (width * height) as usize {
            panic!("Not enough entities");
        }

        Self {
            width,
            height,
            tiles,
        }
    }

    pub fn width(&self) -> u32 { self.width }

    pub fn height(&self) -> u32 { self.height }

    pub fn get_tile_at_pos(&self, x: u32, y: u32) -> Option<Entity> {
        if x >= self.width { return None; }
        if y >= self.height { return None; }

        let idx = (x + y * self.width) as usize;
        debug_assert!(idx < self.tiles.len());

        Some(self.tiles[idx])
    }

    pub fn get_tile_at(&self, pos: TilePos) -> Option<Entity> {
        self.get_tile_at_pos(pos.0, pos.1)
    }
}

/*
 * Set up tile poses and neighbours with special systems
 */

pub fn spawn_tiles(cmds: &mut Commands, width: u32, height: u32) {
    let tile_storage = TileStorage::new(
        width, height,
        (0..width*height).map(|_| cmds.spawn_empty().id())
    );

    for x in 0..width {
        for y in 0..height {
            let pos = TilePos(x, y);
            let tl = tile_storage.get_tile_at(pos).unwrap();
            cmds.entity(tl)
                .insert(TileNeighbor {
                    left: tile_storage.get_tile_at(pos.apply_direction(MoveDirection::Left)),
                    up: tile_storage.get_tile_at(pos.apply_direction(MoveDirection::Up)),
                    right: tile_storage.get_tile_at(pos.apply_direction(MoveDirection::Right)),
                    down: tile_storage.get_tile_at(pos.apply_direction(MoveDirection::Down)),
                });
        }
    }

    cmds.spawn(tile_storage);
}