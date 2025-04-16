use crate::deck::card::{Card, CardRank, Suit};
use rand::seq::SliceRandom;
use rand::Rng;
use strum::IntoEnumIterator;

#[derive(Debug)]
pub struct Deck {
  pub cards: Vec<Card>,
  pub discard: Vec<Card>,
  pub break_card_position: usize,
}

impl Deck {
  pub fn new(amount_of_decks: usize) -> Self {
    let mut rng = rand::rng();

    let mut cards: Vec<Card> = Vec::with_capacity(amount_of_decks * 52);

    for _ in 0..amount_of_decks {
      for rank in CardRank::iter() {
        for suit in Suit::iter() {
          cards.push(Card::new(suit, rank));
        }
      }
    }
    
    let length = cards.len();

    let mut deck = Deck {
      cards,
      discard: Vec::with_capacity(amount_of_decks * 52),
      break_card_position: rng.random_range(0..length),
    };
    
    deck.shuffle();
    
    deck
  }

  pub fn reset(&mut self) {
    let mut rng = rand::rng();
    self.cards.append(&mut self.discard);
    self.shuffle();
    self.break_card_position = rng.random_range(0..self.cards.len())
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
