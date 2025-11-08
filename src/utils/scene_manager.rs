use crate::scenes::{Scene, SceneName, create_scene};

pub struct SceneManager {
    current_scene: Box<dyn Scene>,
    current_scene_name: SceneName,
}

impl SceneManager {
    pub fn new() -> Self {
        let initial_scene = SceneName::MainMenu;
        Self {
            current_scene: create_scene(initial_scene),
            current_scene_name: initial_scene,
        }
    }

    pub async fn update(&mut self) {
        if let Some(next_scene) = self.current_scene.update() {
            self.switch_scene(next_scene).await;
        }
    }

    pub fn draw(&mut self) {
        self.current_scene.draw();
    }

    async fn switch_scene(&mut self, next_scene: SceneName) {
        // Stop current scene
        self.current_scene.stop();

        // Switch to new scene
        self.current_scene_name = next_scene;
        self.current_scene = create_scene(next_scene);

        // Start the new scene
        self.current_scene.start();
    }

    pub fn start_current_scene(&mut self) {
        self.current_scene.start();
    }
}
