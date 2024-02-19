//module rules;
#![allow(dead_code)]

use crate::config::DEFAULT_PLAYER_SPAWN_COORDINATES_F32;
use crate::world_manager::coordinates::GlobalFloatingCoordinates;

pub struct Player {
    pub position:GlobalFloatingCoordinates,
    pub cursor_position: GlobalFloatingCoordinates,
    pub camera_positon: GlobalFloatingCoordinates,
    pub velocity: GlobalFloatingCoordinates,
}

impl Default for Player {
    fn default() -> Self {
        Player { 
            position: Into::into(DEFAULT_PLAYER_SPAWN_COORDINATES_F32), 
            cursor_position: Into::into(DEFAULT_PLAYER_SPAWN_COORDINATES_F32),
            camera_positon: Into::into(DEFAULT_PLAYER_SPAWN_COORDINATES_F32), 
            velocity: Into::into((0.0, 0.0)),
        }
    }
}

impl Player {
    pub fn new(position: GlobalFloatingCoordinates) -> Self {
        Player { 
            position: position,
            cursor_position: position,
            camera_positon: position, 
            velocity: Into::into((0.0, 0.0)),
        }
    }
}