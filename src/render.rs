use crate::scene::Scene;
use crossterm::cursor::MoveTo;
use crossterm::terminal::{Clear, ClearType};
use crossterm::QueueableCommand;
use std::io::{Stdout, Write};

pub fn render(stdout: &mut Stdout, last_scene: &Scene, curr_scene: &Scene, init: bool) {
    if init {
        stdout.queue(Clear(ClearType::All)).unwrap();
    }

    for (x, col) in curr_scene.iter().enumerate() {
        for (y, s) in col.iter().enumerate() {
            if *s != last_scene[x][y] || init {
                stdout.queue(MoveTo(x as u16, y as u16)).unwrap();
                print!("{}", *s)
            }
        }
    }
    stdout.flush().unwrap();
}
