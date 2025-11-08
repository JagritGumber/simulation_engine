mod particles;
mod scenes;
mod utils;

use macroquad::prelude::*;
use utils::scene_manager::SceneManager;

#[macroquad::main("Physics Simulation Engine")]
async fn main() {
    let mut scene_manager = SceneManager::new();
    scene_manager.start_current_scene();

    loop {
        scene_manager.update().await;
        scene_manager.draw();

        next_frame().await;
    }
}
