use std::fmt;

const BLOCK : char = '\u{2580}';

pub struct Game {
    width: usize,
    height: usize,
}

impl Game {
    pub fn new(width: usize, height: usize) -> Game {
        Game {
            width,
            height,
        }
    }
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        for _i in 0..self.width {
            write!(f, "{BLOCK}")?;
        }

        writeln!(f)?;
        for _i in 2..self.height {
            write!(f, "{BLOCK}")?;
            for _j in 2..self.width {
               write!(f, " ")?;
            }
            writeln!(f, "{BLOCK}")?;
        }

        for _i in 0..self.width {
            write!(f,"{BLOCK}")?;
        }
        Ok(()) 
    }
}