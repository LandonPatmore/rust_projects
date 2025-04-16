use crate::deck::Deck;
use crate::player::Player;
use crate::table::Table;
use crate::table::betting::Betting;
use std::rc::Rc;

#[derive(Debug)]
pub struct Game {
  pub table: Table,
  current_turn: Option<Rc<Player>>,
}

impl Game {
  pub fn new() -> Self {
    let table = Table::new(
      Betting {
        min_bet: 10,
        max_bet: 100,
      },
      Deck::new(6),
    );

    Game {
      table,
      current_turn: None,
    }
  }

  pub fn add_player(
    &mut self,
    player: Player,
  ) {
    self.table.add_player(player);
  }

  pub fn remove_player(
    &mut self,
    index: usize,
  ) {
    self.table.seats.remove(index);
  }

  // TODO: Make sure we deal with cut card, or maybe we just reshuffle on each new round?
  // we deal two cards to each person
  pub fn initialize_round(&mut self) {
    for _ in 0..2 {
      self
        .table
        .seats
        .iter_mut()
        .filter(|seat| seat.has_player())
        .for_each(|seat| {
          // loop until the player gets a card
          loop {
            let card = self.table.deck.cards.pop();

            match card {
              Some(card) => {
                seat.deal_card(card);
                break;
              }
              None => self.table.deck.reset(),
            }
          }
        })
    }
  }
}
