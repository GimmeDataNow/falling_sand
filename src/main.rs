use std::mem;


use crate::rendering_engine::start_vulkano;

mod world_manager;
use crate::world_manager::chunk_manager::chunks::Chunk;
mod custom_error;
mod config;
mod rendering_engine;

// temporary renderer
// foreign imports
use pixels::{Error, Pixels, SurfaceTexture};
use fps_counter;
use winit::dpi::LogicalSize;
use winit::event::{Event};
use winit::keyboard::KeyCode;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;


fn main() {
    let mut cam_pos = (0, 0);
    let event_loop = EventLoop::new();
    let mut input = WinitInputHelper::new();
    let window = {
        let size = LogicalSize::new(config::SCREEN_WIDTH as f32 * config::SCREEN_SCALE, config::SCREEN_HEIGHT as f32 * config::SCREEN_SCALE);
        WindowBuilder::new()
            .with_title("Re-Noita")
            .with_inner_size(size)
            //.with_min_inner_size(size)
            .build(&event_loop.as_ref().expect("failed to build"))
            .unwrap()
    };

    let mut pixels: Pixels = {
        let window_size = window.inner_size();
        let scale_factor = window.scale_factor() as f32;
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(config::SCREEN_WIDTH as u32, config::SCREEN_HEIGHT, surface_texture).expect("failed to pixel");
        pixels
    };

    let world = world_manager::chunk_manager::ChunkManager::new();
    let simulation_space = world_manager::chunk_manager::ChunkChache::new();

    let _ = event_loop.expect("error 1").run(move |event, _| {

        // Handle input events
        if input.update(&event) {

            // Exit events
            if input.key_pressed(KeyCode::Escape) || input.close_requested() { panic!("oh cool"); return; }

            // key binds || fulscreen may not work on mac_os, but fuck mac_os
            {
                if input.key_held(KeyCode::KeyW) { cam_pos= (cam_pos.0, cam_pos.1 + 1) };
                if input.key_held(KeyCode::KeyA) { cam_pos= (cam_pos.0 - 1, cam_pos.1) };
                if input.key_held(KeyCode::KeyS) { cam_pos= (cam_pos.0, cam_pos.1 - 1) };
                if input.key_held(KeyCode::KeyD) { cam_pos= (cam_pos.0 + 1, cam_pos.1) };
            }

            

            // resize the window | errors are dbg only
            if let Some(size) = input.window_resized() {
                if let Err(err) = pixels.resize_surface(size.width, size.height) {
                    dbg!("pixels.resize_surface {}", err);
                    panic!("some kind of error here");
                    return;
                }
            }
            

            // update display
            window.request_redraw();
        }

        // window events
        match event {
            // Draw the current frame
            Event::Resumed => {
                // Draw the world
                
                //simulation_space.draw(cam_pos, pixels.frame_mut());

                // Render everything together
                let render_result = pixels.render_with(|encoder, render_target, context| {

                    // Render the world texture
                    context.scaling_renderer.render(encoder, render_target);
                    // Render egui

                    Ok(())
                });

            }
            _ => (),
        }
    });
}
