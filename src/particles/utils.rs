use macroquad::prelude::*;

pub enum Direction {
    Fixed(Vec3),
    Random,
}

pub struct ParticleQuad(pub Vec3, pub Vec3, pub Vec3, pub Vec3);

pub enum Spawn {
    Volume,
    Surface,
}

pub fn random_direction() -> Vec3 {
    let x = rand::gen_range(-1.0, 1.0);
    let y = rand::gen_range(-1.0, 1.0);
    let z = rand::gen_range(-1.0, 1.0);
    vec3(x, y, z).normalize()
}
