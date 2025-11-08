mod cube_emitter_scene;
mod menu_scene;
mod point_emitter_scene;
mod sphere_emitter_scene;

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
