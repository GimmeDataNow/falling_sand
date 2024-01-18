pub struct EntityManager {
    pub generation: u16,
    pub map: fnv::FnvHashMap<(i32, i32), Vec<Entity>>,
}