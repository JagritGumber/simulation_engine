use crate::particles::system::ParticleSystem;
use macroquad::prelude::*;

struct ParticleManager {
    systems: Vec<ParticleSystem>,
}

impl ParticleManager {
    pub fn new() -> Self {
        Self { systems: vec![] }
    }

    pub fn add_system(&mut self, system: ParticleSystem) -> usize {
        self.systems.push(system);
        return self.systems.len() - 1;
    }

    pub fn remove_system(&mut self, id: usize) {
        self.systems.swap_remove(id);
    }
}
