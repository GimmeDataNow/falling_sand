use crate::cells_layer::Space;
use winit::event::VirtualKeyCode;
use winit_input_helper::WinitInputHelper;

// StateOfAggregation

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct PlayerState {
    pub positon: (i32, i32),
    pub velocity: (i32, i32),
    pub health: i32,
    pub is_stuck: bool,
}

impl PlayerState {
    pub fn new(positon: (i32, i32), health: i32) -> PlayerState { PlayerState { positon, velocity: ( 0, 0 ), health, is_stuck: false } }

}