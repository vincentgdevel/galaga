use crossterm::cursor::{Hide, Show};
use crossterm::event::{Event, KeyCode};
use crossterm::terminal::{EnterAlternateScreen, LeaveAlternateScreen};
use crossterm::{event, terminal, ExecutableCommand};
use galaga::enemies::Enemies;
use galaga::render::render;
use galaga::scene::{new_scene, Drawable};
use galaga::ship::Ship;
use std::error::Error;
use std::sync::mpsc;
use std::time::{Duration, Instant};
use std::{io, thread};

fn main() -> Result<(), Box<dyn Error>> {
    let mut stdout = io::stdout();

    //initialize
    terminal::enable_raw_mode()?; // enable input capture
    stdout.execute(EnterAlternateScreen)?; //enter game screen
    stdout.execute(Hide)?; //hide cursor

    //render channel
    let (rtx, rrx) = mpsc::channel();

    let render_handle = thread::spawn(move || {
        let mut last_scene = new_scene();
        let mut stdout = io::stdout();
        render(&mut stdout, &last_scene, &last_scene, true);
        loop {
            let curr_scene = match rrx.recv() {
                Ok(x) => x,
                Err(_) => break,
            };
            render(&mut stdout, &last_scene, &curr_scene, false);
            last_scene = curr_scene;
        }
    });

    //game loop
    let mut ship = Ship::new();
    let mut instant = Instant::now();
    let mut enemies = Enemies::new();

    'game_loop: loop {
        let delta = instant.elapsed();
        instant = Instant::now();

        let mut curr_scene = new_scene();

        //wait for event (key)
        while event::poll(Duration::default())? {
            //key is pressed
            if let Event::Key(key) = event::read()? {
                match key.code {
                    KeyCode::Left => ship.move_left(),
                    KeyCode::Right => ship.move_right(),
                    KeyCode::Char(' ') => ship.shoot(),
                    KeyCode::Esc => {
                        break 'game_loop;
                    }
                    _ => {}
                }
            }
        }

        //update
        ship.update(delta);
        enemies.update(delta);
        ship.detect_hits(&mut enemies);

        //draw and render
        let drawables: Vec<&dyn Drawable> = vec![&ship, &enemies];
        for drawable in drawables {
            drawable.draw(&mut curr_scene)
        }
        let _ = rtx.send(curr_scene);
        thread::sleep(Duration::from_millis(10));

        //win/lose check
        if enemies.eliminated() {
            break 'game_loop;
        }
    }

    //cleanup
    drop(rtx);
    render_handle.join().unwrap();
    stdout.execute(Show)?; //show cursor
    stdout.execute(LeaveAlternateScreen)?; //leave game screen
    terminal::disable_raw_mode()?; //disable input capture

    Ok(())
}
