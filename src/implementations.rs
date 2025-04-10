use crate::structs::{Betting, Card, CardRank, Deck, Play, Player, Seat, Suit, Table};
use rand::Rng;
use rand::prelude::SliceRandom;
use strum::IntoEnumIterator;
use crate::structs::Play::{Double, Hit, Stand};

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

impl Table {
  pub fn create(betting: Betting, deck: Deck, players: Vec<Player>) -> Table {
    let seats = players
      .into_iter()
      .map(|player| Seat {
        player: Some(player),
        current_bet: 0,
      })
      .collect();

    Table {
      betting,
      deck,
      seats,
    }
  }
}

impl Player {
  pub fn new(name: String, is_dealer: bool) -> Player {
    let mut rng = rand::rng();

    Player {
      name,
      is_dealer,
      cash: rng.random_range(100..=500),
      hand: Vec::new(),
    }
  }

  pub fn can_play(&self, betting: Betting) -> bool {
    self.cash >= betting.min_bet
  }

  pub fn bet(&mut self, betting: Betting) -> u32 {
    let mut rng = rand::rng();

    let should_min_bet: bool = rng.random::<f32>() > 0.50;

    let bet_amount = match should_min_bet {
      true => betting.min_bet,
      false => rng.random_range(
        betting.min_bet..if self.cash <= betting.max_bet {
          self.cash
        } else {
          betting.max_bet
        },
      ),
    };

    self.cash -= bet_amount;

    bet_amount
  }

  pub fn turn(self, dealer_visible_card: &Card) -> Play {
    if dealer_visible_card.value() == 11 {
      return if (self.total()) >= 17 { Stand } else { Hit };
    }

    let dealer_value = dealer_visible_card.value();

    match self.total() {
      8 => Hit,  // Always hit if hand total is 8 or less
      9 => match dealer_value {
        2..=6 => Double, // Double down on 9 if dealer has 2-6
        _ => Hit,        // Otherwise, just hit
      },
      10 => match dealer_value {
        2..=9 => Double, // Double down on 10 if dealer has 2-9
        _ => Hit,        // Otherwise, just hit
      },
      11 => Double, // Always double down on 11
      12 => match dealer_value {
        4..=6 => Stand, // Stand on 12 if dealer has 4-6
        _ => Hit,       // Otherwise, just hit
      },
      13..=16 => match dealer_value {
        2..=6 => Stand, // Stand if dealer has 2-6
        _ => Hit,       // Otherwise, hit
      },
      17..=21 => Stand, // Always stand on 17 or more
      _ => Hit, // If player has less than 8, always hit
    }
  }

  fn total(&self) -> usize {
    let initial_total = self.hand.iter().fold(0, |acc, card| acc + card.value());
    let aces = self.hand.iter().filter(|card| matches!(card.rank, CardRank::Ace)).count();

    initial_total - if initial_total > 21 { aces * 10 } else { 0 }
  }
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
