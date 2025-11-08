use macroquad::ui::widgets::Button;
use macroquad::{prelude::*, ui::root_ui};

use super::{Scene, SceneName};

pub struct MenuScene {
    buttons: Vec<(SceneName, &'static str)>,
}

impl MenuScene {
    pub fn new() -> Self {
        Self {
            buttons: vec![(SceneName::PointEmitter, "Point Emitter")],
        }
    }
}

impl Scene for MenuScene {
    fn start(&mut self) {}
    fn stop(&mut self) {}

    fn update(&mut self) -> Option<SceneName> {
        let screen_center_x = screen_width() / 2.0;

        for (scene_name, button_text) in &self.buttons {
            if Button::new(*button_text)
                .position(vec2(screen_center_x, 100.0))
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
