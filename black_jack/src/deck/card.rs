use strum_macros::EnumIter;

#[derive(Debug, PartialEq, Eq)]
pub struct Card {
  pub suit: Suit,
  pub rank: CardRank,
}

#[derive(Debug, EnumIter, Copy, Clone, PartialEq, Eq)]
pub enum Suit {
  Spades,
  Hearts,
  Diamonds,
  Clubs,
}

#[derive(Debug, EnumIter, PartialEq, Eq, Copy, Clone)]
pub enum CardRank {
  Ace,
  Two,
  Three,
  Four,
  Five,
  Six,
  Seven,
  Eight,
  Nine,
  Ten,
  Jack,
  Queen,
  King,
}

impl Card {
  pub fn new(
    suit: Suit,
    rank: CardRank,
  ) -> Card {
    Card { suit, rank }
  }

  pub fn value(&self) -> usize {
    match self.rank {
      CardRank::Ace => 11,
      CardRank::Two => 2,
      CardRank::Three => 3,
      CardRank::Four => 4,
      CardRank::Five => 5,
      CardRank::Six => 6,
      CardRank::Seven => 7,
      CardRank::Eight => 8,
      CardRank::Nine => 9,
      _ => 10,
    }
  }

  fn suit(&self) -> char {
    match self.suit {
      Suit::Spades => '\u{2660}',
      Suit::Hearts => '\u{2665}',
      Suit::Diamonds => '\u{2666}',
      Suit::Clubs => '\u{2663}',
    }
  }

  pub fn card_display(&self) {
    println!("┌───────┐");
    println!("│       │");
    println!("│  {:<2}{}  │", self.value(), self.suit());
    println!("│       │");
    println!("└───────┘");
  }
}
