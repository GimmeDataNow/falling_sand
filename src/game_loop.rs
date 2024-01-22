use crate::window_utils::rendering_engine::using_pixels_lib::WindowInfo;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;

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

fn game_loop(window_info: WindowInfo) {

    match window_info.event_loop {

        Ok(event_loop) => event_loop.run(move |event: winit::event::Event<()>, window_target: &EventLoopWindowTarget<()>| {

            if let Event::WindowEvent { event, .. } = event {

                match (event, window_target) {

                    (WindowEvent::CloseRequested, target) => target.exit(),

                    (WindowEvent::RedrawRequested, _) => { /* renderer here */},

                    (WindowEvent::KeyboardInput { event, .. }, _) => (),

                    (_, _) => (),

                };
            };
            

        }).expect("There was an error in the event.run() loop"),
        Err(err) => panic!("There was an error in the event loop: {:?}", err)
    }
}