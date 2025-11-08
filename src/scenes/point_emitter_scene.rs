use crate::particles::{
    emitter::Emitter,
    system::{ParticleStyle, ParticleSystem},
    utils::Direction,
};
use macroquad::prelude::*;

use super::{Scene, SceneName};

pub struct PointEmitterScene {
    particle_system: Option<ParticleSystem>,
}

impl PointEmitterScene {
    pub fn new() -> Self {
        Self {
            particle_system: None,
        }
    }
}

impl Scene for PointEmitterScene {
    fn start(&mut self) {
        let style = ParticleStyle::Color(RED);
        let bounding_box = (vec3(0.0, 0.0, 0.0), vec3(100.0, 100.0, 100.0));

        self.particle_system = Some(
            ParticleSystem::new()
                .point(
                    vec3(0.0, 0.0, 0.0),
                    Direction::Fixed(vec3(0.2, 0.0, 0.)),
                    0.3,
                )
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
