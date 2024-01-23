//module rules;
#![allow(dead_code)]

use crate::config;
// temporary renderer
// foreign imports
use pixels::{Pixels, SurfaceTexture};
use winit::dpi::LogicalSize;
use winit::keyboard::ModifiersState;
use winit::event_loop::EventLoop;
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct WindowInfo {
    pub event_loop: Result<EventLoop<()>, winit::error::EventLoopError>,
    pub input: WinitInputHelper,
    pub window: winit::window::Window,
    pub pixels: Pixels,
    pub modifiers: ModifiersState,
}



pub fn init_window() -> WindowInfo {

    // create a new default event loop
    let event_loop: Result<EventLoop<()>, winit::error::EventLoopError> = EventLoop::new();

    // needed for input handeling later on
    let input: WinitInputHelper = WinitInputHelper::new();

    // window creation
    let window: winit::window::Window = {
        let size = LogicalSize::new(config::SCREEN_WIDTH as f32 * config::SCREEN_SCALE, config::SCREEN_HEIGHT as f32 * config::SCREEN_SCALE);
        WindowBuilder::new()
            .with_title("Re-Noita")
            .with_inner_size(size)
            //.with_min_inner_size(size)
            .build(&event_loop.as_ref().expect("failed to build"))
            .unwrap()
    };
    
    // this is where the pixels-lib is first utilised
    let pixels: Pixels = {
        let window_size = window.inner_size();
        let surface_texture = SurfaceTexture::new(window_size.width, window_size.height, &window);
        let pixels = Pixels::new(config::SCREEN_WIDTH as u32, config::SCREEN_HEIGHT, surface_texture).expect("failed to pixel");
        pixels
    };

    let modifiers: ModifiersState = ModifiersState::default();

    WindowInfo { 
        event_loop, 
        input, 
        window, 
        pixels,
        modifiers,
    }
}