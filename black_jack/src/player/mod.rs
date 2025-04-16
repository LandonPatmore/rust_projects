use crate::deck::card::CardRank::Ace;
use crate::deck::card::Card;
use crate::player::BetError::{NotEnoughCash, TooLarge, TooSmall};
use crate::table::betting::Betting;
use std::rc::Rc;

#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub is_dealer: bool,
  pub cash: u32
}

impl Player {
  pub fn new(
    name: String,
    is_dealer: bool,
    cash: u32,
  ) -> Player {
    Player {
      name,
      is_dealer,
      cash
    }
  }

  pub fn can_play(
    &self,
    betting: &Betting,
  ) -> bool {
    self.cash >= betting.min_bet
  }

  pub fn bet(
    &mut self,
    betting: &Betting,
    bet: u32,
  ) -> Result<u32, BetError> {
    if bet > self.cash {
      return Err(NotEnoughCash);
    }

    if bet < betting.min_bet {
      return Err(TooSmall);
    }

    if bet > betting.max_bet {
      return Err(TooLarge);
    }

    self.cash -= bet;

    Ok(bet)
  }
}

#[derive(Debug, PartialEq)]
pub enum BetError {
  TooSmall,
  TooLarge,
  NotEnoughCash,
}

#[derive(Debug, PartialEq)]
pub enum Play {
  Hit,
  Stand,
  Double,
  Split,
}

pub mod ai;

#[cfg(test)]
mod tests {
  use crate::deck::card::Card;
  use crate::deck::card::CardRank::{Ace, Ten, Three, Two};
  use crate::deck::card::Suit::{Hearts, Spades};
  use crate::player::BetError::{NotEnoughCash, TooLarge, TooSmall};
  use crate::player::Player;
  use crate::table::betting::Betting;
  use std::rc::Rc;

  #[test]
  fn player_can_play() {
    let player = Player::new(String::from("P13"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 20,
    };

    let result = player.can_play(&betting);

    assert_eq!(result, true);
  }

  #[test]
  fn player_cannot_play() {
    let player = Player::new(String::from("P13"), false, 100);
    let betting = Betting {
      min_bet: 1000000,
      max_bet: 2000000,
    };

    let result = player.can_play(&betting);

    assert_eq!(result, false);
  }

  #[test]
  fn player_bets_valid_amount() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Player has enough cash and bet is within the valid range
    let result = player.bet(&betting, 30);
    assert_eq!(result, Ok(30));
    assert_eq!(player.cash, 70); // Player's cash should decrease by the bet amount
  }

  #[test]
  fn player_bets_too_small() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Bet is too small (below min_bet)
    let result = player.bet(&betting, 5);
    assert_eq!(result, Err(TooSmall));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_too_large() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Bet is too large (above max_bet)
    let result = player.bet(&betting, 60);
    assert_eq!(result, Err(TooLarge));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_insufficient_cash() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Bet exceeds the player's available cash
    let result = player.bet(&betting, 200);
    assert_eq!(result, Err(NotEnoughCash));
    assert_eq!(player.cash, 100); // Player's cash should remain unchanged
  }

  #[test]
  fn player_bets_exact_min_bet() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Bet is exactly equal to the min_bet
    let result = player.bet(&betting, 10);
    assert_eq!(result, Ok(10));
    assert_eq!(player.cash, 90); // Player's cash should decrease by the bet amount
  }

  #[test]
  fn player_bets_exact_max_bet() {
    let mut player = Player::new(String::from("Landon"), false, 100);
    let betting = Betting {
      min_bet: 10,
      max_bet: 50,
    };

    // Bet is exactly equal to the max_bet
    let result = player.bet(&betting, 50);
    assert_eq!(result, Ok(50));
    assert_eq!(player.cash, 50); // Player's cash should decrease by the bet amount
  }
}
