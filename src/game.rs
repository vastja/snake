use snake::{Snake, Pixel, Direction};
use termion::get_tty;
use std::fmt;

const BLOCK : char = '\u{2580}';

pub struct Game {
    board: Vec<char>,
    snake: Snake,
    width: u16,
    height: u16,
}

impl Game {
    pub fn new(width: u16, height: u16) -> Game {
        Game {
            // TODO
            board: Game::initialize_border(width as usize, height as usize),
            width,
            height,
            snake: Snake::new(Pixel { x : (width as f32 * 0.5) as u16, y : (height as f32 * 0.5) as u16}, Direction::Right)
        }
    }

    fn initialize_border(width: usize, height: usize) -> Vec<char> {
        let mut board = vec!['\u{0020}'; width * height];

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

    fn get_index(i: usize, j: usize, width: usize) -> usize {
        i * width + j
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
    use crate::snake::Pixel;

    use super::Game;

    #[test]
    fn after_creation_snake_is_in_the_middle() {
        let game = Game::new(100, 50);

        assert_eq!(game.snake.head(), Pixel {x: 50, y:25})
    } 
}