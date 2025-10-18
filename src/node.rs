use macroquad::prelude::*;

pub struct Node {
    pub mass: f32,
    pub pos: Vec2,
    pub velocity: Vec2,
}

impl Node {
    pub fn apply_velocity(&mut self) {
        self.pos += self.velocity
    }
}
