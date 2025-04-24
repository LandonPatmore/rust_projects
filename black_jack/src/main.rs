use crate::game::Game;

mod deck;
mod game;
mod player;
mod table;

fn main() {
  let mut game = Game::new();

  game.run_loop();
}
