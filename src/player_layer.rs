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

    pub fn player_movement(&mut self, simulation_space: &Space, input: &WinitInputHelper) {
        if (input.key_held(VirtualKeyCode::W) || input.key_held(VirtualKeyCode::Up)) && simulation_space.is_solid((self.positon.0, self.positon.1 - 1)) {
            self.positon = (self.positon.0, self.positon.1 - 1)
        }
        if (input.key_held(VirtualKeyCode::A) || input.key_held(VirtualKeyCode::Left)) && simulation_space.is_solid((self.positon.0 - 1, self.positon.1)) {
            self.positon = (self.positon.0 - 1, self.positon.1)
        }
        if (input.key_held(VirtualKeyCode::S) || input.key_held(VirtualKeyCode::Down)) && simulation_space.is_solid((self.positon.0, self.positon.1 + 1)) {
            self.positon = (self.positon.0, self.positon.1 + 1)
        }
        if (input.key_held(VirtualKeyCode::D) || input.key_held(VirtualKeyCode::Right)) && simulation_space.is_solid((self.positon.0 + 1, self.positon.1)) {
            self.positon = (self.positon.0 + 1, self.positon.1)
        }
    }
}