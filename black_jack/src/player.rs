use rand::Rng;
use crate::deck::card::{Card, CardRank};
use crate::player::BetError::{NotEnoughCash, TooLarge, TooSmall};
use crate::player::Play::{Double, Hit, Stand};
use crate::table::Betting;

#[derive(Debug)]
pub struct Player<'a> {
  pub name: String,
  pub is_dealer: bool,
  pub cash: u32,
  pub hand: Vec<&'a Card>,
}

impl<'a> Player<'a> {
  pub fn new(name: String, is_dealer: bool, cash: u32) -> Player<'a> {
    let mut rng = rand::rng();

    Player {
      name,
      is_dealer,
      cash,
      hand: Vec::new(),
    }
  }

  pub fn can_play(&self, betting: Betting) -> bool {
    self.cash >= betting.min_bet
  }

  pub fn bet(&mut self, betting: Betting, bet: u32) -> Result<u32, BetError> {
    if bet > self.cash {
      return Err(NotEnoughCash)
    }
    
    if bet < betting.min_bet {
      return Err(TooSmall)
    }

    if bet > betting.max_bet {
      return Err(TooLarge)
    }
    
    self.cash -= bet;
    
    Ok(bet)
  }

  // pub fn turn(self, dealer_visible_card: &Card) -> Play {
  //   if dealer_visible_card.value() == 11 {
  //     return if (self.total()) >= 17 { Stand } else { Hit };
  //   }
  // 
  //   let dealer_value = dealer_visible_card.value();
  // 
  //   match self.total() {
  //     8 => Hit,  // Always hit if hand total is 8 or less
  //     9 => match dealer_value {
  //       2..=6 => Double, // Double down on 9 if dealer has 2-6
  //       _ => Hit,        // Otherwise, just hit
  //     },
  //     10 => match dealer_value {
  //       2..=9 => Double, // Double down on 10 if dealer has 2-9
  //       _ => Hit,        // Otherwise, just hit
  //     },
  //     11 => Double, // Always double down on 11
  //     12 => match dealer_value {
  //       4..=6 => Stand, // Stand on 12 if dealer has 4-6
  //       _ => Hit,       // Otherwise, just hit         
  //     },
  //     13..=16 => match dealer_value {
  //       2..=6 => Stand, // Stand if dealer has 2-6
  //       _ => Hit,       // Otherwise, hit
  //     },
  //     17..=21 => Stand, // Always stand on 17 or more
  //     _ => Hit, // If player has less than 8, always hit
  //   }
  // }

  fn total(&self) -> usize {
    let initial_total = self.hand.iter().fold(0, |acc, card| acc + card.value());
    let aces = self.hand.iter().filter(|card| matches!(card.rank, CardRank::Ace)).count();

    initial_total - if initial_total > 21 { aces * 10 } else { 0 }
  }

  pub fn add_card(&mut self, card: &'a Card) {
    self.hand.push(card);
  }
}

#[derive(Debug)]
#[derive(PartialEq)]
pub enum BetError {
  TooSmall,
  TooLarge,
  NotEnoughCash
}

#[derive(Debug, PartialEq)]
pub enum Play {
  Hit,
  Stand,
  Double,
  // Split TODO: later on
}

#[cfg(test)]
mod tests {
  use crate::deck::card::Card;
  use crate::deck::card::CardRank::{Ace, Eight, Five, Four, King, Nine, Queen, Seven, Six, Ten, Three};
  use crate::deck::card::Suit::{Clubs, Diamonds, Hearts, Spades};
  use crate::player::BetError::{NotEnoughCash, TooLarge, TooSmall};
  use crate::player::Play::{Double, Hit, Stand};
  use crate::player::Player;
  use crate::table::Betting;

