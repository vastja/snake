extern crate rand;

use snake::{Snake, Direction};
use std::fmt;

const BLOCK : char = '\u{2580}';
const EMPTY : char = '\u{0020}';
const APPLE : char = '\u{002A}';

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Pixel {
    pub x : u16,
    pub y : u16,
}

pub struct Game {
    pub is_game_over : bool,
    board: Vec<char>,
    snake: Snake,
    width: u16,
    height: u16,
    apple: Option<Pixel>
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            // TODO
            is_game_over: false,
            board: Game::initialize_border(width as usize, height as usize),
            width,
            height,
            snake: Snake::new(Pixel { x : (width as f32 * 0.5) as u16, y : (height as f32 * 0.5) as u16}, Direction::Right),
            apple: None
        }
    }

    fn initialize_border(width: usize, height: usize) -> Vec<char> {
        let mut board = vec![EMPTY; width * height];

        for i in 0..width {
            board[i] = BLOCK;
        }

        for i in 1..height-1 {
            board[Game::get_index(i, 0, width)] = BLOCK;
            board[Game::get_index(i, width - 1, width)] = BLOCK;
        }

        for i in 0..width {
            board[Game::get_index(height - 1, i, width)] = BLOCK;
        }
        board
    }

    pub fn update_snake_position(&mut self) {
        let tail : Pixel = self.snake.tail();
        self.board[Game::get_index(tail.y as usize, tail.x as usize, self.width as usize)] = EMPTY;
        self.snake.do_step();
        let head : Pixel = self.snake.head();
        self.board[Game::get_index(head.y as usize, head.x as usize, self.width as usize)] = BLOCK;
    }

    pub fn update(&mut self) {
        if !self.is_game_over {
            let head = self.snake.head();
            if (head.x == 0 || head.y == 0 || head.x == self.width - 1|| head.y == self.height - 1) {
                self.is_game_over = true;
                return;
            }
            self.update_snake_position();
            if self.apple.is_none() {
                self.apple = Some(self.spawn_apple());
            }
        }
    }

    pub fn change_direction(&mut self, direction : Direction) {
        self.snake.direction = direction;
    }

    fn get_index(i: usize, j: usize, width: usize) -> usize {
        i * width + j
    }

    fn spawn_apple(&mut self) -> Pixel {
        // Todo - handle when the board is full
        let mut position = self.get_random_board_position();
        while self.snake.consists_of(position) {
            position = self.get_random_board_position();
        }
        self.board[Game::get_index(position.y as usize, position.x as usize, self.width as usize)] = APPLE;
        position
    } 

    fn get_random_board_position(&self) -> Pixel {
        let x = (rand::random::<f32>() * (self.width as f32 - 2.0)) + 1.0;
        let y = (rand::random::<f32>() * (self.height as f32 - 2.0)) + 1.0;
        Pixel { x: x as u16, y: y as u16}
    }   
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for i in 0..self.height {
            for j in 0..self.width {
                // TODO
                write!(f, "{}", self.board[Game::get_index(i as usize, j as usize, self.width as usize)].to_string())?;
            }
            write!(f, "\r\n")?;
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::Game;

    #[test]
    fn after_creation_snake_is_in_the_middle() {
        let game = Game::new(100, 50);

        assert_eq!(game.snake.head(), Pixel {x: 50, y:25})
    } 
}