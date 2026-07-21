use std::fmt;

use rand::rng;
use rand::seq::SliceRandom;

pub struct GameState {
  deck: Vec<Card>,
  status: GameStatus,
  player_hand: Vec<Card>,
  dealer_hand: Vec<Card>,
  wins: u32,
  losses: u32,
  ties: u32,
}

impl GameState {
  pub fn new_game() -> Self {
    Self {
      deck: {
        let mut deck = build_deck();
        shuffle_deck(&mut deck);
        deck
      },
      status: GameStatus::InProgress,
      player_hand: Vec::new(),
      dealer_hand: Vec::new(),
      wins: 0,
      losses: 0,
      ties: 0,
    }
  }
  pub fn setup_round(&mut self) {
    self.status = GameStatus::InProgress;
    self.player_hand = Vec::new();
    self.dealer_hand = Vec::new();
    if self.deck.len() < 10 {
      self.deck = build_deck();
      shuffle_deck(&mut self.deck);
    }
    let hands = [&mut self.player_hand, &mut self.dealer_hand];
    for hand in hands {
      for _ in 0..2 {
        if let Some(dealt_card) = deal_card(&mut self.deck) {
          hand.push(dealt_card);
        }
      }
    }
  }
  pub fn update(&mut self, action: Action) -> GameStatus {
    match action {
      Action::Hit => self.handle_hit(),
      Action::Stand => self.handle_stand(),
    }
  }
  fn handle_hit(&mut self) -> GameStatus {
    if let Some(dealt_card) = deal_card(&mut self.deck) {
      self.player_hand.push(dealt_card);
    }
    if calc_hand_value(&self.player_hand) > 21 {
      self.losses += 1;
      GameStatus::PlayerBusted
    } else {
      GameStatus::InProgress
    }
  }
  fn handle_stand(&mut self) -> GameStatus {
    while calc_hand_value(&self.dealer_hand) <= 16 {
      if let Some(dealt_card) = deal_card(&mut self.deck) {
        self.dealer_hand.push(dealt_card);
      }
    }
    let player_val = calc_hand_value(&self.player_hand);
    let dealer_val = calc_hand_value(&self.dealer_hand);
    if player_val > 21 {
      self.losses += 1;
      GameStatus::PlayerBusted
    } else if dealer_val > 21 {
      self.wins += 1;
      GameStatus::PlayerWon
    } else if player_val > dealer_val {
      self.wins += 1;
      GameStatus::PlayerWon
    } else if dealer_val > player_val {
      self.losses += 1;
      GameStatus::DealerWon
    } else {
      self.ties += 1;
      GameStatus::Push
    }
  }

  pub fn player_hand(&self) -> &[Card] {
    &self.player_hand
  }
  pub fn dealer_hand(&self) -> &[Card] {
    &self.dealer_hand
  }
  pub fn player_score(&self) -> u8 {
    calc_hand_value(&self.player_hand)
  }
  pub fn dealer_score(&self) -> u8 {
    calc_hand_value(&self.dealer_hand)
  }
  pub fn stats(&self) -> (u32, u32, u32) {
    (self.wins, self.losses, self.ties)
  }
  pub fn status(&self) -> GameStatus {
    self.status.clone()
  }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Hit,
  Stand,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum GameStatus {
  InProgress,
  PlayerBusted,
  PlayerWon,
  DealerWon,
  Push,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
  Hearts,
  Diamonds,
  Spades,
  Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
  Numeric(u8),
  Jack,
  Queen,
  King,
  Ace,
}

impl fmt::Display for Suit {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Suit::Hearts => write!(f, "Hearts"),
      Self::Diamonds => write!(f, "Diamonds"),
      Self::Spades => write!(f, "Spades"),
      Self::Clubs => write!(f, "Clubs"),
    }
  }
}

impl fmt::Display for Rank {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Rank::Numeric(num) => write!(f, "{}", num),
      Rank::Jack => write!(f, "Jack"),
      Rank::Queen => write!(f, "Queen"),
      Rank::King => write!(f, "King"),
      Rank::Ace => write!(f, "Ace"),
    }
  }
}

impl fmt::Display for Card {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    write!(f, "{} of {}", self.rank, self.suit)
  }
}

impl Rank {
  fn value(&self) -> u8 {
    match self {
      Rank::Ace => 11,
      Rank::King | Rank::Queen | Rank::Jack => 10,
      Rank::Numeric(number) => *number,
    }
  }
}

const SUITS: [Suit; 4] = [Suit::Hearts, Suit::Diamonds, Suit::Spades, Suit::Clubs];
const RANKS: [Rank; 13] = [
  Rank::Numeric(2),
  Rank::Numeric(3),
  Rank::Numeric(4),
  Rank::Numeric(5),
  Rank::Numeric(6),
  Rank::Numeric(7),
  Rank::Numeric(8),
  Rank::Numeric(9),
  Rank::Numeric(10),
  Rank::Jack,
  Rank::Queen,
  Rank::King,
  Rank::Ace,
];

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Card {
  pub rank: Rank,
  pub suit: Suit,
}

impl Card {
  fn value(&self) -> u8 {
    self.rank.value()
  }
}

fn build_deck() -> Vec<Card> {
  let mut deck = Vec::new();

  for suit in SUITS {
    for rank in RANKS {
      deck.push(Card { rank, suit });
    }
  }
  deck
}

fn shuffle_deck(deck: &mut Vec<Card>) {
  let mut rng = rng();
  deck.shuffle(&mut rng);
}

fn deal_card(deck: &mut Vec<Card>) -> Option<Card> {
  deck.pop()
}

fn calc_hand_value(hand: &[Card]) -> u8 {
  let mut total: u8 = 0;
  let mut aces: u8 = 0;

  for card in hand {
    total += card.value();
    if card.rank == Rank::Ace {
      aces += 1;
    };
  }

  while total > 21 && aces > 0 {
    total -= 10;
    aces -= 1;
  }

  total
}
