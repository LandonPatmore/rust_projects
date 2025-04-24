use crate::deck::Deck;
use crate::deck::card::Card;
use crate::game::dealer::Dealer;
use crate::player::{Player, PlayerData};
use betting::Betting;
use std::fmt::format;
use crate::game::betting::Play;
use crate::player;

pub mod betting;
pub mod dealer;

#[derive(Debug)]
pub struct Game {
  betting: Betting,
  deck: Deck,
  players: [Option<Player>; 6],
  dealer: Dealer,
}

impl Game {
  pub fn new() -> Self {
    Game {
      betting: Betting {
        min_bet: 10,
        max_bet: 100,
      },
      players: std::array::from_fn(|_| None),
      dealer: Dealer::new(),
      deck: Deck::new(6),
    }
  }

  // we deal two cards to each person
  fn initialize_round(&mut self) {
    // Get count of active players once
    let active_player_count = self.players.iter().filter(|p| p.is_some()).count();

    // Collect all needed cards (2 per player) + dealer
    let cards: Vec<Card> = (0..(2 * active_player_count))
      .map(|_| self.deal_card())
      .collect();

    // Get mutable references to active players
    let mut players: Vec<&mut Player> = self
      .players
      .iter_mut()
      .filter_map(|opt_player| opt_player.as_mut())
      .filter(|q| q.can_play(&self.betting))
      .collect();

    // Distribute cards
    let mut card_iter = cards.into_iter();
    (0..2).for_each(|_| {
      players.iter_mut().for_each(|player| {
        if let Some(card) = card_iter.next() {
          player.deal_card(card);
        }
      });
    });

    (0..2).for_each(|_| {
      self.dealer.deal_card(self.deck.deal_card());
    })
  }

  // TODO: Player is always first, we will allow choosing next
  fn generate_players(&mut self) {
    self.add_player(Player::Human(PlayerData::new("Player".to_string())));

    let mut ai_index = 1;
    loop {
      let ai_player = Player::Ai(PlayerData::new(format(format_args!("AI #{}", ai_index))));
      ai_index += 1;

      if !self.add_player(ai_player) {
        break;
      }
    }
  }

  fn add_player(
    &mut self,
    player: Player,
  ) -> bool {
    for seat in self.players.iter_mut() {
      if seat.is_none() {
        *seat = Some(player);
        return true;
      }
    }
    false
  }

  fn remove_player(
    &mut self,
    index: usize,
  ) {
    self.players[index] = None
  }

  fn deal_card(&mut self) -> Card {
    self.deck.deal_card()
  }

  pub fn run_loop(&mut self) {
    // loop {
      self.generate_players();
      self.initialize_round();

      self
        .players
        .iter_mut()
        .filter_map(|player| player.as_mut())
        .for_each(|player| {
          loop {
            let turn = player.turn(self.dealer.face_up_card());
            
            match turn {
              Play::Hit => player.deal_card(self.deck.deal_card()),
              Play::Stand => break,
                // TODO: Fix
              Play::Double => player.deal_card(self.deck.deal_card())
            };
            
            if !player.can_keep_playing() {
              println!("You lose!");
              break
            }
          }
        });
    }
  // }
}
