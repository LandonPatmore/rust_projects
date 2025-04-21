use strum_macros::EnumIter;

#[derive(Debug)]
pub struct Betting {
  pub min_bet: u32,
  pub max_bet: u32,
}

#[derive(Debug, PartialEq)]
pub enum BetError {
  TooSmall,
  TooLarge,
  NotEnoughCash,
}

#[derive(Debug, PartialEq, EnumIter)]
pub enum Play {
  Hit,
  Stand,
  Double(u32),
  Split,
}