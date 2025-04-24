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

#[derive(Debug, PartialEq, Clone)]
pub enum Play {
  Hit,
  Stand,
  Double,
  // Split,
}