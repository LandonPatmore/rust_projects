use crate::table::betting::{BetError, Betting, Play};
use std::fmt::Debug;
use std::io;
use strum::IntoEnumIterator;
use crate::deck::card::Card;

#[derive(Debug)]
pub struct PlayerData {
  name: String,
  role: Role,
  cash: u32,
  current_bet: u32,
}

#[derive(Debug, PartialEq, Eq)]
enum Role {
  Player,
  Dealer,
}

#[derive(Debug)]
struct PlayerHuman {
  player: PlayerData,
}

trait PlayerActions {
  fn turn(
    &mut self,
    turn_type: TurnType,
    cards: &[Card],
  ) -> Result<Play, BetError>;
}

impl PlayerActions for PlayerHuman {
  fn turn(
    &mut self,
    turn_type: TurnType,
    cards: &[Card],
  ) -> Result<Play, BetError> {
    
    !todo!()
  }
}

#[derive(Debug)]
struct PlayerAi {
  player: PlayerData,
}

impl PlayerActions for PlayerAi {
  fn turn(
    &mut self,
    turn_type: TurnType,
    cards: &[Card],
  ) -> Result<Play, BetError> {
    todo!()
  }
}

#[derive(Debug)]
enum Player {
  Human(PlayerHuman),
  Ai(PlayerAi),
}

enum TurnType {
  Initial,
  MidGame,
}

impl Player {
  fn turn(
    &self,
    turn_type: TurnType,
  ) {
    // match self {
    //   Player::Human(q) => q.turn(turn_type)
    //   Player::Ai(q) => q.turn(turn_type)
    // };
  }
}

impl PlayerData {
  pub fn new(
    name: String,
    role: Role,
  ) -> PlayerData {
    let cash = match role {
      Role::Player => 100,
      Role::Dealer => 0,
    };

    PlayerData {
      name,
      role,
      cash,
      current_bet: 0,
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
    bet: u32,
    betting: &Betting,
  ) -> Result<u32, BetError> {
    if (bet > self.cash) {
      return Err(BetError::TooLarge);
    }

    if (bet < betting.min_bet) {
      return Err(BetError::TooSmall);
    }

    if (bet > betting.max_bet) {
      return Err(BetError::TooLarge);
    }

    self.cash -= bet;

    Ok(bet)
  }

  pub fn is_dealer(&self) -> bool {
    self.role == Role::Dealer
  }

  pub fn give_winnings(
    &mut self,
    cash: u32,
  ) {
    self.cash += cash
  }
}
