use std::mem;


use crate::rendering_engine::start_vulkano;

mod world_manager;
use crate::world_manager::chunk_manager::chunks::Chunk;
mod custom_error;
mod config;
mod rendering_engine;


fn main() {
    print!("{:?}", mem::size_of::<Chunk>());
    println!("Hello, world!");
    print!("{} | ", (1.000045_f64).floor());
    print!("{} | ", (0.000045_f64).floor());
    print!("{} | ", (-0.000045_f64).floor());
    print!("{} | ", (-1.000045_f64).floor());
    start_vulkano()
}
