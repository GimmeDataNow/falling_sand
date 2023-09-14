//module rules
// should disable these once in a while to check the quality of the code
#![allow(dead_code)]
#![allow(unused_mut)]
#![allow(unused_must_use)]


// my imports
mod custom_errors;
mod chunk_manager;
use chunk_manager::chunks::cells::{Cell, CellType};
use chunk_manager::world_tools::paint_tools::BrushType;
mod config;
use crate::debug_gui::Framework;
mod debug_gui;


mod entity_manager;

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
    // should probably move these into some kind of struct
    let mut cam_pos = (0, 0);
    let mut mouse_pos = (0, 0);
    let mut fps_tracker = fps_counter::FPSCounter::new();
    let mut step_by_frame = false;
    let mut paint_brush_toggle = false;
    let mut brush = BrushType::Square(30);
    let mut paint_material = CellType::Air;
    let mut toggle_simulation = false;
    

    // builds the Widow
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(config::SCREEN_WIDTH as f32 * config::SCREEN_SCALE, config::SCREEN_HEIGHT as f32 * config::SCREEN_SCALE);
        WindowBuilder::new()
            .with_title("Re-Noita")
            .with_inner_size(size)
            //.with_min_inner_size(size)
            .build(&event_loop)
            .unwrap()
    };
    
    let mut monitor = event_loop.available_monitors().next().expect("well damn");

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
            if input.key_pressed(VirtualKeyCode::Escape) || input.close_requested() { *control_flow = ControlFlow::Exit; return; }

            // window scaling
            if let Some(scale_factor) = input.scale_factor() { framework.scale_factor(scale_factor) }

            // key binds || fulscreen may not work on mac_os, but fuck mac_os
            {
                map_key!(VirtualKeyCode::W, VirtualKeyCode::Up,     cam_pos= (cam_pos.0, cam_pos.1 + 1));
                map_key!(VirtualKeyCode::A, VirtualKeyCode::Left,   cam_pos= (cam_pos.0 - 1, cam_pos.1));
                map_key!(VirtualKeyCode::S, VirtualKeyCode::Down,   cam_pos= (cam_pos.0, cam_pos.1 - 1));
                map_key!(VirtualKeyCode::D, VirtualKeyCode::Right,  cam_pos= (cam_pos.0 + 1, cam_pos.1));
                map_key!(VirtualKeyCode::F11, window.set_fullscreen(Some(winit::window::Fullscreen::Borderless(Some(monitor.clone())))));
                map_key!(VirtualKeyCode::F12, window.set_fullscreen(None));
                map_key!(VirtualKeyCode::F3, simulation_space.set_chunk(cam_pos, CellType::Lava));
            }

            // mouse inputs
            {
                let (mouse_cell, mouse_prev_cell) = input
                .mouse()
                .map(|(mx, my)| {
                    let (dx, dy) = input.mouse_diff();
                    let prev_x = mx - dx;
                    let prev_y = my - dy;
                    let (mx_i, my_i) = pixels
                        .window_pos_to_pixel((mx, my))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    let (px_i, py_i) = pixels
                        .window_pos_to_pixel((prev_x, prev_y))
                        .unwrap_or_else(|pos| pixels.clamp_pixel_pos(pos));
                    (
                        (mx_i as isize, my_i as isize),
                        (px_i as isize, py_i as isize),
                    )
                })
                .unwrap_or_default();
                // fancy painting mode
                if (input.mouse_held(1) || input.mouse_released(1)) && paint_brush_toggle {
                    simulation_space.draw_line(cam_pos, mouse_prev_cell, mouse_cell, brush, paint_material)
                }
                mouse_pos = (mouse_cell.0 as i32, mouse_cell.1 as i32)
            }
            

            // resize the window | errors are dbg only
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    dbg!("pixels.resize_surface {}", err);
                    *control_flow = ControlFlow::Exit;
                    return;
                }
                framework.resize(size.width, size.height);
            }
            
            // step by step simulation
            if !step_by_frame {
                //simulation_space.update_cell_behaviour();
                //simulation_space.update_cell_alchemy();
            }
            if toggle_simulation {
                simulation_space.iterate_area_around_coordinate(cam_pos.0, cam_pos.1);

                // allows for overflow
                simulation_space.generation.wrapping_add(1);
            }

            // update display
            window.request_redraw();
        }

        // window events
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
                framework.prepare(&window, &mut cam_pos, &mouse_pos, &mut fps_tracker, &mut paint_brush_toggle, &mut paint_material, &mut toggle_simulation, &simulation_space);

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
                    dbg!("pixels.render {}", err);
                    *control_flow = ControlFlow::Exit;
                }
            }
            _ => (),
        }
    });
    
}