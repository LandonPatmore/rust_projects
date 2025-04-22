use crate::deck::card::Card;
use crate::table::betting::{BetError, Play};

pub trait PlayerActions {
  fn turn(
    &mut self,
    turn_type: TurnType,
    cards: &[Card],
  ) -> Result<Play, BetError>;
}

pub enum TurnType {
  Initial,
  MidGame,
}