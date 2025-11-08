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
    start_color: Color,
    end_color: Color,
    // HSV cached values for interactive picker
    start_h: f32,
    start_s: f32,
    start_v: f32,
    end_h: f32,
    end_s: f32,
    end_v: f32,
    show_color_picker: bool,
    picker_for_start: bool,
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
            start_color: WHITE,
            end_color: RED,
            start_h: 0.0,
            start_s: 0.0,
            start_v: 1.0,
            end_h: 0.0,
            end_s: 1.0,
            end_v: 1.0,
            show_color_picker: false,
            picker_for_start: true,
        }
    }

    fn rebuild_system(&mut self) {
        let style = ParticleStyle::ColorGradient(self.start_color, self.end_color);
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

// Helper: convert HSV (h:0..1, s:0..1, v:0..1) to RGB Color
fn hsv_to_rgb(h: f32, s: f32, v: f32) -> Color {
    let i = (h * 6.0).floor();
    let f = h * 6.0 - i;
    let p = v * (1.0 - s);
    let q = v * (1.0 - f * s);
    let t = v * (1.0 - (1.0 - f) * s);
    let (r, g, b) = match i as i32 % 6 {
        0 => (v, t, p),
        1 => (q, v, p),
        2 => (p, v, t),
        3 => (p, q, v),
        4 => (t, p, v),
        _ => (v, p, q),
    };
    Color::new(r, g, b, 1.0)
}

// Helper: convert RGB Color to HSV (h,s,v) where h in 0..1
fn rgb_to_hsv(c: Color) -> (f32, f32, f32) {
    let r = c.r;
    let g = c.g;
    let b = c.b;
    let max = r.max(g).max(b);
    let min = r.min(g).min(b);
    let v = max;
    let d = max - min;
    let s = if max == 0.0 { 0.0 } else { d / max };
    let mut h = 0.0;
    if d != 0.0 {
        if max == r {
            h = (g - b) / d + if g < b { 6.0 } else { 0.0 };
        } else if max == g {
            h = (b - r) / d + 2.0;
        } else {
            h = (r - g) / d + 4.0;
        }
        h /= 6.0;
    }
    (h, s, v)
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

        let panel_w = 420.0;
        let panel_h = 420.0;
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
            ui.label(None, "Start color - Presets / HSV");
            // Quick presets for start color (small grid to avoid overlap)
            ui.label(None, "Presets:");
            if ui.button(None, "White") {
                self.start_color = WHITE;
                let (h, s, v) = rgb_to_hsv(self.start_color);
                self.start_h = h;
                self.start_s = s;
                self.start_v = v;
                self.rebuild_system();
            }
            if ui.button(None, "Red") {
                self.start_color = RED;
                let (h, s, v) = rgb_to_hsv(self.start_color);
                self.start_h = h;
                self.start_s = s;
                self.start_v = v;
                self.rebuild_system();
            }
            ui.separator();
            if ui.button(None, "Green") {
                self.start_color = GREEN;
                let (h, s, v) = rgb_to_hsv(self.start_color);
                self.start_h = h;
                self.start_s = s;
                self.start_v = v;
                self.rebuild_system();
            }
            if ui.button(None, "Blue") {
                self.start_color = BLUE;
                let (h, s, v) = rgb_to_hsv(self.start_color);
                self.start_h = h;
                self.start_s = s;
                self.start_v = v;
                self.rebuild_system();
            }
            if ui.button(None, "Pick...") {
                self.show_color_picker = true;
                self.picker_for_start = true;
            }

            // HSV sliders for precise selection
            let (mut sh, mut ss, mut sv) = rgb_to_hsv(self.start_color);
            ui.label(None, "Hue");
            ui.slider(hash!(), "Start Hue", 0.0f32..1.0f32, &mut sh);
            ui.label(None, "Sat");
            ui.slider(hash!(), "Start Sat", 0.0f32..1.0f32, &mut ss);
            ui.label(None, "Val");
            ui.slider(hash!(), "Start Val", 0.0f32..1.0f32, &mut sv);
            let new_start = hsv_to_rgb(sh, ss, sv);
            if new_start != self.start_color {
                self.start_color = new_start;
                self.start_h = sh;
                self.start_s = ss;
                self.start_v = sv;
                self.rebuild_system();
            }

            ui.separator();
            ui.label(None, "End color - Presets / HSV");
            ui.label(None, "Presets:");
            if ui.button(None, "Red") {
                self.end_color = RED;
                let (h, s, v) = rgb_to_hsv(self.end_color);
                self.end_h = h;
                self.end_s = s;
                self.end_v = v;
                self.rebuild_system();
            }
            if ui.button(None, "Yellow") {
                self.end_color = YELLOW;
                let (h, s, v) = rgb_to_hsv(self.end_color);
                self.end_h = h;
                self.end_s = s;
                self.end_v = v;
                self.rebuild_system();
            }
            ui.separator();
            if ui.button(None, "Cyan") {
                self.end_color = Color::new(0.0, 1.0, 1.0, 1.0);
                let (h, s, v) = rgb_to_hsv(self.end_color);
                self.end_h = h;
                self.end_s = s;
                self.end_v = v;
                self.rebuild_system();
            }
            if ui.button(None, "Pick...") {
                self.show_color_picker = true;
                self.picker_for_start = false;
            }

            let (mut eh, mut es, mut ev) = rgb_to_hsv(self.end_color);
            ui.label(None, "Hue");
            ui.slider(hash!(), "End Hue", 0.0f32..1.0f32, &mut eh);
            ui.label(None, "Sat");
            ui.slider(hash!(), "End Sat", 0.0f32..1.0f32, &mut es);
            ui.label(None, "Val");
            ui.slider(hash!(), "End Val", 0.0f32..1.0f32, &mut ev);
            let new_end = hsv_to_rgb(eh, es, ev);
            if new_end != self.end_color {
                self.end_color = new_end;
                self.end_h = eh;
                self.end_s = es;
                self.end_v = ev;
                self.rebuild_system();
            }

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

        // Interactive handling for the visual color picker (must be in update where &mut self is available)
        if self.show_color_picker {
            let panel_w = 320.0;
            let panel_pos = vec2(screen_width() - (panel_w + 20.0), 20.0);
            let picker_w = 220.0;
            let mut picker_x = panel_pos.x - picker_w - 10.0;
            // If the picker would be off-screen on the left, move it to the right of the panel
            if picker_x < 8.0 {
                picker_x = panel_pos.x + panel_w + 10.0;
            }
            let picker_y = panel_pos.y;
            let sv_size = 140.0;
            let sv_x = picker_x + 10.0;
            let sv_y = picker_y + 10.0;
            let hue_x = sv_x + sv_size + 8.0;
            let hue_y = sv_y;
            let hue_w = 16.0;
            let hue_h = sv_size;

            let (mx, my) = mouse_position();
            let mouse_v = vec2(mx, my);
            if is_mouse_button_down(MouseButton::Left) {
                // SV interaction
                if mouse_v.x >= sv_x
                    && mouse_v.x <= sv_x + sv_size
                    && mouse_v.y >= sv_y
                    && mouse_v.y <= sv_y + sv_size
                {
                    let s = ((mouse_v.x - sv_x) / sv_size).clamp(0.0, 1.0);
                    let v = (1.0 - (mouse_v.y - sv_y) / sv_size).clamp(0.0, 1.0);
                    if self.picker_for_start {
                        self.start_s = s;
                        self.start_v = v;
                        self.start_color = hsv_to_rgb(self.start_h, self.start_s, self.start_v);
                    } else {
                        self.end_s = s;
                        self.end_v = v;
                        self.end_color = hsv_to_rgb(self.end_h, self.end_s, self.end_v);
                    }
                    self.rebuild_system();
                }

                // Hue interaction
                if mouse_v.x >= hue_x
                    && mouse_v.x <= hue_x + hue_w
                    && mouse_v.y >= hue_y
                    && mouse_v.y <= hue_y + hue_h
                {
                    let hh = ((mouse_v.y - hue_y) / hue_h).clamp(0.0, 1.0);
                    if self.picker_for_start {
                        self.start_h = hh;
                        self.start_color = hsv_to_rgb(self.start_h, self.start_s, self.start_v);
                    } else {
                        self.end_h = hh;
                        self.end_color = hsv_to_rgb(self.end_h, self.end_s, self.end_v);
                    }
                    self.rebuild_system();
                }
            }
        }

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

        // Draw color picker overlay if requested (visual only)
        if self.show_color_picker {
            // compute picker rect near the panel
            let panel_w = 420.0;
            let panel_pos = vec2(screen_width() - (panel_w + 20.0), 20.0);
            let picker_w = 220.0;
            let picker_h = 200.0;
            let mut picker_x = panel_pos.x - picker_w - 10.0;
            if picker_x < 8.0 {
                picker_x = panel_pos.x + panel_w + 10.0;
            }
            let picker_y = panel_pos.y;
            // background
            draw_rectangle(
                picker_x,
                picker_y,
                picker_w,
                picker_h,
                Color::new(0.12, 0.12, 0.12, 0.95),
            );

            // SV square
            let sv_size = 140.0;
            let sv_x = picker_x + 10.0;
            let sv_y = picker_y + 10.0;
            let hue = if self.picker_for_start {
                self.start_h
            } else {
                self.end_h
            };
            // draw per-pixel-ish SV square (coarse grid to keep perf ok)
            let steps = 120u32;
            let cell = sv_size / steps as f32;
            for ix in 0..steps {
                for iy in 0..steps {
                    let s = ix as f32 / (steps as f32 - 1.0);
                    let v = 1.0 - (iy as f32 / (steps as f32 - 1.0));
                    let col = hsv_to_rgb(hue, s, v);
                    draw_rectangle(
                        sv_x + ix as f32 * cell,
                        sv_y + iy as f32 * cell,
                        cell,
                        cell,
                        col,
                    );
                }
            }

            // Hue bar on the right
            let hue_x = sv_x + sv_size + 8.0;
            let hue_y = sv_y;
            let hue_w = 16.0;
            let hue_h = sv_size;
            let hue_steps = 120u32;
            for i in 0..hue_steps {
                let hh = i as f32 / (hue_steps as f32 - 1.0);
                let c = hsv_to_rgb(hh, 1.0, 1.0);
                draw_rectangle(
                    hue_x,
                    hue_y + i as f32 * (hue_h / hue_steps as f32),
                    hue_w,
                    hue_h / hue_steps as f32,
                    c,
                );
            }
        }
        // Draw color swatches near the window so the selected colors are obvious.
        // Calculate same panel position as in update().
        let panel_w = 320.0;
        let panel_pos = vec2(screen_width() - (panel_w + 20.0), 20.0);
        // small swatches inside the panel header area
        let sw_x = panel_pos.x + panel_w - 80.0;
        let sw_y = panel_pos.y + 10.0;
        draw_rectangle(sw_x, sw_y, 28.0, 28.0, self.start_color);
        draw_rectangle(sw_x + 34.0, sw_y, 28.0, 28.0, self.end_color);

        // UI is drawn in `update()` via macroquad windows, nothing else to draw here.

        set_default_camera();
    }
}
