//module rules;
#![allow(dead_code)]

pub mod player;
pub enum EntityType {
    Error,
    Player(player::Player)
}
pub struct Entity {
    pub entity_type: EntityType,
    pub additional_tags: Option<Box<String>>,
}

impl Entity {
    fn new(entity_type: EntityType) -> Self {
        Entity { 
            entity_type,
            additional_tags: None
        }
    }
}