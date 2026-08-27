//! Core game logic for blackjack_rs.
//!
//! Provides the deck, card types, game state, and round resolution logic.

// ============================================================
// Imports
// ============================================================

use std::fmt;

use rand::rng;
use rand::seq::SliceRandom;

// ============================================================
// Types: Enums
// ============================================================

/// A player action during a round.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
  Hit,
  Stand,
}

/// The current status of a round.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum GameStatus {
  /// Waiting for the player to place a bet via `place_bet`.
  AwaitingBet,
  /// A round is underway; further `Action`s are expected.
  InProgress,
  PlayerBusted,
  PlayerWon,
  /// Player was dealt a natural 21 (ace + ten-value card) on the deal.
  PlayerBlackjack,
  DealerWon,
  Push,
}

/// Reasons `place_bet` can fail.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum BetError {
  /// A bet of 0 was attempted.
  ZeroAmount,
  /// The bet exceeds the player's current balance.
  InsufficientBalance,
  /// `place_bet` was called while status wasn't `AwaitingBet`.
  WrongStatus,
}

/// A card suit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Suit {
  Hearts,
  Diamonds,
  Spades,
  Clubs,
}

/// A card rank, with numeric cards carrying their face value.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Rank {
  Numeric(u8),
  Jack,
  Queen,
  King,
  Ace,
}

// ============================================================
// Display Implementations
// ============================================================

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

// ============================================================
// Rank Value
// ============================================================

impl Rank {
  /// Returns the blackjack point value of this rank.
  /// Aces are initially valued at 11; `calc_hand_value` reduces them as needed.
  fn value(&self) -> u8 {
    match self {
      Rank::Ace => 11,
      Rank::King | Rank::Queen | Rank::Jack => 10,
      Rank::Numeric(number) => *number,
    }
  }
}

// ============================================================
// Constants
// ============================================================

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

// ============================================================
// Types: Card
// ============================================================

/// A single playing card with a rank and suit.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Card {
  pub rank: Rank,
  pub suit: Suit,
}

impl Card {
  fn value(&self) -> u8 {
    self.rank.value()
  }
}

// ============================================================
// Game State
// ============================================================

/// Holds all state for an ongoing blackjack session, including the deck,
/// both hands, and win/loss/tie counters across rounds.
///
/// `balance` and `current_bet` are stored internally at 2x their real
/// chip value ("half-chip units"), so a 3:2 blackjack payout is always
/// computed as an exact integer (`current_bet * 3`) with no rounding
/// loss. Callers only ever see real chip amounts via `balance()`,
/// `current_bet()`, and `place_bet()`, which convert at the boundary.
pub struct GameState {
  deck: Vec<Card>,
  status: GameStatus,
  player_hand: Vec<Card>,
  dealer_hand: Vec<Card>,
  wins: u32,
  losses: u32,
  ties: u32,
  balance: u32,
  current_bet: u32,
}

impl GameState {
  /// Creates a new session with a freshly built and shuffled deck.
  pub fn new_game() -> Self {
    Self {
      deck: {
        let mut deck = build_deck();
        shuffle_deck(&mut deck);
        deck
      },
      status: GameStatus::AwaitingBet,
      player_hand: Vec::new(),
      dealer_hand: Vec::new(),
      wins: 0,
      losses: 0,
      ties: 0,
      balance: 2000,
      current_bet: 0,
    }
  }

