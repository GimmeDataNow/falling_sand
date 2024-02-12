// mod game_loop;

mod world_manager;
mod custom_error;
mod config;
mod window_utils;

use crate::window_utils::rendering_engine::winit;

use pollster::FutureExt;
// use game_loop::game_loop;
//use window_utils::rendering_engine::using_pixels_lib::init_window;
// use winit_input_helper::WinitInputHelper;
use crate::world_manager::chunk_manager::chunks::Chunk;
use crate::world_manager::chunk_manager::cells::CellType;
use world_manager::coordinates::ChunkCoords;
use crate::world_manager::chunk_manager::{ChunkCache, ChunkManager};


// my renderer
// temporary renderer
// foreign imports


fn main() {
    
    let chunk = Chunk::new_from_cell_type(CellType::Pink);
    println!("{}", std::mem::size_of::<Chunk>());
    let _ = chunk.save_chunk(&ChunkCoords::from((0, 0))).expect("failed to save");
    
    winit::wgpu_run().block_on();
    
    // init_vulkano();
    // let mut input = &mut WinitInputHelper::new();
    // let mut player: Player = Player::default();
    // let mut world_map = ChunkManager::new();
    
    // for x in -4..=4 {
    //     for y in -4..=4 {
    //         world_map.load_chunk_into_map(&ChunkCoords::from((x,y)));
    //     }
    // }
    
    //let (event_loop, mut window_info) = init_window();
    //game_loop(event_loop, &mut window_info, input, &mut player, &mut world_map);
}
