use super::{tile_walker::TileWalkerPos, Entity, TileKind, World, LOGIC_CFG_ENTITY};

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
            fn apply(self, world: &World, target: Entity) -> anyhow::Result<()> {
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
    pub fn new(target: Entity, ty: MutationTy) -> Self {
        Self { target, ty }
    }

    fn apply(self, world: &World) -> anyhow::Result<()> {
        self.ty.apply(world, self.target)
    }

    fn revert(self, world: &World) -> anyhow::Result<()> {
        self.ty.revert(world, self.target)
    }
}

#[derive(Clone, Debug)]
struct TransactionSystem {
    mutations: Vec<Mutation>,
    committed: Vec<usize>,
    revert_pending: bool,
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

    fn revert(&mut self, world: &World) -> anyhow::Result<()> {
        let res = (&self.mutations[self.first_uncommitted()..])
            .iter()
            .try_for_each(|x| x.revert(world));

        self.committed.pop();

        res
    }

    fn mutations_pending(&self) -> bool {
        self.first_uncommitted() < self.mutations.len()
    }

    fn new() -> Self {
        Self {
            mutations: Vec::new(),
            committed: Vec::new(),
            revert_pending: false,
        }
    }

    fn update(&mut self, world: &World) -> anyhow::Result<()> {
        while self.mutations_pending() {
            self.commit(world)?;
        }

        if self.revert_pending {
            self.revert(world)?;
            self.revert_pending = false;
        }

        Ok(())
    }

    fn add_mutation(&mut self, mu: Mutation) {
        self.mutations.push(mu);
    }
}

pub fn init(world: &mut World) -> anyhow::Result<()> {
    world.insert_one(LOGIC_CFG_ENTITY, TransactionSystem::new())?;

    Ok(())
}

pub fn add_mutation(world: &World, mu: Mutation) {
    let mut sys = world.query_one::<(&mut TransactionSystem,)>(LOGIC_CFG_ENTITY)
                                                        .expect("Transaction system must be initialised");
    let sys = sys.get().unwrap();

    sys.0.add_mutation(mu);
}

pub fn update(world: &World) -> anyhow::Result<()> {
    let mut sys = world.query_one::<(&mut TransactionSystem,)>(LOGIC_CFG_ENTITY)
                                                        .expect("Transaction system must be initialised");
    let sys = sys.get().unwrap();

    sys.0.update(world)
}

pub fn request_revert(world: &World) {
    let mut sys = world.query_one::<(&mut TransactionSystem,)>(LOGIC_CFG_ENTITY)
                    .expect("Transaction system must be initialised");
    let sys = sys.get().unwrap();

    sys.0.revert_pending = true;
}