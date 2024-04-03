use std::collections::LinkedList;

#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct Pixel {
    pub x : u16,
    pub y : u16,
}

pub struct Snake {
    body : LinkedList<Pixel>,
    direction: Direction
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

    pub fn do_step(&mut self) {
        let mut next : Pixel = self.head();
        match self.direction {
            Direction::Up => next.y += 1,
            Direction::Right => next.x += 1,
            Direction::Down => next.y  -= 1,
            Direction::Left => next.x -= 1, 
        }
        self.body.push_front(next);
        self.body.pop_back();
    }

    pub fn head(&self) -> Pixel {
        (*self.body.front().unwrap()).clone()
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

    #[test_case(Direction::Up, Pixel { x : 1, y : 2 })]
    #[test_case(Direction::Right, Pixel { x : 2, y : 1 })]
    #[test_case(Direction::Down, Pixel { x : 1, y : 0 })]
    #[test_case(Direction::Left, Pixel { x : 0, y : 1 })]
    fn snake_step_change_head_correctly(direction : Direction, expected_head_position: Pixel) {
         let mut snake = Snake::new(Pixel { x: 1, y: 1}, direction);

         snake.do_step();

         assert_eq!(*snake.body.front().unwrap(), expected_head_position);
    }

    #[test]
    fn snake_step_remove_tail() {
        let mut snake = Snake::new(Pixel {x : 0, y : 0}, Direction::Right);

        snake.do_step();

        assert_eq!(snake.body.contains(&Pixel { x : 0, y : 0}), false);
    }
}