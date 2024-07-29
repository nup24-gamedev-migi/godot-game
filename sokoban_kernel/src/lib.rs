mod table;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
#[repr(u8)]
pub enum Thing {
    Void = 0,
    Player = 1,
    Box = 2,
    Chest = 3,
}

pub struct SokobanKernel {

}