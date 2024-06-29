use super::{transactions, World, LOGIC_CFG_ENTITY};

pub type Filter = fn(&World) -> bool;

struct FilterSystem {
    filters: Vec<Filter>,
}

impl FilterSystem {
    fn new(filters: impl IntoIterator<Item = Filter>) -> Self {
        Self {
            filters: filters.into_iter().collect(),
        }
    }

    fn update(&self, world: &World) -> bool {
        let all_good = self.filters.iter()
                        .all(|f| f(world));

        if !all_good {
            transactions::request_revert(world);
            return false;
        }

        true
    }
}

pub fn init(world: &mut World, filters: impl IntoIterator<Item = Filter>) -> anyhow::Result<()> {
    world.insert_one(LOGIC_CFG_ENTITY, FilterSystem::new(filters))?;

    Ok(())
}

pub fn update(world: &World) -> bool {
    let mut sys = world.query_one::<(&mut FilterSystem,)>(LOGIC_CFG_ENTITY)
                                                .expect("Transaction system must be initialised");
    let sys = sys.get().unwrap();

    sys.0.update(world)
}