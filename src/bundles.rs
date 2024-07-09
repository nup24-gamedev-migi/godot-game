use crate::prelude::*;

use crate::tiles::*;
use crate::collision::*;
use crate::shadow::*;

#[derive(Clone, Copy, Debug)]
#[derive(Bundle, Reflect)]
pub struct WalkerBundle {
    pub pos: TilePos,
    pub just_moved: JustMoved,
    pub ty: WalkerType,
}

impl WalkerBundle {
    pub fn new(
        pos: TilePos,
        ty: WalkerType,
    ) -> Self {
        Self {
            pos,
            ty,
            just_moved: JustMoved(false),
        }
    }
}

#[derive(Clone, Copy, Debug)]
#[derive(Bundle, Reflect)]
pub struct TileBundle {
    pub pos: TilePos,
    pub neighbor: TileNeighbor,
    pub ty: TileType,
    pub shadow_state: TileShadowState,
}

impl TileBundle {
    pub fn new(ty: TileType) -> Self {
        Self {
            ty,
            pos: TilePos(0, 0),
            neighbor: TileNeighbor::new(),
            shadow_state: TileShadowState::Free,
        }
    }
}