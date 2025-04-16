use crate::deck::card::{Card, CardRank, Suit};
use rand::seq::SliceRandom;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
  pub cards: Vec<Card>,
  pub discard: Vec<Card>
}

impl Deck {
  pub fn new(amount_of_decks: usize) -> Self {
    let mut cards: Vec<Card> = Vec::with_capacity(amount_of_decks * 52);

    for _ in 0..amount_of_decks {
      for rank in CardRank::iter() {
        for suit in Suit::iter() {
          cards.push(Card::new(suit, rank));
        }
      }
    }

    let mut deck = Deck {
      cards,
      discard: Vec::with_capacity(amount_of_decks * 52)
    };
    
    deck.shuffle();
    
    deck
  }

  pub fn reset(&mut self) {
    self.cards.append(&mut self.discard);
    self.shuffle();
  }

  fn shuffle(&mut self) {
    let mut rng = rand::rng();
    self.cards.shuffle(&mut rng);
  }

  fn deal_card(&mut self) -> Option<Card> {
    self.cards.pop()
  }
}

pub mod card;
