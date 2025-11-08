use macroquad::ui::widgets::Button;
use macroquad::{prelude::*, ui::root_ui};

use super::{Scene, SceneName};

pub struct MenuScene {
    buttons: Vec<(SceneName, &'static str)>,
}

impl MenuScene {
    pub fn new() -> Self {
        Self {
            buttons: vec![
                (SceneName::PointEmitter, "Point Emitter"),
                (SceneName::CubeEmitter, "Cube Emitter"),
                (SceneName::SphereEmitter, "Sphere Emitter"),
            ],
        }
    }
}

impl Scene for MenuScene {
    fn start(&mut self) {}
    fn stop(&mut self) {}

    fn update(&mut self) -> Option<SceneName> {
        let screen_center_x = screen_width() / 2.0;
        let start_y = 100.0f32;
        let spacing = 50.0f32; // vertical spacing between buttons

        for (i, (scene_name, button_text)) in self.buttons.iter().enumerate() {
            let y = start_y + i as f32 * spacing;
            if Button::new(*button_text)
                .position(vec2(screen_center_x, y))
                .ui(&mut root_ui())
            {
                return Some(*scene_name);
            }
        }
        return None;
    }

    fn draw(&self) {
        clear_background(GRAY);
    }
}
