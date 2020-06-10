extern crate specs;

use specs::{Component, VecStorage};

#[derive(Debug, Default)]
pub struct GameState {
    pub exit: bool,
    pub delta: f64,
    pub mouse_position: Position,
}

#[derive(Default, Component, Debug)]
#[storage(VecStorage)]
pub struct Position {
    pub x: f32,
    pub y: f32,
}
