use crate::game::Game;
use crate::player::Player;

mod deck;
mod game;
mod player;
mod table;

fn main() {
  let player1 = Player::new("Alice".to_string(), false, 100);
  let player2 = Player::new("Bob".to_string(), false, 100);
  let player3 = Player::new("Charlie".to_string(), false, 100);
  
  let mut game = Game::new();
  
  game.add_player(player1);
  game.add_player(player2);
  game.add_player(player3);
  
  game.initialize_round();
  
  dbg!(game);
}
