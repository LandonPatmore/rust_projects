
#[cfg(test)]
mod tests {
  use crate::structs::{Card, Player};
  use crate::structs::CardRank::Three;
  use crate::structs::Play::Hit;
  use crate::structs::Suit::Hearts;
  use super::*;
  
  #[test]
  fn player_hits_properly() {
    let player = Player::new(String::from("Landon"), false);
    let dealer_card = Card {suit: Hearts, rank: Three};
    
    let turn = player.turn(&dealer_card);
    
    assert_eq!(turn, Hit);
  }
}