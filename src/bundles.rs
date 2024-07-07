use crate::prelude::*;

use crate::tiles::*;
use crate::collision::*;

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