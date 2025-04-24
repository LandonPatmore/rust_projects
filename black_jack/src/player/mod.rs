pub mod ai;
pub mod human;

use crate::deck::card::{Card, CardRank};
use crate::game::betting::Play::{Double, Hit, Stand};
use crate::game::betting::{BetError, Betting, Play};
use std::fmt::Debug;

#[derive(Debug)]
pub enum Player {
  Human(PlayerData),
  Ai(PlayerData),
}

#[derive(Debug)]
pub struct PlayerData {
  name: String,
  cash: u32,
  current_bet: u32,
  cards: Vec<Card>,
}

pub fn total(cards: &[Card]) -> usize {
  let initial_total = cards.iter().fold(0, |acc, card| acc + card.value());
  let aces = cards
    .iter()
    .filter(|card| matches!(card.rank, CardRank::Ace))
    .count();

  initial_total - if initial_total > 21 { aces * 10 } else { 0 }
}

impl Player {
  pub(crate) fn turn(
    &self,
    dealer_visible_card: &Card,
  ) -> Play {
    match self {
      Player::Human(player) => human::turn(player, dealer_visible_card),
      Player::Ai(player) => ai::turn(player, dealer_visible_card),
    }
  }

  fn bet(
    &mut self,
    betting: &Betting,
  ) -> u32 {
    match self {
      Player::Human(player) => human::bet(player, betting),
      Player::Ai(player) => ai::bet(player, betting),
    }
  }

  pub(crate) fn can_play(
    &self,
    betting: &Betting,
  ) -> bool {
    match self {
      Self::Human(player) | Self::Ai(player) => player.can_play(betting),
    }
  }

  fn give_winnings(
    &mut self,
    cash: u32,
  ) {
    match self {
      Self::Human(player) | Self::Ai(player) => player.give_winnings(cash),
    }
  }

  pub fn deal_card(
    &mut self,
    card: Card,
  ) {
    match self {
      Self::Human(player) | Self::Ai(player) => player.deal_card(card),
    }
  }

  pub fn can_keep_playing(&self) -> bool {
    match self {
      Self::Human(player) | Self::Ai(player) => total(&player.cards) <= 21,
    }
  }
}

impl PlayerData {
  pub fn new(name: String) -> PlayerData {
    PlayerData {
      name,
      cash: 100,
      current_bet: 0,
      cards: Vec::new(),
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

  pub fn give_winnings(
    &mut self,
    cash: u32,
  ) {
    self.cash += cash
  }

  pub fn deal_card(
    &mut self,
    card: Card,
  ) {
    self.cards.push(card);
  }

  fn possible_plays(&self) -> Vec<Play> {
    let mut plays = Vec::new();

    plays.push(Hit);
    plays.push(Stand);

    if self.can_double() {
      plays.push(Double);
    }

    plays
  }

  fn can_double(&self) -> bool {
    self.cards.len() == 2
  }

  // TODO: Will have to refactor cards to be vec of vec of cards
  fn can_split(&self) -> bool {
    todo!()
  }
}
