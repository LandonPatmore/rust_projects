use crate::deck::Deck;
use crate::player::{Player, PlayerData};
use crate::table::betting::Betting;

#[derive(Debug)]
pub struct Table {
  betting: Betting,
  players: [Option<Player>; 7],
  deck: Deck,
}

impl Table {
  pub fn new(betting: Betting, deck: Deck) -> Table {
    // 1 dealer, 6 players = 7 players
    let players = std::array::from_fn(|_| None);

    Table { betting, players, deck }
  }

  pub fn add_player(
    &mut self,
    player: Player,
  ) -> bool {
    for seat in self.players.iter_mut() {
      if let None = seat {
        *seat = Some(player);
        return true
      }
    }

    eprintln!("Could not add player to table since all seats are occupied");
    false
  }

  pub fn remove_player(
    &mut self,
    index: usize,
  ) {
    self.players[index] = None
  }
}

pub mod betting;
