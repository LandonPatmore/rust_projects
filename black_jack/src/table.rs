use crate::deck::Deck;
use crate::player::Player;

#[derive(Debug)]
pub struct Table<'a> {
  pub betting: Betting,
  pub deck: Deck,
  pub seats: Vec<Seat<'a>>,
}

#[derive(Debug)]
pub struct Betting {
  pub min_bet: u32,
  pub max_bet: u32,
}

#[derive(Debug)]
pub struct Seat<'a> {
  pub player: Option<Player<'a>>,
  pub current_bet: u32,
}

impl<'a> Table<'a> {
  pub fn create(betting: Betting, deck: Deck, players: Vec<Player>) -> Table {
    let seats = players
      .into_iter()
      .map(|player| Seat {
        player: Some(player),
        current_bet: 0,
      })
      .collect();

    Table {
      betting,
      deck,
      seats,
    }
  }
}