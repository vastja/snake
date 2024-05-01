mod game;
mod snake;

use game::Game;
extern crate termion;
extern crate test_case;

use termion::raw::IntoRawMode;
use termion::async_stdin;
use std::io::{Read, Write, stdout};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Snake in terminal.");
    let mut game = Game::new(32, 16);


    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
        "{}{}",
        termion::clear::All, termion::cursor::Hide)
         .unwrap();
    
    let mut timeElapsed = 0;

    loop {
        let input = stdin.next();
        if input.is_some() {
            let result = input.unwrap();
            if result.is_ok() {
                let input_key = result.unwrap();
                match input_key {
                    b'q' => break,
                    b'a' => game.change_direction(snake::Direction::Left),
                    b'w' => game.change_direction(snake::Direction::Up),
                    b's' => game.change_direction(snake::Direction::Down),
                    b'd' => game.change_direction(snake::Direction::Right),
                    _ => (),
                }
            }
        }
        
        thread::sleep(Duration::from_millis(50));
        timeElapsed += 50;

        write!(stdout,
            "{}",
            termion::cursor::Goto(1, 1))
             .unwrap();
        if 250 <= timeElapsed {
            timeElapsed = 0;
            game.update();
            if (game.is_game_over) {
                write!(stdout,
                    "{}",
                    termion::clear::All)
                     .unwrap();
                write!(stdout, "Game Over\r\n").unwrap();
            }
            else {
                write!(stdout, "{game}\r\n").unwrap();
            }
            stdout.flush().unwrap();
        }
    }
}