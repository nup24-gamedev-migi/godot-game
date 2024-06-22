use hecs::Entity;

pub const fn ent_from_id(id: u32) -> Entity {
    let (gen, id) = (1u64, id as u64);

    match Entity::from_bits(gen << 32 | id) {
        Some(x) => x,
        None => panic!("Bad gen"),
    }
}