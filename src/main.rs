mod creature;
mod edge;
mod node;
mod utils;

use macroquad::prelude::*;

use crate::creature::Creature;

#[macroquad::main("Creatures")]
async fn main() {
    // To get a new seed each round
    rand::srand(miniquad::date::now() as u64);

    let mut c1 = Creature::new_rand(5);
    loop {
        clear_background(WHITE);
        c1.draw();
        c1.update_nodes();

        next_frame().await
    }
}
