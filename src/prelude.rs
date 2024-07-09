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

impl MoveDirection {
    pub fn reverse(&self) -> Self {
        match self {
            MoveDirection::Left => MoveDirection::Right,
            MoveDirection::Up => MoveDirection::Down,
            MoveDirection::Right => MoveDirection::Left,
            MoveDirection::Down => MoveDirection::Up,
        }
    }

    pub fn to_egui_vec2(&self) -> egui::Vec2 {
        match self {
            MoveDirection::Left => egui::vec2(-1., 0.),
            MoveDirection::Up => egui::vec2(0., -1.),
            MoveDirection::Right => egui::vec2(1., 0.),
            MoveDirection::Down => egui::vec2(0., 1.),
        }
    }
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