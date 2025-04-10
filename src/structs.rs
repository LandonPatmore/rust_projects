use strum_macros::EnumIter;

pub struct Game {
  table: Table,
  current_turn: Player,

}

#[derive(Debug)]
pub struct Table {
  pub betting: Betting,
  pub deck: Deck,
  pub seats: Vec<Seat>,
}

#[derive(Debug)]
pub struct Betting {
  pub min_bet: u32,
  pub max_bet: u32,
}

#[derive(Debug)]
pub struct Seat {
  pub player: Option<Player>,
  pub current_bet: u32,
}

#[derive(Debug)]
pub struct Player {
  pub name: String,
  pub is_dealer: bool,
  pub cash: u32,
  pub hand: Vec<Card>,
}

#[derive(Debug)]
pub struct Card {
  pub suit: Suit,
  pub rank: CardRank,
}

#[derive(Debug)]
pub struct Deck {
  pub cards: Vec<Card>,
  pub discard: Vec<Card>,
  pub break_card_position: usize,
}

#[derive(Debug, EnumIter, Copy, Clone)]
pub enum Suit {
  Spades,
  Hearts,
  Diamonds,
  Clubs,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone)]
pub enum CardRank {
  Ace,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Play {
  Hit,
  Stand,
  Double,
  // Split TODO: later on
}