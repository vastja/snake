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
    let game = Game::new(16, 8);


    let stdout = stdout();
    let mut stdout = stdout.lock().into_raw_mode().unwrap();
    let mut stdin = async_stdin().bytes();

    write!(stdout,
        "{}{}",
        termion::clear::All, termion::cursor::Hide)
         .unwrap();
    
    loop {
        let input = stdin.next();
        if input.is_some() {
            let result = input.unwrap();
            if result.is_ok() {
                let input_key = result.unwrap();
                if input_key == b'q' {
                    break;
                }
            }
        }
        
        thread::sleep(Duration::from_millis(50));
        
        write!(stdout,
            "{}",
            termion::cursor::Goto(1, 1))
             .unwrap();
        
        write!(stdout, "{game}\r\n").unwrap();
        stdout.flush().unwrap();
    }
}