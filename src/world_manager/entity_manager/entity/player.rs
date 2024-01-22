//module rules;
#![allow(dead_code)]

use crate::config::DEFAULT_PLAYER_SPAWN_COORDINATES;

pub struct Player {
    pub position: (i32, i32),
    pub camera_positon: (i32, i32),
    pub velocity: (i32, i32),
}

impl Default for Player {
    fn default() -> Self {
        Player { 
            position: DEFAULT_PLAYER_SPAWN_COORDINATES, 
            camera_positon: DEFAULT_PLAYER_SPAWN_COORDINATES, 
            velocity: (0, 0),
        }
    }
}

impl Player {
    fn new(position: (i32, i32)) -> Self {
        Player { 
            position, 
            camera_positon: position, 
            velocity: (0, 0),
        }
    }
}