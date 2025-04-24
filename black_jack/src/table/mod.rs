use crate::game::betting::Betting;
use crate::game::dealer::Dealer;
use crate::player::Player;

pub struct Table {
  betting: Betting,
  players: [Option<Player>; 6],
  dealer: Dealer,
}

impl Table {
  pub fn new(
    min_bet: u32,
    max_bet: u32,
  ) -> Self {
    Table {
      betting: Betting { min_bet, max_bet },
      players: std::array::from_fn(|_| None),
      dealer: Dealer::new(),
    }
  }
}
