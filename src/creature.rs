use macroquad::prelude::*;

use crate::{edge::Edge, node::Node, utils::cheap_normal};

pub struct Creature {
    pub color: Color,

    // Array of all nodes in the creature
    pub nodes: Vec<Node>,

    // Elasticity applies to every edge
    pub elasticity: f32,

    // Index pairs for edges that exist
    pub edges: Vec<Edge>,
}

impl Creature {
    pub fn new_rand(num_nodes: u32) -> Self {
        let color = Color::new(
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            rand::gen_range(0.0, 1.0),
            1.0,
        );

        let nodes: Vec<Node> = (0..num_nodes)
            .map(|_| Node {
                mass: rand::gen_range(1.0, 5.0),
                pos: Vec2::new(
                    rand::gen_range(0.0, screen_width()),
                    rand::gen_range(0.0, screen_height()),
                ),
                velocity: Vec2::ZERO,
            })
            .collect();

        let mut edges = vec![];
        for i in 0..num_nodes {
            for j in (i + 1)..num_nodes {
                edges.push(Edge {
                    start_index: i as usize,
                    end_index: j as usize,
                    rest_length: 0.5 * nodes[i as usize].pos.distance(nodes[j as usize].pos),
                });
            }
        }

        Creature {
            color,
            nodes,
            elasticity: 0.001,
            edges,
        }
    }

    pub fn update_nodes(&mut self) {
        let mut forces: Vec<Vec2> = vec![Vec2::ZERO; self.nodes.len()];

        for edge in &self.edges {
            let start_node = &self.nodes[edge.start_index];
            let end_node = &self.nodes[edge.end_index];

            let actual_length = start_node.pos.distance(end_node.pos);
            let direction = (end_node.pos - start_node.pos).normalize();
            let force_magnitude = self.elasticity * (actual_length - edge.rest_length);
            let force = direction * force_magnitude;

            forces[edge.start_index] += force;
            forces[edge.end_index] -= force;
        }

        for i in 0..self.nodes.len() {
            let acceleration = forces[i] / self.nodes[i].mass;
            self.nodes[i].velocity += acceleration;
            self.nodes[i].apply_velocity();
        }
    }

    pub fn draw(&mut self) {
        for edge in &self.edges {
            let start_node = &self.nodes[edge.start_index];
            let end_node = &self.nodes[edge.end_index];

            let actual_length = start_node.pos.distance(end_node.pos);
            let thickness = 5.0 * cheap_normal(0.01 * (actual_length - edge.rest_length));

            draw_line(
                start_node.pos.x,
                start_node.pos.y,
                end_node.pos.x,
                end_node.pos.y,
                thickness,
                BLACK,
            );
        }

        for node in &self.nodes {
            draw_circle(node.pos.x, node.pos.y, 5.0 * node.mass, self.color);
        }
    }
}
