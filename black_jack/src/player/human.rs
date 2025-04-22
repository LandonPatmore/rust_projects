use crate::deck::card::Card;
use crate::player::actions::{PlayerActions, TurnType};
use crate::player::PlayerData;
use crate::table::betting::{BetError, Play};

#[derive(Debug)]
pub struct PlayerHuman {
  player: PlayerData,
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