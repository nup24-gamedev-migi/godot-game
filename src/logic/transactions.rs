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

            fn revert(self, world: &World, target: Entity) -> anyhow::Result<()> {
                match self {
                    $(
                        MutationTy::$ty { from, .. } => {
                            let mut q = world.query_one::<&mut $ty>(target)?;
                            let r = q.get().ok_or_else(|| anyhow::anyhow!("Acquiting ref failed"))?;

                            *r = from;
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

    fn revert(self, world: &World) -> anyhow::Result<()> {
        self.ty.revert(world, self.target)
    }
}

#[derive(Clone, Debug)]
pub struct TransactionSystem {
    mutations: Vec<Mutation>,
    committed: Vec<usize>,
}

impl TransactionSystem {
    fn first_uncommitted(&self) -> usize {
        self.committed.last()
            .map(|x| *x)
            .unwrap_or(0)
    }

    fn commit(&mut self, world: &World) -> anyhow::Result<()> {
        let res = (&self.mutations[self.first_uncommitted()..])
            .iter()
            .try_for_each(|x| x.apply(world));

        self.committed.push(self.mutations.len());

        res
    }
}