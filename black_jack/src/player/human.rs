use crate::deck::card::Card;
use crate::player::player_data::PlayerData;
use crate::player::{Play, Player};
use crate::table::betting::Betting;
use std::io;

struct Human {
  player_data: PlayerData,
}

impl Player for Human {
  fn new(
    name: String,
    is_dealer: bool,
    cash: u32,
  ) -> Self {
    Human {
      player_data: PlayerData {
        name,
        is_dealer,
        cash,
        cards: Vec::new(),
      },
    }
  }

  fn bet(
    &mut self,
    betting: &Betting,
  ) -> u32 {
    loop {
      println!("What is your bet?");

      let mut bet = String::new();

      io::stdin()
        .read_line(&mut bet)
        .expect("Failed to read input");

      let bet: u32 = match bet.trim().parse() {
        Ok(num) => num,
        Err(_) => {
          println!("Please input a number for your bet!");
          continue;
        }
      };

      if bet > self.player_data.cash {
        eprintln!(
          "Not enough cash for this bet! You only have: ${}",
          self.player_data.cash
        );
        continue;
      }

      if bet < betting.min_bet {
        eprintln!(
          "The bet was too small! The minimum bet is: ${}",
          betting.min_bet
        );
        continue;
      }

      if bet > betting.max_bet {
        eprintln!(
          "The bet was too large! The maximum bet is: ${}",
          betting.max_bet
        );
        continue;
      }

      self.player_data.cash -= bet;

      return bet;
    }
  }

  fn turn(
    &self,
    dealer_visible_card: &Card,
  ) -> Play {
    todo!()
  }
}
