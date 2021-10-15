use crate::scene::{Drawable, Scene};
use crate::{GALAGA_DELAY_MS, SCENE_COLS, SCENE_ROWS};
use rusty_time::timer::Timer;
use std::cmp::max;
use std::time::Duration;

pub struct Galaga {
    pub x: usize,
    pub y: usize,
}

pub struct Enemies {
    pub army: Vec<Galaga>,
    move_timer: Timer,
    direction: i32,
}

impl Enemies {
    pub fn new() -> Self {
        let mut army = Vec::new();
        for x in 0..SCENE_COLS {
            for y in 0..SCENE_ROWS {
                if x > 1 && x < SCENE_COLS - 2 && y > 0 && y < 9 && x % 2 == 0 && y % 2 == 0 {
                    army.push(Galaga { x, y });
                }
            }
        }
        Self {
            army,
            move_timer: Timer::from_millis(GALAGA_DELAY_MS),
            direction: 1,
        }
    }
    pub fn update(&mut self, delta: Duration) {
        self.move_timer.update(delta);
        if self.move_timer.ready {
            self.move_timer.reset();
            let mut downwards = false;

            // galaga moving left
            if self.direction == -1 {
                let min_x = self.army.iter().map(|galaga| galaga.x).min().unwrap_or(0);

                //left border
                if min_x == 0 {
                    self.direction = 1;
                    downwards = true
                }
            // galaga moving right
            } else {
                let max_x = self.army.iter().map(|galaga| galaga.x).max().unwrap_or(0);
                if max_x == SCENE_COLS - 1 {
                    self.direction = -1;
                    downwards = true
                }
            }

            if downwards {
                let new_duration = max(self.move_timer.duration.as_millis() - 250, 250);
                self.move_timer = Timer::from_millis(new_duration as u64);
                for galaga in self.army.iter_mut() {
                    galaga.y += 1;
                }
            } else {
                for galaga in self.army.iter_mut() {
                    galaga.x = ((galaga.x as i32) + self.direction) as usize;
                }
            }
        }
    }

    pub fn eliminated(&self) -> bool {
        self.army.is_empty()
            || self.army.iter().map(|galaga| galaga.y).max().unwrap_or(0) >= SCENE_ROWS - 1
    }

    pub fn kill_galaga_at(&mut self, x: usize, y: usize) -> bool {
        if let Some(i) = self
            .army
            .iter()
            .position(|galaga| (galaga.x == x) && (galaga.y == y))
        {
            self.army.remove(i);
            true
        } else {
            false
        }
    }
}

impl Drawable for Enemies {
    fn draw(&self, scene: &mut Scene) {
        for galaga in self.army.iter() {
            scene[galaga.x][galaga.y] = if (self.move_timer.time_left.as_secs_f32()
                / self.move_timer.duration.as_secs_f32())
                > 0.5
            {
                "x"
            } else {
                "+"
            }
        }
    }
}
