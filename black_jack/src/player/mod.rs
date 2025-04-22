mod actions;
mod ai;
mod human;

use crate::player::actions::TurnType;
use crate::player::ai::PlayerAi;
use crate::player::human::PlayerHuman;
use crate::table::betting::{BetError, Betting};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Player {
  Human(PlayerHuman),
  Ai(PlayerAi),
}

#[derive(Debug)]
pub struct PlayerData {
  name: String,
  role: Role,
  cash: u32,
  current_bet: u32,
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
    if bet > self.cash {
      return Err(BetError::TooLarge);
    }

    if bet < betting.min_bet {
      return Err(BetError::TooSmall);
    }

    if bet > betting.max_bet {
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

#[derive(Debug, PartialEq, Eq)]
enum Role {
  Player,
  Dealer,
}
