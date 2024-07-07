pub use bevy::prelude::*;
pub use bevy_egui::{egui, EguiContexts, EguiPlugin};

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(Reflect)]
pub enum MoveDirection {
    Left    = 0,
    Up      = 1,
    Right   = 2,
    Down    = 3,
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(Reflect, Component)]
pub enum WalkerType {
    Null    = 0,
    Player  = 1,
    Box     = 2,
}

impl WalkerType {
    pub fn is_null(&self) -> bool {
        *self == WalkerType::Null
    }
}

#[derive(Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Debug)]
#[derive(Reflect, Component)]
pub enum TileType {
    Void        = 0,
    Floor       = 1,
    Entrance    = 2,
    Treasure    = 3,
    DroppedBox  = 4,
}