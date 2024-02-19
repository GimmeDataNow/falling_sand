//module rules;
#![allow(dead_code)]

pub mod entity;

use entity::{Entity, EntityType};
use entity::player::Player;

pub struct EntityManager<'a> {
    pub players: Vec<Entity<'a>>,
}