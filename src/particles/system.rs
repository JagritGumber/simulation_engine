use crate::particles::{
    emitter::Emitter,
    particle::Particle,
    utils::{Direction, Spawn},
};
use macroquad::prelude::*;

pub enum ParticleStyle {
    Texture { texture: Texture2D, color: Color },
    Color(Color),
}

pub struct ParticleSystem {
    style: Option<ParticleStyle>,
    // blend_mode: BlendMode,
    particles: Vec<Particle>,
    bounding_box: Option<(Vec3, Vec3)>,
    emitter: Emitter,
    spawn_per_update: usize,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            emitter: Emitter::Point {
                position: vec3(0.0, 0.0, 0.0),
                direction: Direction::Fixed(vec3(0.0, 0.0, 0.0)),
                spread: 0.3,
            },
            bounding_box: None,
            particles: vec![],
            style: None,
            spawn_per_update: 2,
        }
    }

    /// Configure how many particles to spawn per update() call.
    pub fn spawn_rate(mut self, per_update: usize) -> Self {
        self.spawn_per_update = per_update;
        self
    }

    pub fn point(mut self, position: Vec3, direction: Direction, spread: f32) -> Self {
        self.emitter = Emitter::Point {
            position,
            direction,
            spread,
        };
        self
    }

    pub fn cube(mut self, position: Vec3, size: f32, spawn_type: Spawn) -> Self {
        self.emitter = Emitter::Cube {
            position,
            size,
            spawn_type,
        };
        self
    }

    pub fn sphere(mut self, position: Vec3, size: f32, spawn_type: Spawn) -> Self {
        self.emitter = Emitter::Sphere {
            position,
            size,
            spawn_type,
        };
        self
    }

    pub fn style(mut self, style: ParticleStyle) -> Self {
        self.style = Some(style);
        self
    }

    pub fn bounding_box(mut self, bounding_box: (Vec3, Vec3)) -> Self {
        self.bounding_box = Some(bounding_box);
        self
    }

    pub fn draw(&self) {
        if let Some(style) = &self.style {
            match style {
                ParticleStyle::Color(color) => self.draw_color_particles(color),
                ParticleStyle::Texture { texture, color } => {
                    self.draw_texture_particles(texture, color)
                }
            }
        }
    }

    fn draw_color_particles(&self, color: &Color) {
        // draw a small 3D cross for each particle so depth is visible
        for particle in &self.particles {
            let p = particle.position;
            let s = particle.size * 0.5;
            // X axis line
            draw_line_3d(p - vec3(s, 0.0, 0.0), p + vec3(s, 0.0, 0.0), *color);
            // Y axis line
            draw_line_3d(p - vec3(0.0, s, 0.0), p + vec3(0.0, s, 0.0), *color);
            // Z axis line
            draw_line_3d(p - vec3(0.0, 0.0, s), p + vec3(0.0, 0.0, s), *color);
        }
    }

    fn draw_texture_particles(&self, texture: &Texture2D, color: &Color) {
        for particle in &self.particles {
            draw_texture_ex(
                texture,
                particle.position.x,
                particle.position.y,
                *color,
                DrawTextureParams {
                    dest_size: Some(vec2(particle.size, particle.size)),
                    ..Default::default()
                },
            );
        }
    }

    pub fn update(&mut self, delta: f32) {
        for particle in &mut self.particles {
            particle.prev_position = particle.position;
            particle.position += particle.velocity * delta;
            particle.energy -= delta; // reduction of timespan overtime
            particle.velocity.y -= 9.8 * delta; // gravity
        }

        let new_particles = self.emitter.spawn(self.spawn_per_update);

        self.particles.extend(new_particles);

        self.particles.retain(|particle| particle.energy > 0.0);
    }
}
