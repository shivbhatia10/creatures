use macroquad::prelude::*;

use crate::floor::Floor;

pub struct Node {
    pub mass: f32,
    pub pos: Vec2,
    pub velocity: Vec2,
}

impl Node {
    pub fn apply_gravity(&mut self, gravity_acceleration: f32) {
        self.velocity += Vec2::new(0.0, gravity_acceleration);
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.velocity += force / self.mass;
    }

    pub fn interact_with_floor(&mut self, floor: &Floor) {
        if self.pos.y >= floor.level {
            self.pos.y = floor.level;
            if self.velocity.y > 0.0 {
                self.velocity.y *= -floor.bounce; // bounce (0.0 = no bounce, 1.0 = perfect)
            }
            // friction opposes horizontal motion
            self.velocity.x *= 1.0 - floor.friction_coeff;
        }
    }

    pub fn apply_velocity(&mut self) {
        self.pos += self.velocity
    }
}
