use super::{tile_walker::TileWalkerPos, Entity, TileKind, World};

macro_rules! define_mutations {
    (
        $(
            $ty:ident,
        )+
    ) => {
        #[derive(Clone, Copy, Debug)]
        pub enum MutationTy {
            $( $ty {
                from: $ty,
                to: $ty,
            } ),+
        }

        impl MutationTy {
            pub fn apply(self, world: &World, target: Entity) -> anyhow::Result<()> {
                match self {
                    $(
                        MutationTy::$ty { to, .. } => {
                            let mut q = world.query_one::<&mut $ty>(target)?;
                            let r = q.get().ok_or_else(|| anyhow::anyhow!("Acquiting ref failed"))?;

                            *r = to;
                        }
                    ),+
                }

                Ok(())
            }
        }
    };
}

define_mutations!(
    TileWalkerPos,
    TileKind,
);

#[derive(Clone, Copy, Debug)]
pub struct Mutation {
    target: Entity,
    ty: MutationTy,
}

impl Mutation {
    fn apply(self, world: &World) -> anyhow::Result<()> {
        self.ty.apply(world, self.target)
    }
}