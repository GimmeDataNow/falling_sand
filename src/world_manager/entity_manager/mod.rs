//module rules;
#![allow(dead_code)]

pub mod entity;

use entity::{Entity, EntityType};
use entity::player::Player;

pub struct EntityManager {
    pub generation: u16,
    pub players: Vec<Entity>,
}

impl EntityManager {
    fn new() -> Self {
        EntityManager { 
            generation: 0, 
            players: vec!(Entity::new(EntityType::Player(Player::default())))
        }
    }
}