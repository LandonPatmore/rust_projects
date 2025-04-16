use crate::deck::Deck;
use crate::player::Player;
use crate::table::Table;
use crate::table::betting::Betting;
use std::fmt;
use std::rc::Rc;

#[derive(Debug)]
pub struct Game {
  pub table: Table,
  // current_turn: Option<Rc<Player>>,
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
      // current_turn: None,
    }
  }

  // we deal two cards to each person
  pub fn initialize_round(&mut self) {
    (0..2).for_each(|_| {
      self
        .table
        .seats
        .iter_mut()
        .filter(|seat| seat.has_player())
        .for_each(|seat| {
          // loop until the player gets a card, since pop may come back with nothing
          // which should never happen, but defensive programming
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
    })
  }

  pub fn generate_players(&mut self) {
    (0..self.table.seats.len() - 1).for_each(|index| {
      self.table.add_player(Player::new(
        fmt::format(format_args!("AI #{}", index + 1)),
        false,
        false,
        100,
      ))
    });
    
    self
      .table
      .add_player(Player::new("Dealer".to_string(), true, false, 100));
  }

  fn add_player(
    &mut self,
    player: Player,
  ) {
    self.table.add_player(player);
  }

  fn remove_player(
    &mut self,
    index: usize,
  ) {
    self.table.remove_player(index);
  }
  
  fn game_loop(&mut self) {
    self.table.seats.iter().for_each(|seat| {
      if seat.has_player() {
        seat.total()
      }
    })
  }
}
