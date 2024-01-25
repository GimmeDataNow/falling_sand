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


// my renderer
use window_utils::rendering_engine::vulkan::init_vulkano;
// temporary renderer
// foreign imports


fn main() {
    // init_vulkano();
    let mut input = &mut WinitInputHelper::new();
    let mut player: Player = Player::default();
    let mut world_map = ChunkManager::new();
    let mut chunk_cache = ChunkCache::new();
    
    for x in -2..=2 {
        for y in -2..=2 {
            chunk_cache.load_chunk(&mut world_map, &ChunkCoords::from((0,0)), true);
        }
    }
    
    let (event_loop, mut window_info) = init_window();
    game_loop(event_loop, &mut window_info, input, &mut player, &mut world_map, &mut chunk_cache);
}
