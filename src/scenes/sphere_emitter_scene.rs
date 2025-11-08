use crate::particles::{system::ParticleSystem, utils::Spawn};
use macroquad::prelude::*;

use super::{Scene, SceneName};

pub struct SphereEmitterScene {
    particle_system: Option<ParticleSystem>,
}

impl SphereEmitterScene {
    pub fn new() -> Self {
        Self {
            particle_system: None,
        }
    }
}

impl Scene for SphereEmitterScene {
    fn start(&mut self) {
        let style = crate::particles::system::ParticleStyle::Color(BLUE);
        let bounding_box = (vec3(-10.0, -10.0, -10.0), vec3(10.0, 10.0, 10.0));

        self.particle_system = Some(
            ParticleSystem::new()
                .sphere(vec3(0.0, 0.0, 0.0), 4.0, Spawn::Volume)
                .style(style)
                .bounding_box(bounding_box),
        );
    }

    fn stop(&mut self) {
        self.particle_system = None;
    }

    fn update(&mut self) -> Option<SceneName> {
        let delta = get_frame_time();
        if let Some(system) = &mut self.particle_system {
            system.update(delta);
        }

        if let Some(scene) = self.handle_back() {
            return Some(scene);
        }

        None
    }

    fn draw(&self) {
        clear_background(WHITE);

        set_camera(&Camera3D {
            position: vec3(0.0, 0.0, 20.0),
            target: vec3(0., 0., 0.),
            up: vec3(0., 1., 0.),
            ..Default::default()
        });

        if let Some(system) = &self.particle_system {
            system.draw();
        }

        set_default_camera();
    }
}
