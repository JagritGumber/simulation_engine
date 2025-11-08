use crate::particles::system::{ParticleStyle, ParticleSystem};
use crate::particles::utils::Spawn;
use macroquad::prelude::*;

use super::{CameraController, Scene, SceneName};

pub struct UnifiedEmitterScene {
    particle_system: Option<ParticleSystem>,
    camera: CameraController,

    // UI state
    emitter_index: usize, // 0: Point, 1: Cube, 2: Sphere
    spawn_volume: bool,
    size: f32,
    spread: f32,
    spawn_per_update: usize,
}

impl UnifiedEmitterScene {
    pub fn new() -> Self {
        Self {
            particle_system: None,
            camera: CameraController::new(vec3(0.0, 0.0, 0.0), 20.0),
            emitter_index: 0,
            spawn_volume: true,
            size: 4.0,
            spread: 0.3,
            spawn_per_update: 2,
        }
    }

    fn rebuild_system(&mut self) {
        let style = ParticleStyle::Color(WHITE);
        let bounding_box = (vec3(-50.0, -50.0, -50.0), vec3(50.0, 50.0, 50.0));

        let system = match self.emitter_index {
            0 => ParticleSystem::new().point(
                vec3(0.0, 0.0, 0.0),
                crate::particles::utils::Direction::Random,
                self.spread,
            ),
            1 => ParticleSystem::new().cube(
                vec3(0.0, 0.0, 0.0),
                self.size,
                if self.spawn_volume {
                    Spawn::Volume
                } else {
                    Spawn::Surface
                },
            ),
            _ => ParticleSystem::new().sphere(
                vec3(0.0, 0.0, 0.0),
                self.size,
                if self.spawn_volume {
                    Spawn::Volume
                } else {
                    Spawn::Surface
                },
            ),
        }
        .style(style)
        .bounding_box(bounding_box)
        .spawn_rate(self.spawn_per_update);

        self.particle_system = Some(system);
    }
}

impl Scene for UnifiedEmitterScene {
    fn start(&mut self) {
        self.rebuild_system();
    }

    fn stop(&mut self) {
        self.particle_system = None;
    }

    fn update(&mut self) -> Option<SceneName> {
        self.camera.update();

        let delta = get_frame_time();
        if let Some(system) = &mut self.particle_system {
            system.update(delta);
        }

        // UI using macroquad windows and widgets
        use macroquad::ui::{hash, root_ui, widgets};

        let panel_w = 320.0;
        let panel_h = 380.0;
        let panel_pos = vec2(screen_width() - (panel_w + 20.0), 20.0);

        // Use a window id that depends on the current screen size so the
        // window position won't be persisted from previous windowed/resized
        // states. This keeps the UI anchored to the top-right when toggling
        // fullscreen or resizing.
        widgets::Window::new(
            hash!(screen_width() as i32, screen_height() as i32),
            panel_pos,
            vec2(panel_w, panel_h),
        )
        .label("Emitter Editor")
        .titlebar(true)
        .ui(&mut *root_ui(), |ui| {
            ui.label(None, "Emitter Type:");
            // Stack buttons vertically to avoid overlap issues on different
            // window sizes. If you prefer a horizontal layout we can use
            // `ui.same_line(...)` with explicit sizes, but vertical is
            // robust for now.
            if ui.button(None, "Point") {
                if self.emitter_index != 0 {
                    self.emitter_index = 0;
                    self.rebuild_system();
                }
            }
            ui.separator();
            if ui.button(None, "Cube") {
                if self.emitter_index != 1 {
                    self.emitter_index = 1;
                    self.rebuild_system();
                }
            }
            ui.separator();
            if ui.button(None, "Sphere") {
                if self.emitter_index != 2 {
                    self.emitter_index = 2;
                    self.rebuild_system();
                }
            }

            ui.separator();

            // Spawn mode
            let spawn_label = if self.spawn_volume {
                "Volume"
            } else {
                "Surface"
            };
            ui.label(None, &format!("Spawn: {spawn_label}"));
            if ui.button(None, "Toggle Spawn Mode") {
                self.spawn_volume = !self.spawn_volume;
                self.rebuild_system();
            }

            ui.separator();

            // Size slider
            ui.label(None, "Size:");
            let size_range = 0.1f32..50.0f32;
            ui.slider(hash!(), "Size", size_range.clone(), &mut self.size);

            // Spread slider (only meaningful for point emitter)
            ui.label(None, "Spread (point emitter)");
            let spread_range = 0.0f32..3.0f32;
            ui.slider(hash!(), "Spread", spread_range.clone(), &mut self.spread);

            ui.separator();
            ui.label(None, "Spawn per update (particles/frame)");
            // small range up to 200 for performance; use integer step
            let spawn_min = 0usize as f32;
            let spawn_max = 200usize as f32;
            let mut spawn_val = self.spawn_per_update as f32;
            // ui.slider expects a Range<f32> (start..end), not RangeInclusive
            ui.slider(hash!(), "SpawnRate", spawn_min..spawn_max, &mut spawn_val);
            let new_spawn = spawn_val.round().clamp(spawn_min, spawn_max) as usize;
            if new_spawn != self.spawn_per_update {
                self.spawn_per_update = new_spawn;
                self.rebuild_system();
            }

            ui.separator();

            if ui.button(None, "Rebuild System") {
                self.rebuild_system();
            }
        });

        if let Some(scene) = self.handle_back() {
            return Some(scene);
        }

        None
    }

    fn draw(&self) {
        // Use a dark gray background so the UI (which uses light/white panels)
        // doesn't feel like a white rectangle on pure black.
        clear_background(Color::new(0.06, 0.06, 0.06, 1.0));

        // set interactive camera
        set_camera(&self.camera.camera());

        // draw room and particles
        self.draw_room();
        if let Some(system) = &self.particle_system {
            system.draw();
        }

        // UI is drawn in `update()` via macroquad windows, nothing else to draw here.

        set_default_camera();
    }
}
