mod cells_layer;
mod config;
use pixels::{Error, Pixels, SurfaceTexture};

use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

use fps_counter;
// debug
// use std::env;
// use backtrace::Backtrace;

// here are the env variables that toggle dev tools
const TOGGLE_DESCRIPTOR:bool = true;

fn main() -> Result<(), Error> {
    let mut step_by_frame = false;
    // debug section

    // env::set_var("RUST_BACKTRACE", "full");
    // let bt = Backtrace::new();

    // builds the Widow
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(config::SCREEN_WIDTH as f32 * config::SCREEN_SCALE, config::SCREEN_HEIGHT as f32 * config::SCREEN_SCALE);
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
        Pixels::new(config::SCREEN_WIDTH as u32, config::SCREEN_HEIGHT as u32, surface_texture)?
    };

    // this is where the magic starts
    let mut simulation_space = cells_layer::Space::new(config::SCREEN_WIDTH, config::SCREEN_HEIGHT);

    let mut fps_tracker = fps_counter::FPSCounter::new();
    let mut counter:usize = 0;
    event_loop.run(move |event, _, control_flow| {
        
        // Draw the current frame
        if let Event::RedrawRequested(_) = event {
            simulation_space.draw(pixels.get_frame_mut());
            if let Err(err) = pixels.render() {
                println!("{:?}", err);
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
                    println!("pixels.resize_surface() failed: {err}");
                    *control_flow = ControlFlow::Exit;
                    return;
                }
            }


            {
                let mouse_pos = match input.mouse() {
                    Some(mouse_position_raw) => ((mouse_position_raw.0 / config::SCREEN_SCALE).trunc() as i32, (mouse_position_raw.1 / config::SCREEN_SCALE).trunc() as i32) ,
                    None => (0, 0),
                };

                #[allow(unused_parens)]
                match simulation_space.get_index_checked(mouse_pos.0, mouse_pos.1) {
                    Ok(i) => {
                        
                        if input.mouse_held(1) {  simulation_space.paint_bush(mouse_pos, 5, cells_layer::CellType::Sand, cells_layer::BrushType::Circle) }

                        if input.mouse_held(2) { 
                            let a = cells_layer::CellTypeProperties::rand_cell_properties();
                            cells_layer::Cell::build_cell(a.cell_type);
                        }
                        // toggles the step by frame mode
                        if input.key_pressed(VirtualKeyCode::Return) { step_by_frame = !step_by_frame }
                        if input.key_pressed(VirtualKeyCode::Space) {
                            simulation_space.update_cell_behaviour();
                            simulation_space.update_cell_alchemy();
                        }
                        
                        // move between the possible CellTypes
                        if input.key_pressed(VirtualKeyCode::P) { counter += 1 }


                        if TOGGLE_DESCRIPTOR && simulation_space.index_inbounds(i) {
                            let a = simulation_space.cells[i as usize].get_cell_properties().name;
                            
                            
                            let b = cells_layer::CellTypeProperties::get_cell_properties_by_number(&counter);

                            if input.mouse_held(0) { simulation_space.paint_bush(mouse_pos, 5, b.cell_type, cells_layer::BrushType::Circle);}
                            
                            //print!("The selected Material is {} | You are looking at {}                                          \r",b.name, a);
                        }
                        
                    },
                    // discard errors
                    Err(_) => (),
                }
            }


            // player.player_movement(&simulation_space, &input);
            pixels.get_frame_mut();
            if !step_by_frame {
                simulation_space.update_cell_behaviour();
                simulation_space.update_cell_alchemy();
            }
            window.request_redraw();
            println!("{}", fps_tracker.tick());
            //player.get_sim_dimensions();
            // let a = Chunk::new_with_fill(cells_layer::CellType::Sand, (0,0));
            // println!("{:#?}", Chunk::save_to_file_bin(&a));
            //println!("{:?}", bt);
        }
    });
    
}

impl Space {
    fn draw(&self, frame: &mut [u8]) {
        for (i, pixel) in frame.chunks_exact_mut(4).enumerate() {

            let mut rgba = self.cells[i].color;


            pixel.copy_from_slice(&rgba);
        }
    }
}
