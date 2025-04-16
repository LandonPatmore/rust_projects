use crate::deck::card::{Card, CardRank};
use crate::player::Play::{Double, Hit, Stand};
use crate::player::player_data::PlayerData;
use crate::player::{Play, Player};
use crate::table::betting::Betting;
use rand::Rng;

struct Ai {
  player_data: PlayerData,
}

impl Player for Ai {
  fn new(
    name: String,
    is_dealer: bool,
    cash: u32,
  ) -> Self {
    Ai {
      player_data: PlayerData {
        name,
        is_dealer,
        cash,
        cards: Vec::new(),
      },
    }
  }

  fn bet(
    &mut self,
    betting: &Betting,
  ) -> u32 {
    let mut rand = rand::rng();

    let probability: f64 = rand.random();

    let bet = match probability > 0.75 {
      true => self.risky_bet(&betting),
      false => betting.min_bet,
    };

    self.player_data.cash -= bet;

    bet
  }

  // TODO: Need to take extra money on double
  fn turn(
    &self,
    visible_dealer_card: &Card,
  ) -> Play {
    if visible_dealer_card.value() == 11 {
      return if (self.total()) >= 17 { Stand } else { Hit };
    }

    let dealer_value = visible_dealer_card.value();

    match self.total() {
      8 => Hit, // Always hit if hand total is 8 or less
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
      _ => Hit,         // If player has less than 8, always hit
    }
  }
}

impl Ai {
  fn risky_bet(
    &self,
    betting: &Betting,
  ) -> u32 {
    let mut rand = rand::rng();

    let probability: f64 = rand.random();

    match probability {
      p if p > 0.95 => {
        if self.player_data.cash > betting.max_bet {
          betting.max_bet
        } else {
          self.player_data.cash
        }
      }
      p if p > 0.75 => ((self.player_data.cash as f64) * 0.75).round() as u32,
      p if p > 0.50 => ((self.player_data.cash as f64) * 0.50).round() as u32,
      _ => ((self.player_data.cash as f64) * 0.25).round() as u32,
    }
  }

  pub fn total(&self) -> usize {
    let initial_total = self
      .player_data
      .cards
      .iter()
      .fold(0, |acc, card| acc + card.value());
    let aces = self
      .player_data
      .cards
      .iter()
      .filter(|card| matches!(card.rank, CardRank::Ace))
      .count();

    initial_total - if initial_total > 21 { aces * 10 } else { 0 }
  }

  pub fn deal_card(
    &mut self,
    card: Card,
  ) {
    self.player_data.cards.push(card);
  }

  pub fn remove_cards(&mut self) -> Vec<Card> {
    let mut removed = Vec::new();

    loop {
      let removed_card = self.player_data.cards.pop();

      match removed_card {
        Some(card) => removed.push(card),
        None => break,
      }
    }

    removed
  }
}

// #[test]
// fn player_hits_properly_below_8() {
//   let mut player = Player::new(String::from("Landon"), false, 0);
//   player.add_card(&Card { suit: Diamonds, rank: Three });
//   player.add_card(&Card { suit: Spades, rank: Three });
//
//   let dealer_card = Card { suit: Hearts, rank: Three };
//
//   let turn = player.turn(&dealer_card);
//
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_hits_on_8() {
//   let mut player = Player::new(String::from("P1"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Four });
//   player.add_card(&Card { suit: Clubs, rank: Four });
//
//   let dealer_card = Card { suit: Spades, rank: Five };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_doubles_on_9_vs_6() {
//   let mut player = Player::new(String::from("P2"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Five });
//   player.add_card(&Card { suit: Clubs, rank: Four });
//
//   let dealer_card = Card { suit: Spades, rank: Six };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Double);
// }
//
// #[test]
// fn player_hits_on_9_vs_10() {
//   let mut player = Player::new(String::from("P3"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Five });
//   player.add_card(&Card { suit: Clubs, rank: Four });
//
//   let dealer_card = Card { suit: Spades, rank: Ten };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_doubles_on_10_vs_9() {
//   let mut player = Player::new(String::from("P4"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Six });
//   player.add_card(&Card { suit: Clubs, rank: Four });
//
//   let dealer_card = Card { suit: Spades, rank: Nine };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Double);
// }
//
// #[test]
// fn player_hits_on_10_vs_10() {
//   let mut player = Player::new(String::from("P5"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Six });
//   player.add_card(&Card { suit: Clubs, rank: Four });
//
//   let dealer_card = Card { suit: Spades, rank: King };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_always_doubles_on_11() {
//   let mut player = Player::new(String::from("P6"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Six });
//   player.add_card(&Card { suit: Clubs, rank: Five });
//
//   let dealer_card = Card { suit: Spades, rank: Queen };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Double);
// }
//
// #[test]
// fn player_stands_on_12_vs_4() {
//   let mut player = Player::new(String::from("P7"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Seven });
//   player.add_card(&Card { suit: Clubs, rank: Five });
//
//   let dealer_card = Card { suit: Spades, rank: Four };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Stand);
// }
//
// #[test]
// fn player_hits_on_12_vs_3() {
//   let mut player = Player::new(String::from("P8"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Seven });
//   player.add_card(&Card { suit: Clubs, rank: Five });
//
//   let dealer_card = Card { suit: Spades, rank: Three };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_stands_on_15_vs_5() {
//   let mut player = Player::new(String::from("P9"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Eight });
//   player.add_card(&Card { suit: Clubs, rank: Seven });
//
//   let dealer_card = Card { suit: Spades, rank: Five };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Stand);
// }
//
// #[test]
// fn player_hits_on_15_vs_10() {
//   let mut player = Player::new(String::from("P10"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Eight });
//   player.add_card(&Card { suit: Clubs, rank: Seven });
//
//   let dealer_card = Card { suit: Spades, rank: Queen };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
//
// #[test]
// fn player_stands_on_17_vs_ace() {
//   let mut player = Player::new(String::from("P11"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Ten });
//   player.add_card(&Card { suit: Clubs, rank: Seven });
//
//   let dealer_card = Card { suit: Spades, rank: Ace };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Stand);
// }
//
// #[test]
// fn player_hits_on_16_vs_ace() {
//   let mut player = Player::new(String::from("P12"), false, 0);
//   player.add_card(&Card { suit: Hearts, rank: Nine });
//   player.add_card(&Card { suit: Clubs, rank: Seven });
//
//   let dealer_card = Card { suit: Spades, rank: Ace };
//   let turn = player.turn(&dealer_card);
//   assert_eq!(turn, Hit);
// }
