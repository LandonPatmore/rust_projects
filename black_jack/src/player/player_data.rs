use crate::deck::card::Card;

pub struct PlayerData {
  pub(crate) name: String,
  pub(crate) is_dealer: bool,
  pub(crate) cash: u32,
  pub(crate) cards: Vec<Card>,
}