  /// Resets hands and deals two cards each to the player and dealer.
  /// Rebuilds and reshuffles the deck if fewer than 10 cards remain.
  ///
  /// If the deal is a natural blackjack, resolves the round immediately:
  /// sets status to `PlayerBlackjack`, records the win, and pays out.
  /// Callers must check `status()` after calling this before prompting
  /// for a hit/stand action.
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
    if self.is_nat_blackjack() {
      self.status = GameStatus::PlayerBlackjack;
      self.wins += 1;
      self.resolve_payout(GameStatus::PlayerBlackjack);
    };
  }

  /// Applies a player action and returns the resulting [`GameStatus`].
  pub fn update(&mut self, action: Action) -> GameStatus {
    match action {
      Action::Hit => self.handle_hit(),
      Action::Stand => self.handle_stand(),
    }
  }

  /// Resets status to `AwaitingBet`, readying the game for the next
  /// `place_bet` call. Must be called after the current round's result
  /// has been read/rendered, since it discards the terminal status.
  pub fn reset_status(&mut self) {
    self.status = GameStatus::AwaitingBet;
  }

  // ----------------------------------------
  // Betting & Payout
  // ----------------------------------------

  /// Places a bet of `real_amount` chips, deducting it from `balance`
  /// and moving status from `AwaitingBet` to `InProgress`.
  ///
  /// `real_amount` is a real chip amount, not the internal half-chip
  /// representation — this converts at the boundary.
  pub fn place_bet(&mut self, real_amount: u32) -> Result<(), BetError> {
    let amount: u32 = real_amount * 2;
    if self.status != GameStatus::AwaitingBet {
      Err(BetError::WrongStatus)
    } else if real_amount == 0 {
      Err(BetError::ZeroAmount)
    } else if amount > self.balance {
      Err(BetError::InsufficientBalance)
    } else {
      self.balance -= amount;
      self.current_bet = amount;
      self.status = GameStatus::InProgress;
      Ok(())
    }
  }

  /// Pays out `current_bet` according to the round's terminal status,
  /// then clears `current_bet` back to 0.
  ///
  /// Amounts are stored at 2x real value (see struct docs), so a
  /// blackjack's 3:2 payout is `current_bet * 3` and a normal win's 1:1
  /// payout is `current_bet * 2` — both exact integers of the doubled
  /// value. Losses forfeit the bet outright (no arm needed).
  fn resolve_payout(&mut self, status: GameStatus) {
    match status {
      GameStatus::PlayerBlackjack => self.balance += self.current_bet * 3,
      GameStatus::PlayerWon => self.balance += self.current_bet * 2,
      GameStatus::Push => self.balance += self.current_bet,
      _ => {},
    };
    self.current_bet = 0;
  }

  // ----------------------------------------
  // Action Handlers
  // ----------------------------------------

  /// Deals one card to the player. Busts the round if this pushes their
  /// hand over 21.
  fn handle_hit(&mut self) -> GameStatus {
    if let Some(dealt_card) = deal_card(&mut self.deck) {
      self.player_hand.push(dealt_card);
    }
    self.status = if calc_hand_value(&self.player_hand) > 21 {
      self.losses += 1;
      self.resolve_payout(GameStatus::PlayerBusted);
      GameStatus::PlayerBusted
    } else {
      GameStatus::InProgress
    };
    self.status
  }

  /// Plays out the dealer's hand (hitting until 17+) and resolves the
  /// round's final outcome.
  fn handle_stand(&mut self) -> GameStatus {
    while calc_hand_value(&self.dealer_hand) <= 16 {
      if let Some(dealt_card) = deal_card(&mut self.deck) {
        self.dealer_hand.push(dealt_card);
      }
    }
    let player_val = calc_hand_value(&self.player_hand);
    let dealer_val = calc_hand_value(&self.dealer_hand);
    self.status = if player_val > 21 {
      self.losses += 1;
      self.resolve_payout(GameStatus::PlayerBusted);
      GameStatus::PlayerBusted
    } else if dealer_val > 21 || player_val > dealer_val {
      self.wins += 1;
      self.resolve_payout(GameStatus::PlayerWon);
      GameStatus::PlayerWon
    } else if dealer_val > player_val {
      self.losses += 1;
      self.resolve_payout(GameStatus::DealerWon);
      GameStatus::DealerWon
    } else {
      self.ties += 1;
      self.resolve_payout(GameStatus::Push);
      GameStatus::Push
    };
    self.status
  }

  // ----------------------------------------
  // Accessors
  // ----------------------------------------

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

  /// Current balance in real chip units (internal half-chip value / 2).
  pub fn balance(&self) -> u32 {
    self.balance / 2
  }

  /// Current bet in real chip units (internal half-chip value / 2).
  pub fn current_bet(&self) -> u32 {
    self.current_bet / 2
  }

  /// Returns session totals as `(wins, losses, ties)`.
  pub fn stats(&self) -> (u32, u32, u32) {
    (self.wins, self.losses, self.ties)
  }

  pub fn status(&self) -> GameStatus {
    self.status
  }

  /// True if the player's current hand is a natural blackjack
  /// (an ace + ten-value card dealt as the opening two cards).
  pub fn is_nat_blackjack(&self) -> bool {
    self.player_hand.len() == 2 && calc_hand_value(&self.player_hand) == 21
  }
}

// ============================================================
// Deck Helpers
// ============================================================

/// Builds a fresh, unshuffled 52-card deck.
fn build_deck() -> Vec<Card> {
  let mut deck = Vec::new();

  for suit in SUITS {
    for rank in RANKS {
      deck.push(Card { rank, suit });
    }
  }
  deck
}

/// Shuffles a deck in place.
fn shuffle_deck(deck: &mut [Card]) {
  let mut rng = rng();
  deck.shuffle(&mut rng);
}

/// Deals (pops) the top card from the deck, if any remain.
fn deal_card(deck: &mut Vec<Card>) -> Option<Card> {
  deck.pop()
}

// ============================================================
// Hand Value Calculator
// ============================================================

/// Calculates the best blackjack value for a hand.
/// Aces are counted as 1 instead of 11 when needed to avoid busting.
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
