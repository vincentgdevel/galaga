use crate::enemies::Enemies;
use crate::scene::{Drawable, Scene};
use crate::shot::Shot;
use crate::{SCENE_COLS, SCENE_ROWS, SHOTS};
use std::time::Duration;

pub struct Ship {
    x: usize,
    y: usize,
    shots: Vec<Shot>,
}

impl Ship {
    pub fn new() -> Self {
        Self {
            x: SCENE_COLS / 2,
            y: SCENE_ROWS - 1,
            shots: vec![],
        }
    }

    pub fn move_left(&mut self) {
        if self.x > 0 {
            self.x -= 1;
        }
    }

    pub fn move_right(&mut self) {
        if self.x < SCENE_COLS - 1 {
            self.x += 1;
        }
    }

    pub fn shoot(&mut self) {
        if self.shots.len() < SHOTS {
            self.shots.push(Shot::new(self.x, self.y - 1));
        }
    }

    pub fn update(&mut self, delta: Duration) {
        for shot in self.shots.iter_mut() {
            shot.update(delta);
        }
        self.shots.retain(|shot| !shot.dead());
    }

    pub fn detect_hits(&mut self, enemies: &mut Enemies) {
        for shot in self.shots.iter_mut() {
            if !shot.exploding {
                if enemies.kill_galaga_at(shot.x, shot.y) {
                    shot.explode()
                }
            }
        }
    }
}

impl Drawable for Ship {
    fn draw(&self, scene: &mut Scene) {
        scene[self.x][self.y] = "A";
        for shot in self.shots.iter() {
            shot.draw(scene);
        }
    }
}
