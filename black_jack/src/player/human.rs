use crate::deck::card::Card;
use crate::game::betting::{Betting, Play};
use crate::player::PlayerData;
use std::io;
use crate::player;

pub fn turn(
  player_data: &PlayerData,
  dealer_visible_card: &Card,
) -> Play {
  loop {
    println!("Dealer visible card: {:?}", dealer_visible_card);
    println!("Your cards: {:?}", player_data.cards);
    println!("Your total: {:?}", player::total(&player_data.cards));
    println!("What would you like to do?");

    let possible_plays = player_data.possible_plays();

    possible_plays.iter().enumerate().for_each(|(index, play)| {
      println!("Play {}: {:?}", index + 1, play);
    });

    let mut action = String::new();

    io::stdin()
      .read_line(&mut action)
      .expect("Failed to read line");

    let action: usize = match action.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Not a valid numbered action");
        continue;
      }
    };

    // -1 since we want users to use 1 index not 0
    if let Some(play) = possible_plays.get(action - 1) {
      return play.clone();
    }

    println!("Out of bounds selection");
    continue;
  }
}

pub(crate) fn bet(
  player_data: &mut PlayerData,
  betting: &Betting,
) -> u32 {
  loop {
    println!("What would you like to bet?");
    println!("You currently have: ${}", player_data.cash);

    let mut bet = String::new();

    io::stdin()
      .read_line(&mut bet)
      .expect("Failed to read line");

    let bet: u32 = match bet.trim().parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Not a number");
        continue;
      }
    };

    match player_data.bet(bet, betting) {
      Ok(b) => return b,
      Err(error) => {
        println!("Error: {:?}", error);
        continue;
      }
    }
  }
}
