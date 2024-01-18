mod world_manager;
mod custom_error;
mod config;


fn main() {
    println!("Hello, world!");
    print!("{} | ", (1.000045_f64).floor());
    print!("{} | ", (0.000045_f64).floor());
    print!("{} | ", (-0.000045_f64).floor());
    print!("{} | ", (-1.000045_f64).floor());
}
