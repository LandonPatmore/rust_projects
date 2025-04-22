use crate::deck::card::Card;
use crate::player::{PlayerData, TurnType};
use crate::player::actions::PlayerActions;
use crate::table::betting::{BetError, Play};

#[derive(Debug)]
pub struct PlayerAi {
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