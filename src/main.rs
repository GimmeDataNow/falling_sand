mod game_loop;

mod world_manager;
mod custom_error;
mod config;
mod window_utils;

use game_loop::game_loop;
use window_utils::rendering_engine::using_pixels_lib::init_window;
use winit_input_helper::WinitInputHelper;
use world_manager::coordinates::ChunkCoords;
use crate::world_manager::entity_manager::entity::player::Player;
use crate::world_manager::chunk_manager::{ChunkCache, ChunkManager};

// temporary renderer
// foreign imports


fn main() {
    let mut input = &mut WinitInputHelper::new();
    let mut player: Player = Player::default();
    let mut world_map = ChunkManager::new();
    let mut chunk_cache = ChunkCache::new();

    for x in (-1..=1) {
        for y in (-1..=1) {
            chunk_cache.load_chunk(&mut world_map, &ChunkCoords::from((0,0)), true);
        }
    }

    let window_info = init_window();
    game_loop(window_info, input, &mut player, world_map, chunk_cache);
}
