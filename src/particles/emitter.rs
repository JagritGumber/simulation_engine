use crate::particles::{
    particle::Particle,
    utils::{Direction, Spawn, random_direction},
};
use macroquad::prelude::*;

pub enum Emitter {
    Point {
        position: Vec3,
        direction: Direction,
        spread: f32,
    },
    Sphere {
        position: Vec3,
        size: f32,
        spawn_type: Spawn,
    },
    Cube {
        position: Vec3,
        size: f32,
        spawn_type: Spawn,
    },
}

impl Emitter {
    pub fn point(position: Vec3, direction: Direction, spread: f32) -> Self {
        Emitter::Point {
            position,
            direction,
            spread,
        }
    }

    pub fn spawn(&self, count: usize) -> Vec<Particle> {
        let mut particles = Vec::new();

        match self {
            Emitter::Point {
                position,
                direction,
                spread,
            } => {
                for _ in 0..count {
                    let base_velocity = match direction {
                        Direction::Fixed(vec) => vec.normalize(),
                        Direction::Random => random_direction(),
                    };

                    let random_offset = vec3(
                        rand::gen_range(-1.0, 1.0),
                        rand::gen_range(-1.0, 1.0),
                        rand::gen_range(-1.0, 1.0),
                    )
                    .normalize()
                        * *spread;

                    let velocity = (base_velocity + random_offset).normalize() * 2.0;

                    particles.push(Particle {
                        position: *position,
                        prev_position: *position,
                        velocity,
                        energy: 1.0,
                        size: 0.1,
                    })
                }
            }
            _ => {}
        }

        return particles;
    }
}
