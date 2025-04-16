use crate::deck::card::{Card, CardRank};
use crate::player::Player;
use crate::table::seat::RoundStatus::PLAYING;
use std::rc::Rc;

#[derive(Debug)]
pub struct Seat {
  player: Option<Player>,
  round_status: RoundStatus,
  current_bet: u32,
  cards: Vec<Card>,
}

#[derive(Debug)]
pub enum RoundStatus {
  WIN,
  LOSE,
  PLAYING,
}

impl Seat {
  pub fn new() -> Self {
    Seat {
      player: None,
      round_status: PLAYING,
      current_bet: 0,
      cards: Vec::new(),
    }
  }

  pub fn add_player(
    &mut self,
    player: Player,
  ) {
    self.player = Some(player);
  }

  pub fn remove_player(&mut self) {
    self.player = None;
  }

  pub fn has_player(&self) -> bool {
    self.player.is_some()
  }

  pub fn add_bet(
    &mut self,
    bet: u32,
  ) {
    self.current_bet += bet
  }

  pub fn reset_bet(&mut self) {
    self.current_bet = 0
  }

  pub fn total(&self) -> usize {
    let initial_total = self.cards.iter().fold(0, |acc, card| acc + card.value());
    let aces = self
      .cards
      .iter()
      .filter(|card| matches!(card.rank, CardRank::Ace))
      .count();

    initial_total - if initial_total > 21 { aces * 10 } else { 0 }
  }

  pub fn deal_card(
    &mut self,
    card: Card,
  ) {
    self.cards.push(card);
  }

  pub fn remove_cards(&mut self) -> Vec<Card> {
    let mut removed = Vec::new();

    loop {
      let removed_card = self.cards.pop();

      match removed_card {
        Some(card) => removed.push(card),
        None => break,
      }
    }

    removed
  }
}

mod tests {
  use crate::deck::card::Card;
  use crate::deck::card::CardRank::{Ace, Ten, Three, Two};
  use crate::deck::card::Suit::{Hearts, Spades};
  use crate::table::seat::Seat;
  use std::rc::Rc;
  use crate::player::Player;

  #[test]
  fn total_under_21() {
    let card1 = Card::new(Hearts, Ten);
    let card2 = Card::new(Spades, Three);

    let mut seat = Seat::new();
    seat.deal_card(card1);
    seat.deal_card(card2);

    let total = seat.total();

    assert_eq!(total, 13);
  }

  #[test]
  fn total_over_21() {
    let card1 =Card::new(Hearts, Ten);
    let card2 =Card::new(Spades, Three);
    let card3 = Card::new(Spades, Ten);

    let mut seat = Seat::new();
    seat.deal_card(card1);
    seat.deal_card(card2);
    seat.deal_card(card3);

    let total = seat.total();

    assert_eq!(total, 23);
  }

  #[test]
  fn total_21_with_ace() {
    let card1 = Card::new(Hearts, Ten);
    let card2 = Card::new(Spades, Ace);

    let mut seat = Seat::new();
    seat.deal_card(card1);
    seat.deal_card(card2);

    let total = seat.total();

    assert_eq!(total, 21);
  }

  #[test]
  fn total_13_with_ace() {
    let card1 = Card::new(Hearts, Ten);
    let card2 = Card::new(Spades, Ace);
    let card3 = Card::new(Spades, Two);

    let mut seat = Seat::new();
    seat.deal_card(card1);
    seat.deal_card(card2);
    seat.deal_card(card3);

    let total = seat.total();

    assert_eq!(total, 13);
  }
  
  #[test]
  fn add_player() {
    let mut seat = Seat::new();
    
    let player = Player::new("Bob".to_string(), false, 100);
    
    seat.add_player(player);
    
    assert!(seat.has_player());
  }

  #[test]
  fn remove_player() {
    let mut seat = Seat::new();

    let player = Player::new("Bob".to_string(), false, 100);

    seat.add_player(player);

    assert!(seat.has_player());
    
    seat.remove_player();
    
    assert!(!seat.has_player());
  }
  
  #[test]
  fn card_is_dealt_to_seat() {
    let mut seat = Seat::new();
    
    seat.deal_card(Card::new(Hearts, Ten));
    
    assert_eq!(seat.cards.len(), 1);
    assert_eq!(seat.cards[0].suit, Hearts);
    assert_eq!(seat.cards[0].rank, Ten);
  }
  
  #[test]
  fn cards_are_removed() {
    let mut seat = Seat::new();
    
    seat.deal_card(Card::new(Hearts, Ten));
    
    assert_eq!(seat.cards.len(), 1);
    
    seat.remove_cards();
    
    assert_eq!(seat.cards.len(), 0);
  }
  
  #[test]
  fn bet() {
    let mut seat = Seat::new();
    
    seat.add_bet(10);
    
    assert_eq!(seat.current_bet, 10);
  }
  
  #[test]
  fn bet_multiple_times() {
    let mut seat = Seat::new();
    
    seat.add_bet(10);
    seat.add_bet(10);
    seat.add_bet(10);
    
    assert_eq!(seat.current_bet, 30);
  }
  
  #[test]
  fn bet_and_then_reset_bet_to_0() {
    let mut seat = Seat::new();
    
    seat.add_bet(10);
    
    assert_eq!(seat.current_bet, 10);
    
    seat.reset_bet();
    
    assert_eq!(seat.current_bet, 0);
  }
}