  #[test]
  fn player_hits_properly_below_8() {
    let mut player = Player::new(String::from("Landon"), false, 0);
    player.add_card(&Card { suit: Diamonds, rank: Three });
    player.add_card(&Card { suit: Spades, rank: Three });

    let dealer_card = Card { suit: Hearts, rank: Three };

    let turn = player.turn(&dealer_card);

    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_hits_on_8() {
    let mut player = Player::new(String::from("P1"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Four });
    player.add_card(&Card { suit: Clubs, rank: Four });

    let dealer_card = Card { suit: Spades, rank: Five };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_doubles_on_9_vs_6() {
    let mut player = Player::new(String::from("P2"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Five });
    player.add_card(&Card { suit: Clubs, rank: Four });

    let dealer_card = Card { suit: Spades, rank: Six };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Double);
  }

  #[test]
  fn player_hits_on_9_vs_10() {
    let mut player = Player::new(String::from("P3"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Five });
    player.add_card(&Card { suit: Clubs, rank: Four });

    let dealer_card = Card { suit: Spades, rank: Ten };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_doubles_on_10_vs_9() {
    let mut player = Player::new(String::from("P4"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Six });
    player.add_card(&Card { suit: Clubs, rank: Four });

    let dealer_card = Card { suit: Spades, rank: Nine };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Double);
  }

  #[test]
  fn player_hits_on_10_vs_10() {
    let mut player = Player::new(String::from("P5"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Six });
    player.add_card(&Card { suit: Clubs, rank: Four });

    let dealer_card = Card { suit: Spades, rank: King };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_always_doubles_on_11() {
    let mut player = Player::new(String::from("P6"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Six });
    player.add_card(&Card { suit: Clubs, rank: Five });

    let dealer_card = Card { suit: Spades, rank: Queen };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Double);
  }

  #[test]
  fn player_stands_on_12_vs_4() {
    let mut player = Player::new(String::from("P7"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Seven });
    player.add_card(&Card { suit: Clubs, rank: Five });

    let dealer_card = Card { suit: Spades, rank: Four };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Stand);
  }

  #[test]
  fn player_hits_on_12_vs_3() {
    let mut player = Player::new(String::from("P8"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Seven });
    player.add_card(&Card { suit: Clubs, rank: Five });

    let dealer_card = Card { suit: Spades, rank: Three };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_stands_on_15_vs_5() {
    let mut player = Player::new(String::from("P9"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Eight });
    player.add_card(&Card { suit: Clubs, rank: Seven });

    let dealer_card = Card { suit: Spades, rank: Five };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Stand);
  }

  #[test]
  fn player_hits_on_15_vs_10() {
    let mut player = Player::new(String::from("P10"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Eight });
    player.add_card(&Card { suit: Clubs, rank: Seven });

    let dealer_card = Card { suit: Spades, rank: Queen };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_stands_on_17_vs_ace() {
    let mut player = Player::new(String::from("P11"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Ten });
    player.add_card(&Card { suit: Clubs, rank: Seven });

    let dealer_card = Card { suit: Spades, rank: Ace };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Stand);
  }

  #[test]
  fn player_hits_on_16_vs_ace() {
    let mut player = Player::new(String::from("P12"), false, 0);
    player.add_card(&Card { suit: Hearts, rank: Nine });
    player.add_card(&Card { suit: Clubs, rank: Seven });

    let dealer_card = Card { suit: Spades, rank: Ace };
    let turn = player.turn(&dealer_card);
    assert_eq!(turn, Hit);
  }

  #[test]
  fn player_can_play() {
    let player = Player::new(String::from("P13"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 20 };

    let result = player.can_play(betting);

    assert_eq!(result, true);
  }

  #[test]
  fn player_cannot_play() {
    let player = Player::new(String::from("P13"), false, 100);
    let betting = Betting { min_bet: 1000000, max_bet: 2000000 };

    let result = player.can_play(betting);

    assert_eq!(result, false);
  }

  #[test]
  fn player_bets_valid_amount() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Player has enough cash and bet is within the valid range
    let result = player.bet(betting, 30);
    assert_eq!(result, Ok(30));
    assert_eq!(player.cash, 70); // Player's cash should decrease by the bet amount
  }

  #[test]
  fn player_bets_too_small() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Bet is too small (below min_bet)
    let result = player.bet(betting, 5);
    assert_eq!(result, Err(TooSmall));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_too_large() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Bet is too large (above max_bet)
    let result = player.bet(betting, 60);
    assert_eq!(result, Err(TooLarge));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_insufficient_cash() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Bet exceeds the player's available cash
    let result = player.bet(betting, 200);
    assert_eq!(result, Err(NotEnoughCash));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_exact_min_bet() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Bet is exactly equal to the min_bet
    let result = player.bet(betting, 10);
    assert_eq!(result, Ok(10));
    assert_eq!(player.cash, 90); // Player's cash should decrease by the bet amount
  }

  #[test]
  fn player_bets_exact_max_bet() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting { min_bet: 10, max_bet: 50 };

    // Bet is exactly equal to the max_bet
    let result = player.bet(betting, 50);
    assert_eq!(result, Ok(50));
    assert_eq!(player.cash, 50); // Player's cash should decrease by the bet amount
  }
}