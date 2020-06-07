extern crate specs;

use specs::{Component, VecStorage};

#[derive(Debug)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Default for Color {
    fn default() -> Color {
        Color {
            r: 0.,
            g: 0.,
            b: 0.,
            a: 1.,
        }
    }
}

impl Color {
    pub fn to_array(&self) -> [f32; 4] {
        [self.r, self.g, self.b, self.a]
    }
}

#[derive(Debug, Default)]
pub struct Size {
    pub w: f32,
    pub h: f32,
}

#[derive(Debug, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

pub type Pivot = Point;

#[derive(Component, Debug, Default)]
#[storage(VecStorage)]
pub struct Sprite {
    pub color: Color,
    pub size: Size,
    pub pivot: Pivot,
}
