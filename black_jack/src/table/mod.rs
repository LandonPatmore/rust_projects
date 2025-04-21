use crate::deck::Deck;
use crate::player::{PlayerData, PlayerActions};
use crate::table::betting::Betting;
use crate::table::seat::Seat;

#[derive(Debug)]
pub struct Table {
  betting: Betting,
  pub deck: Deck,
  pub seats: [Seat; 7],
}

impl Table {
  pub fn new(
    betting: Betting,
    deck: Deck,
  ) -> Table {
    // 1 dealer, 6 players
    let seats = std::array::from_fn(|i| Seat::new());

    Table {
      betting,
      deck,
      seats,
    }
  }

  pub fn add_player(
    &mut self,
    player: PlayerData,
  ) {
    for seat in &mut self.seats {
      if !seat.has_player() {
        seat.add_player(player);
        return;
      }
    }

    eprintln!("Could not add player to table since all seats are occupied");
  }

  pub fn remove_player(
    &mut self,
    index: usize,
  ) {
    self.seats[index].remove_player();
  }
}

pub mod betting;
mod seat;
