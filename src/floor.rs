use macroquad::prelude::*;

pub struct Floor {
    pub level: f32,
    pub bounce: f32,
    pub friction_coeff: f32,
}

impl Floor {
    pub fn draw(&self) {
        draw_line(0.0, self.level, screen_width(), self.level, 2.0, DARKGRAY);
    }
}
