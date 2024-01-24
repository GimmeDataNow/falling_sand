use crate::window_utils::rendering_engine::using_pixels_lib::WindowInfo;
use crate::world_manager;
use crate::world_manager::chunk_manager::cells::Cell;
use crate::world_manager::chunk_manager::cells::CellType;
use crate::world_manager::coordinates::GlobalCoords;
use crate::world_manager::entity_manager::entity::player::Player;
use winit::event::{Event, WindowEvent};
use winit::event_loop::EventLoopWindowTarget;
use winit::keyboard::KeyCode;
use winit_input_helper::WinitInputHelper;
use winit::event_loop::EventLoop;



pub fn game_loop(event_loop: Result<EventLoop<()>, winit::error::EventLoopError>, window_info: &mut WindowInfo, input: &mut WinitInputHelper, player: &mut Player, world_manager: &mut world_manager::chunk_manager::ChunkManager, chunk_cache: &mut world_manager::chunk_manager::ChunkCache) {


    match event_loop {

        Ok(event_loop) => event_loop.run(move |event_all: winit::event::Event<()>, window_target: &EventLoopWindowTarget<()>| {

            // winit_helper
            if input.update(&event_all) {

                macro_rules! map_key {

                    ($mand_1:expr, $mand_2:expr) => {
                        if input.key_held($mand_1) {
                            $mand_2;
                        }
                    };

                    ($mand_1:expr, $mand_2:expr, $mand_3:expr) => {
                        if input.key_held($mand_1) || input.key_held($mand_2) {
                            $mand_3;
                        }
                    };

                }

                // keyboard event handeling
                map_key!(KeyCode::KeyW, KeyCode::ArrowUp,   { player.position += (0.0, 1.0)  });
                map_key!(KeyCode::KeyA, KeyCode::ArrowLeft, { player.position += (-1.0, 0.0) });
                map_key!(KeyCode::KeyS, KeyCode::ArrowDown, { player.position += (0.0, -1.0) });
                map_key!(KeyCode::KeyD, KeyCode::ArrowRight,{ player.position += (1.0, 0.0)  });
                map_key!(KeyCode::Numpad5,                  { println!("{}", player.position)});
                map_key!(KeyCode::Enter,                    {chunk_cache.set_cell_force_load(world_manager, &GlobalCoords::from(player.position), Cell::new(CellType::Sand), true)});

                // mouse input handeling
                let cursor_diff = input.cursor_diff();
                if cursor_diff != (0.0, 0.0) {
                    // println!("The cursor diff is: {:?}", cursor_diff);
                    // println!("The cursor position is: {:?}", input.cursor());
                }
            }

            // redraw and other events
            if let Event::WindowEvent { event, .. } = event_all.clone() {

                match (event, window_target) {

                    (WindowEvent::CloseRequested, target) => target.exit(),

                    (WindowEvent::RedrawRequested, _) => { 
                        /* renderer here */
                        chunk_cache.draw_cells(window_info.pixels.frame_mut());

                        let _render_result = window_info.pixels.render_with(|encoder, render_target, context| {

                            // Render the world texture
                            context.scaling_renderer.render(encoder, render_target);
    
                            // is ok?
                            Ok(())
                        });
                    
                    },
                    
                    (_, _) => (),

                };
            };

            window_info.window.request_redraw();


            //match event_all {
            //    Event::Resumed => {
            //        window_info.window.request_redraw();
            //        //let _render_result = window_info.pixels.render_with(|encoder, render_target, context| {
            //        //
            //        //    // Render the world texture
            //        //    context.scaling_renderer.render(encoder, render_target);
            //        //
            //        //    Ok(())
            //        //});
            //    },
            //    _ => (),
            //}
            
        }).expect("There was an error in the event.run() loop"),
        Err(err) => panic!("There was an error in the event loop: {:?}", err)
    }
}