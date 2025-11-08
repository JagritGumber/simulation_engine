mod cube_emitter_scene;
mod menu_scene;
mod point_emitter_scene;
mod sphere_emitter_scene;

use macroquad::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SceneName {
    MainMenu,
    PointEmitter,
    CubeEmitter,
    SphereEmitter,
}

pub trait Scene {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self) -> Option<SceneName>;
    fn draw(&self);

    /// Draw a simple wireframe floor and grid to give a 3D room feel.
    /// Scenes can call `self.draw_room()` after setting a 3D camera.
    fn draw_room(&self) {
        use macroquad::prelude::*;

        let grid_size = 20.0; // half-size in world units
        let grid_step = 1.0; // spacing between grid lines
        let y = -5.0; // vertical position of the floor/grid

        // subtle grid color
        let grid_color = Color::new(0.7, 0.7, 0.7, 0.35);
        let axis_color = Color::new(1.0, 0.2, 0.2, 0.9);

        let mut x = -grid_size;
        while x <= grid_size {
            draw_line_3d(vec3(x, y, -grid_size), vec3(x, y, grid_size), grid_color);
            x += grid_step;
        }

        let mut z = -grid_size;
        while z <= grid_size {
            draw_line_3d(vec3(-grid_size, y, z), vec3(grid_size, y, z), grid_color);
            z += grid_step;
        }

        draw_line_3d(
            vec3(-grid_size, y, 0.0),
            vec3(grid_size, y, 0.0),
            axis_color,
        );
        draw_line_3d(
            vec3(0.0, y, -grid_size),
            vec3(0.0, y, grid_size),
            axis_color,
        );
    }

    /// Default helper to draw a Back button (and handle Escape key) for scenes.
    /// Returns Some(SceneName::MainMenu) when triggered, otherwise None.
    fn handle_back(&mut self) -> Option<SceneName> {
        use macroquad::{
            prelude::*,
            ui::{root_ui, widgets::Button},
        };

        if Button::new("Back")
            .position(vec2(10.0, 10.0))
            .ui(&mut root_ui())
        {
            return Some(SceneName::MainMenu);
        }

        if is_key_pressed(KeyCode::Escape) {
            return Some(SceneName::MainMenu);
        }

        None
    }
}

pub fn create_scene(name: SceneName) -> Box<dyn Scene> {
    match name {
        SceneName::MainMenu => Box::new(menu_scene::MenuScene::new()),
        SceneName::PointEmitter => Box::new(point_emitter_scene::PointEmitterScene::new()),
        SceneName::CubeEmitter => Box::new(cube_emitter_scene::CubeEmitterScene::new()),
        SceneName::SphereEmitter => Box::new(sphere_emitter_scene::SphereEmitterScene::new()),
    }
}

/// Simple orbital camera controller that supports:
/// - left mouse drag to orbit (rotate)
/// - right mouse drag to pan
/// - mouse wheel to zoom
pub struct CameraController {
    pub target: Vec3,
    pub yaw: f32,
    pub pitch: f32,
    pub distance: f32,
    last_mouse: Option<Vec2>,
    rotating: bool,
    panning: bool,
}

impl CameraController {
    pub fn new(target: Vec3, distance: f32) -> Self {
        Self {
            target,
            yaw: 0.0,
            pitch: 0.0,
            distance,
            last_mouse: None,
            rotating: false,
            panning: false,
        }
    }

    pub fn update(&mut self) {
        use macroquad::prelude::*;

        let mouse = mouse_position();
        let mouse_v = vec2(mouse.0, mouse.1);

        if is_mouse_button_pressed(MouseButton::Left) {
            self.rotating = true;
            self.last_mouse = Some(mouse_v);
        }
        if is_mouse_button_released(MouseButton::Left) {
            self.rotating = false;
            self.last_mouse = None;
        }

        if is_mouse_button_pressed(MouseButton::Right) {
            self.panning = true;
            self.last_mouse = Some(mouse_v);
        }
        if is_mouse_button_released(MouseButton::Right) {
            self.panning = false;
            self.last_mouse = None;
        }

        // handle dragging
        if self.rotating || self.panning {
            if let Some(prev) = self.last_mouse {
                let delta = mouse_v - prev;

                if self.rotating {
                    self.yaw += delta.x * 0.008;
                    self.pitch += -delta.y * 0.008;
                    self.pitch = self.pitch.clamp(-1.45, 1.45);
                }

                if self.panning {
                    let pixel_to_world = 0.008;
                    self.target.x += delta.x * pixel_to_world;
                    self.target.y += -delta.y * pixel_to_world;
                }

                self.last_mouse = Some(mouse_v);
            }
        }

        let wheel = mouse_wheel().1;
        if wheel != 0.0 {
            // wheel > 0 means up -> zoom in; smaller multiplier for gentler zoom
            let factor = 1.0 - wheel * 0.02;
            self.distance = (self.distance * factor).clamp(1.0, 200.0);
        }
    }

    pub fn camera(&self) -> Camera3D {
        use macroquad::prelude::*;

        let cos_pitch = self.pitch.cos();
        let pos = vec3(
            self.target.x + self.distance * cos_pitch * self.yaw.sin(),
            self.target.y + self.distance * self.pitch.sin(),
            self.target.z + self.distance * cos_pitch * self.yaw.cos(),
        );

        Camera3D {
            position: pos,
            target: self.target,
            up: vec3(0.0, 1.0, 0.0),
            ..Default::default()
        }
    }
}
