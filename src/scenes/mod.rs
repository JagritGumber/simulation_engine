mod menu_scene;

#[derive(PartialEq, Clone, Copy)]
pub enum SceneName {
    MainMenu,
}

pub trait Scene {
    fn start(&mut self);
    fn stop(&mut self);
    fn update(&mut self) -> Option<SceneName>;
    fn draw(&mut self);
}

pub fn create_scene(name: SceneName) -> Box<dyn Scene> {
    match name {
        SceneName::MainMenu => Box::new(menu_scene::MenuScene::new()),
    }
}
