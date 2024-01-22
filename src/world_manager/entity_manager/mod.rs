//module rules;
#![allow(dead_code)]

pub mod entity;
pub struct EntityManager {
    pub generation: u16,
    pub map: fnv::FnvHashMap<(i32, i32), Vec<entity::Entity>>,
}