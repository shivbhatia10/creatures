mod creature;
mod edge;
mod floor;
mod node;
mod utils;

use macroquad::prelude::*;

use crate::{creature::Creature, floor::Floor};

const FLOOR_DIST_FROM_TOP_OF_SCREEN: f32 = 0.9;
const BOUNCE: f32 = 1.0;
const FRICTION_COEFF: f32 = 0.2;
const CREATURE_RADIUS: f32 = 250.0;

fn window_conf() -> Conf {
    Conf {
        window_title: "creatures".to_owned(),
        fullscreen: true,
        ..Default::default()
    }
}

#[macroquad::main(window_conf)]
async fn main() {
    // To get a new seed each round
    rand::srand(miniquad::date::now() as u64);

    let floor = Floor {
        level: screen_height() * FLOOR_DIST_FROM_TOP_OF_SCREEN,
        bounce: BOUNCE,
        friction_coeff: FRICTION_COEFF,
    };

    let mut c1 = Creature::new_rand(
        5,
        Vec2::new(screen_width() / 2.0, screen_height() * 1.5 / 2.0),
        CREATURE_RADIUS,
    );
    loop {
        clear_background(WHITE);
        c1.draw();
        c1.update_nodes(&floor);

        floor.draw();

        next_frame().await
    }
}
