use std::collections::LinkedList;
use game::Pixel;

pub struct Snake {
    body : LinkedList<Pixel>,
    pub direction: Direction
}

pub enum Direction {
    Up,
    Right,
    Down,
    Left
}

impl Snake {

    pub fn new(position: Pixel, direction: Direction) -> Snake {
        Snake {
            body : LinkedList::from([position]),
            direction : direction
        }
    }

    pub fn do_step<'a, 'b>(&'a mut self, grows : bool) -> Result<(), &'b str> {
        let mut next : Pixel = self.head();
        match self.direction {
            Direction::Down => next.y += 1,
            Direction::Right => next.x += 1,
            Direction::Up => next.y  -= 1,
            Direction::Left => next.x -= 1, 
        }

        if self.consists_of(next) {
            return Result::Err("Snake crashed into itself.");
        }

        self.body.push_front(next);
        if !grows {
            self.body.pop_back();
        }

        Result::Ok(())
    }

    pub fn head(&self) -> Pixel {
        (*self.body.front().unwrap()).clone()
    }

    pub fn tail(&self) -> Pixel {
        (*self.body.back().unwrap()).clone()
    }

    pub fn consists_of(&self, pixel : Pixel) -> bool {
        self.body.contains(&pixel)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn constructed_snake_has_size_eq_one() {
        let snake = Snake::new(Pixel { x : 0, y : 0}, Direction::Right);
        
        assert_eq!(snake.body.len(), 1);
    }

    #[test_case(Direction::Up, Pixel { x : 1, y : 0 })]
    #[test_case(Direction::Right, Pixel { x : 2, y : 1 })]
    #[test_case(Direction::Down, Pixel { x : 1, y : 2 })]
    #[test_case(Direction::Left, Pixel { x : 0, y : 1 })]
    fn snake_step_change_head_correctly(direction : Direction, expected_head_position: Pixel) {
        let mut snake = Snake::new(Pixel { x: 1, y: 1}, direction);

        let result = snake.do_step(false);

        assert_eq!(result.is_ok(), true);
        assert_eq!(snake.head(), expected_head_position);
    }

    #[test]
    fn snake_step_remove_tail() {
        let mut snake = Snake::new(Pixel {x : 0, y : 0}, Direction::Right);

        let result = snake.do_step(false);

        assert_eq!(result.is_ok(), true);
        assert_eq!(snake.consists_of(Pixel { x : 0, y : 0}), false);
    }
}