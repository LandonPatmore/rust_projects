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
  pub fn generate(amount_of_decks: usize) -> Self {
    let mut rng = rand::rng();

    let mut cards: Vec<Card> = Vec::with_capacity(amount_of_decks * 52);

    for _ in 0..amount_of_decks {
      for rank in CardRank::iter() {
        for suit in Suit::iter() {
          cards.push(Card {
            suit,
            rank,
          });
        }
      }
    }

    // TODO: Share this somehow...
    cards.shuffle(&mut rng);
    let length = cards.len();

    Deck {
      cards,
      discard: Vec::with_capacity(amount_of_decks * 52),
      // TODO: Share this somehow...
      break_card_position: rng.random_range(0..length),
    }
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
}

pub mod card {
  use strum_macros::EnumIter;

  #[derive(Debug)]
  pub struct Card {
    pub suit: Suit,
    pub rank: CardRank,
  }

  #[derive(Debug, EnumIter, Copy, Clone)]
  pub enum Suit {
    Spades,
    Hearts,
    Diamonds,
    Clubs,
  }

  #[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone)]
  pub enum CardRank {
    Ace,
    Two,
    Three,
    Four,
    Five,
    Six,
    Seven,
    Eight,
    Nine,
    Ten,
    Jack,
    Queen,
    King,
  }

  impl Card {
    pub fn value(&self) -> usize {
      match self.rank {
        CardRank::Ace => 11,
        CardRank::Two => 2,
        CardRank::Three => 3,
        CardRank::Four => 4,
        CardRank::Five => 5,
        CardRank::Six => 6,
        CardRank::Seven => 7,
        CardRank::Eight => 8,
        CardRank::Nine => 9,
        _ => 10
      }
    }
  }
}