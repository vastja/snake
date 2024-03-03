mod game;
use game::Game;

fn main() {
    println!("Snake in terminal.");
    let game = Game::new(32, 16);

    print!("{}", game);
}