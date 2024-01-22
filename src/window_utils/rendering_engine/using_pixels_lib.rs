//module rules;
#![allow(dead_code)]

use crate::config;
// temporary renderer
// foreign imports
use pixels::{Error, Pixels, SurfaceTexture};
use fps_counter;
use winit::dpi::LogicalSize;
use winit::event::{Event};
use winit::keyboard::{KeyCode, ModifiersState};
use winit::event_loop::{ControlFlow, EventLoop};
use winit::window::WindowBuilder;
use winit_input_helper::WinitInputHelper;

pub struct WindowInfo {
    pub event_loop: Result<EventLoop<()>, winit::error::EventLoopError>,
    pub input: WinitInputHelper,
    pub window: winit::window::Window,
    pub pixels: Pixels,
    pub modifiers: ModifiersState,
}

fn init_window() -> WindowInfo {

    let event_loop: Result<EventLoop<()>, winit::error::EventLoopError> = EventLoop::new();


    let input: WinitInputHelper = WinitInputHelper::new();


    let window: winit::window::Window = {
        let size = LogicalSize::new(config::SCREEN_WIDTH as f32 * config::SCREEN_SCALE, config::SCREEN_HEIGHT as f32 * config::SCREEN_SCALE);
        WindowBuilder::new()
            .with_title("Re-Noita")
            .with_inner_size(size)
            //.with_min_inner_size(size)
            .build(&event_loop.as_ref().expect("failed to build"))
            .unwrap()
    };
    
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

