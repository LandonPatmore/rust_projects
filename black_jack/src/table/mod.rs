use crate::deck::Deck;
use crate::player::Player;
use crate::table::betting::Betting;
use crate::table::seat::Seat;

#[derive(Debug)]
pub struct Table {
  betting: Betting,
  pub deck: Deck,
  pub seats: Vec<Seat>,
}

impl Table {
  pub fn new(
    betting: Betting,
    deck: Deck,
  ) -> Table {
    // 1 dealer, 6 players
    let seats: Vec<Seat> = (0..=6).map(|_| Seat::new()).collect();

    Table {
      betting,
      deck,
      seats,
    }
  }

  pub fn add_player(
    &mut self,
    player: Player,
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
    self.seats.remove(index);
  }
}

pub mod betting;
mod seat;
