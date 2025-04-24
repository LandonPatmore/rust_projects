use crate::deck::card::Card;
use crate::game::betting::Play::{Double, Hit, Stand};
use crate::game::betting::{Betting, Play};
use crate::player::PlayerData;
use rand::Rng;
use crate::player;

pub fn turn(
  player_data: &PlayerData,
  dealer_visible_card: &Card,
) -> Play {
  let total = player::total(&player_data.cards);

  if dealer_visible_card.value() == 11 {
    return if total >= 17 { Stand } else { Hit };
  }

  let dealer_value = dealer_visible_card.value();

  match total {
    8 => Hit, // Always hit if hand total is 8 or less
    9 => match dealer_value {
      2..=6 => Double, // Double down on 9 if dealer has 2-6
      _ => Hit,        // Otherwise, just hit
    },
    10 => match dealer_value {
      2..=9 => Double, // Double down on 10 if dealer has 2-9
      _ => Hit,        // Otherwise, just hit
    },
    11 => Double, // Always double down on 11
    12 => match dealer_value {
      4..=6 => Stand, // Stand on 12 if dealer has 4-6
      _ => Hit,       // Otherwise, just hit
    },
    13..=16 => match dealer_value {
      2..=6 => Stand, // Stand if dealer has 2-6
      _ => Hit,       // Otherwise, hit
    },
    17..=21 => Stand, // Always stand on 17 or more
    _ => Hit,         // If player has less than 8, always hit
  }
}

pub(crate) fn bet(
  player_data: &mut PlayerData,
  betting: &Betting,
) -> u32 {
  let mut rand = rand::rng();

  let probability: f64 = rand.random();

  let bet = match probability > 0.75 {
    true => risky_bet(player_data, betting),
    false => betting.min_bet,
  };

  player_data.cash -= bet;

  bet
}

fn risky_bet(
  player_data: &PlayerData,
  betting: &Betting,
) -> u32 {
  let mut rand = rand::rng();

  let probability: f64 = rand.random();

  match probability {
    p if p > 0.95 => {
      if player_data.cash > betting.max_bet {
        betting.max_bet
      } else {
        player_data.cash
      }
    }
    p if p > 0.75 => ((player_data.cash as f64) * 0.75).round() as u32,
    p if p > 0.50 => ((player_data.cash as f64) * 0.50).round() as u32,
    _ => ((player_data.cash as f64) * 0.25).round() as u32,
  }
}
