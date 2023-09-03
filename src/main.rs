//module rules
// should disable these once in a while to check the quality of the code
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_must_use)]


// my imports
mod chunk_manager;
use chunk_manager::chunks::cells::Cell;
mod config;
use crate::debug_gui::Framework;
mod debug_gui;

#[macro_use]
mod macros;

// foreign imports
use pixels::{Error, Pixels, SurfaceTexture};
use fps_counter;
use winit::dpi::LogicalSize;
use winit::event::{Event, VirtualKeyCode};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

extern crate serde;
#[macro_use]
extern crate serde_derive;

fn main() -> Result<(), Error> {
    let mut cam_pos = (0, 0);
    let mut fps_tracker = fps_counter::FPSCounter::new();
    let mut step_by_frame = false;
    

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

    let (mut pixels, mut framework) = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(config::SCREEN_WIDTH as u32, config::SCREEN_HEIGHT_USIZE as u32, surface_texture)?;
        let framework = Framework::new(
            &event_loop,
            window_size.width,
            window_size.height,
            scale_factor,
            &pixels,
        );

        (pixels, framework)
    };

    // this is where the magic starts
    let mut simulation_space = chunk_manager::ChunkManager::new();

    // build this macro to reduce some code repetition
    macro_rules! map_key {
        ($mand_1:expr, $mand_2:expr) => {
            if input.key_held($mand_1) {
                $mand_2;
            }
        };
        ($mand_1:expr, $mand_2:expr, $mand_3:expr) => {
            if input.key_held($mand_1) || input.key_held($mand_2) {
                $mand_3
            }
        };
    }

    // this is a mess. needs
    event_loop.run(move |event, _, control_flow| {

        // Handle input events
        if input.update(&event) {

            // Exit events
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() {
                *control_flow = ControlFlow::Exit;
                return;
            }
            if let Some(scale_factor) = input.scale_factor() {
                framework.scale_factor(scale_factor);
            }

            //key binds
            map_key!(VirtualKeyCode::W, VirtualKeyCode::Up,     cam_pos= (cam_pos.0, cam_pos.1 + 1));
            map_key!(VirtualKeyCode::A, VirtualKeyCode::Left,   cam_pos= (cam_pos.0 - 1, cam_pos.1));
            map_key!(VirtualKeyCode::S, VirtualKeyCode::Down,   cam_pos= (cam_pos.0, cam_pos.1 - 1));
            map_key!(VirtualKeyCode::D, VirtualKeyCode::Right,  cam_pos= (cam_pos.0 + 1, cam_pos.1));
            map_key!(VirtualKeyCode::Return, {simulation_space.set_cell_at_global_coords(cam_pos, Cell::build_cell(chunk_manager::chunks::cells::CellType::Acid));});
            map_key!(VirtualKeyCode::F12, simulation_space.simple_save(&(0, 0)));
            
            // Resize the window
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    println!("pixels.resize_surface {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }
            
            pixels.frame_mut();
            if !step_by_frame {
                //simulation_space.update_cell_behaviour();
                //simulation_space.update_cell_alchemy();
            }
            window.request_redraw();
        }

        match event {
            Event::WindowEvent { event, .. } => {
                // Update egui inputs
                framework.handle_event(&event);
            }
            // Draw the current frame
            Event::RedrawRequested(_) => {
                // Draw the world
                simulation_space.draw(cam_pos, pixels.frame_mut());

                // Prepare egui
                framework.prepare(&window, &mut cam_pos, &mut fps_tracker);

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {

                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);
                    // Render egui
                    framework.render(encoder, render_target, context);

                    Ok(())
                });

                // Basic error handling
                if let Err(err) = render_result {
                    println!("pixels.render {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
    
}