use macroquad::prelude::*;

pub struct Particle {
    pub position: Vec3,
    pub prev_position: Vec3,
    pub velocity: Vec3,
    pub energy: f32,
    pub size: f32,
}
