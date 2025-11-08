use macroquad::prelude::*;

use super::{Scene, SceneName};

pub struct MenuScene {
    buttons: Vec<(Rect, SceneName, &'static str)>,
}

impl MenuScene {
    pub fn new() -> Self {
        Self {
            buttons: vec![(
                Rect::new(100.0, 100.0, 200.0, 50.0),
                SceneName::MainMenu,
                "Start Simulation",
            )],
        }
    }
}

impl Scene for MenuScene {
    fn start(&mut self) {}
    fn stop(&mut self) {}

    fn update(&mut self) -> Option<SceneName> {
        if is_mouse_button_pressed(MouseButton::Left) {
            let mouse_pos = mouse_position();

            for (rect, scene, _) in &self.buttons {
                if rect.contains(vec2(mouse_pos.0, mouse_pos.1)) {
                    return Some(*scene);
                }
            }
        }
        None
    }

    fn draw(&mut self) {
        clear_background(GRAY);

        for (rect, _, text) in &self.buttons {
            draw_rectangle(rect.x, rect.y, rect.w, rect.h, WHITE);
            draw_text(text, rect.x + 10.0, rect.y + 30.0, 20.0, BLACK);
        }

        draw_text("Select a Scene:", 100.0, 50.0, 30.0, BLACK);
    }
}
