use crate::deck::card::Card;
use crate::game::betting::Play;
use crate::game::betting::Play::{Hit, Stand};
use crate::player;

#[derive(Debug)]
pub struct Dealer {
  cards: Vec<Card>,
}

impl Dealer {
  pub fn new() -> Dealer {
    Dealer { cards: Vec::new() }
  }

  pub fn turn(&mut self) -> Play {
    let total = player::total(&self.cards);

    if total >= 17 { Stand } else { Hit }
  }

  pub fn face_up_card(&mut self) -> &Card {
    self.cards.get(1).unwrap()
  }
  
  pub fn deal_card(&mut self, card: Card) {
    self.cards.push(card)
  }
}
