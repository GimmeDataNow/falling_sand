use winit::{
    dpi::LogicalSize, 
    event::{ElementState, Event, WindowEvent}, 
    event_loop::EventLoop, 
    keyboard::{Key, NamedKey}, 
    platform::modifier_supplement::KeyEventExtModifierSupplement, 
    window::WindowBuilder
};

use wgpu;

use super::wgpu::State;

pub async fn wgpu_run() {
    env_logger::init();

    let event_loop = EventLoop::new().expect("failed to create the event_loop");

    let window: winit::window::Window = WindowBuilder::new()
        .with_inner_size(LogicalSize::new(400.0, 200.0))
        .build(&event_loop).unwrap();

    let mut state = State::new(window).await;

    let _ = event_loop.run(move |event, window_target| {
        match event {
        
            Event::WindowEvent {event , ..} => {
                match event {
                    WindowEvent::CloseRequested => { window_target.exit() },

                    WindowEvent::Resized(physical_size) => state.resize(physical_size),
                    // WindowEvent::ScaleFactorChanged { scale_factor: new_inner_size, .. } => state.resize(new_inner_size),

                    WindowEvent::KeyboardInput { event: key_event, .. } => {
                        match (key_event.state, key_event.repeat, key_event.key_without_modifiers().as_ref()) {

                            // terminate if escape is pressed
                            (ElementState::Pressed, false, Key::Named(NamedKey::Escape)) => { window_target.exit() },

                            // key detection / prints the key to the console
                            (ElementState::Pressed, _, a) => { println!("{:?}", a) },

                            // discard all else
                            (_, _, _) => {},
                        };
                    },

                    WindowEvent::RedrawRequested => {

                        // render stuff here
                        state.update();

                        match state.render() {
                            Ok(_) => {},
                            Err(wgpu::SurfaceError::Lost) => state.resize(state.size),
                            Err(wgpu::SurfaceError::OutOfMemory) => {
                                println!("wgpu::SurfaceError::OutOfMemory"); 
                                window_target.exit()
                            },
                            Err(e) => eprintln!("{:?}", e),
                        }

                    },
                    _ => (),
                }
            },
            Event::MemoryWarning => { window_target.exit() },
            _ => {},
        }
    });
}