mod menu_scene;
mod point_emitter_scene;

#[derive(PartialEq, Clone, Copy)]
pub enum SceneName {
    MainMenu,
    PointEmitter,
}

pub trait Scene {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self) -> Option<SceneName>;
    fn draw(&self);
}

pub fn create_scene(name: SceneName) -> Box<dyn Scene> {
    match name {
        SceneName::MainMenu => Box::new(menu_scene::MenuScene::new()),
        SceneName::PointEmitter => Box::new(point_emitter_scene::PointEmitterScene::new()),
    }
}
