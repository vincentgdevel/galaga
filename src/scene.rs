use crate::{SCENE_COLS, SCENE_ROWS};

pub type Scene = Vec<Vec<&'static str>>;

// create a new scene
pub fn new_scene() -> Scene {
    let scene = vec![vec![" "; SCENE_ROWS]; SCENE_COLS];
    return scene;
}

pub trait Drawable {
    fn draw(&self, scene: &mut Scene);
}
