mod cells_layer;
mod player_layer;
use crate::cells_layer::Space;

use log::error;
use pixels::{Error, Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;
// debug
// use std::env;
// use backtrace::Backtrace;

//const WIDTH: u32 = 2560;
//const HEIGHT: u32 = 1440;

// const WIDTH: i32 = 512;
// const HEIGHT: i32 = 288;
// const SCALE: f32 = 5.0;

const WIDTH: i32 = 128;
const HEIGHT: i32 = 128;
const SCALE: f32 = 5.0;

// here are the env variables that toggle dev tools
const TOGGLE_DESCRIPTOR:bool = true;

fn main() -> Result<(), Error> {

    // debug section

    // env::set_var("RUST_BACKTRACE", "full");
    // let bt = Backtrace::new();

    // builds the Widow
    env_logger::init();
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(WIDTH as f32 * SCALE, HEIGHT as f32 * SCALE);
        WindowBuilder::new()
            .with_title("Re-Noita")
            .with_inner_size(size)
            .with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };

    window.set_cursor_icon(winit::window::CursorIcon::Crosshair);

    // creates a img Buffer
    let mut pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        Pixels::new(WIDTH as u32, HEIGHT as u32, surface_texture)?
    };

    // this is where the magic starts
    let mut simulation_space = cells_layer::Space::new(WIDTH, HEIGHT);
    let mut player = player_layer::PlayerState::new((0, 0),100);
    let mut counter:usize = 0;
    event_loop.run(move |event, _, control_flow| {
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            simulation_space.draw(pixels.get_frame_mut(), player);
            if let Err(err) = pixels.render() {
                error!("pixels.render() failed: {err}");
                *control_flow = ControlFlow::Exit;
                return;
            }
        }

        // Handle input events
        if input.update(&event) {

            // Exit events
            if input.key_pressed(VirtualKeyCode::Escape) || input.quit() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    error!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }


            {
                let mouse_pos = match input.mouse() {
                    Some(mouse_position_raw) => ((mouse_position_raw.0 / SCALE).trunc() as i32, (mouse_position_raw.1 / SCALE).trunc() as i32) ,
                    None => (0, 0),
                };

                #[allow(unused_parens)]
                match simulation_space.get_index_checked(mouse_pos.0, mouse_pos.1) {
                    Ok(i) => {
                        if input.mouse_held(1) { 
                            simulation_space.set_cell_checked(
                                i as usize,  
                            &cells_layer::Cell { 
                                    cell_type: cells_layer::CellType::Sand, 
                                    color: cells_layer::CellTypeProperties::get_cell_properties(cells_layer::CellType::Sand).base_color, 
                                    generation: 0, 
                                    temp: 20.0}
                            ).ok();
                        }
                        

                        if input.mouse_held(2) { 
                            let a = cells_layer::CellTypeProperties::rand_cell_properties();
                            simulation_space.set_cell_checked(i as usize, &cells_layer::Cell { 
                                cell_type: a.cell_type, 
                                color: a.base_color, 
                                generation: 0, 
                                temp: 20.0 
                            }).ok();
                        }
                        if input.key_pressed(VirtualKeyCode::Space) {
                            simulation_space.update_cell_behaviour();
                            simulation_space.update_cell_alchemy();
                        }
                        
                        if input.key_pressed(VirtualKeyCode::P) { counter += 1 }
                        if TOGGLE_DESCRIPTOR && simulation_space.index_inbounds(i) {
                            let a = simulation_space.cells[i as usize].get_properties().name;
                            
                            
                            let b = cells_layer::CellTypeProperties::get_cell_by_number(&counter);
                            if input.mouse_held(0) { 
                                simulation_space.set_cell_checked(i as usize, &cells_layer::Cell { 
                                    cell_type: b.0, 
                                    color: cells_layer::CellTypeProperties::get_cell_properties(b.0).base_color, 
                                    generation: 0, 
                                    temp: 20.0
                                    }
                                ).ok();
                            }
                            
                            print!("The selected Material is {} | You are looking at {}                                          \r",b.1, a);
                        }
                        
                    },
                    Err(_) => (),
                }
            }


            player.player_movement(&simulation_space, &input);
            pixels.get_frame_mut();
            // simulation_space.update_cell_behaviour();
            // simulation_space.update_cell_alchemy();
            window.request_redraw();

            //println!("{:?}", bt);
        }
    });
    
}

impl Space {
    fn draw(&self, frame: &mut [u8], player: player_layer::PlayerState) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let mut rgba = self.cells[i].color;
            if self.get_coordinates(i as isize) == player.positon {
                rgba = [0xFF, 0x0, 0x0, 0xFF];
            }


            pixel.copy_from_slice(&rgba);
        }
    }
}
