use crate::particles::{
    system::{ParticleStyle, ParticleSystem},
    utils::Direction,
};
use macroquad::prelude::*;

use super::{CameraController, Scene, SceneName};

pub struct PointEmitterScene {
    particle_system: Option<ParticleSystem>,
    camera: CameraController,
}

impl PointEmitterScene {
    pub fn new() -> Self {
        Self {
            particle_system: None,
            camera: CameraController::new(vec3(0.0, 0.0, 0.0), 20.0),
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
                    Direction::Fixed(vec3(0.0, 0.0, 0.2)),
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
        // update camera from mouse
        self.camera.update();

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

        // set the interactive camera
        set_camera(&self.camera.camera());

        // draw a simple wireframe floor/grid for 3D context
        self.draw_room();

        if let Some(system) = &self.particle_system {
            system.draw();
        }

        set_default_camera();
    }
}
