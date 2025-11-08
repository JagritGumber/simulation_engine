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
            Emitter::Sphere {
                position,
                size,
                spawn_type,
            } => {
                let radius = *size;
                for _ in 0..count {
                    let spawn_pos = match spawn_type {
                        crate::particles::utils::Spawn::Volume => {
                            // random point inside sphere (uniform)
                            let dir = random_direction();
                            let r: f32 = rand::gen_range(0.0f32, 1.0f32).powf(1.0 / 3.0) * radius;
                            *position + dir * r
                        }
                        crate::particles::utils::Spawn::Surface => {
                            // random point on sphere surface
                            let dir = random_direction();
                            *position + dir * radius
                        }
                    };

                    // velocity: for surface emit outward from center, for volume use random
                    let velocity = match spawn_type {
                        crate::particles::utils::Spawn::Surface => {
                            (spawn_pos - *position).normalize() * 2.0
                        }
                        crate::particles::utils::Spawn::Volume => random_direction() * 2.0,
                    };

                    particles.push(Particle {
                        position: spawn_pos,
                        prev_position: spawn_pos,
                        velocity,
                        energy: 1.0,
                        size: 0.1,
                    });
                }
            }
            Emitter::Cube {
                position,
                size,
                spawn_type,
            } => {
                // treat `size` as full edge length
                let half = *size / 2.0;
                for _ in 0..count {
                    let spawn_pos = match spawn_type {
                        crate::particles::utils::Spawn::Volume => {
                            // random point inside cube
                            let x = rand::gen_range(-half, half);
                            let y = rand::gen_range(-half, half);
                            let z = rand::gen_range(-half, half);
                            *position + vec3(x, y, z)
                        }
                        crate::particles::utils::Spawn::Surface => {
                            // choose one of 6 faces uniformly
                            let face = rand::gen_range(0, 6);
                            let (x, y, z) = match face {
                                0 => (
                                    half,
                                    rand::gen_range(-half, half),
                                    rand::gen_range(-half, half),
                                ),
                                1 => (
                                    -half,
                                    rand::gen_range(-half, half),
                                    rand::gen_range(-half, half),
                                ),
                                2 => (
                                    rand::gen_range(-half, half),
                                    half,
                                    rand::gen_range(-half, half),
                                ),
                                3 => (
                                    rand::gen_range(-half, half),
                                    -half,
                                    rand::gen_range(-half, half),
                                ),
                                4 => (
                                    rand::gen_range(-half, half),
                                    rand::gen_range(-half, half),
                                    half,
                                ),
                                _ => (
                                    rand::gen_range(-half, half),
                                    rand::gen_range(-half, half),
                                    -half,
                                ),
                            };
                            *position + vec3(x, y, z)
                        }
                    };

                    // velocity: surface -> face normal, volume -> random
                    let velocity = match spawn_type {
                        crate::particles::utils::Spawn::Surface => {
                            // approximate normal from position relative to center
                            (spawn_pos - *position).normalize() * 2.0
                        }
                        crate::particles::utils::Spawn::Volume => random_direction() * 2.0,
                    };

                    particles.push(Particle {
                        position: spawn_pos,
                        prev_position: spawn_pos,
                        velocity,
                        energy: 1.0,
                        size: 0.1,
                    });
                }
            }
        }

        return particles;
    }
}
