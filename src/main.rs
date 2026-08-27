//! CLI entry point for blackjack_rs.
//!
//! Handles rendering, input, and the game loop.

// ============================================================
// Imports
// ============================================================

use blackjack_rs::{self, Action, BetError, GameState, GameStatus};
use std::io::{self, Write};

// ============================================================
// Input / Terminal Utilities
// ============================================================

/// Prints `prompt` without a newline, flushes stdout so it's visible
/// immediately, then blocks for a line of input and returns it trimmed.
fn get_input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

/// Clears the terminal, including scrollback, via the `clearscreen` crate
/// rather than a hand-rolled ANSI escape sequence (which doesn't reliably
/// clear scrollback across terminals/platforms).
fn clear() {
  clearscreen::clear().expect("Failed to clear screen")
}

// ============================================================
// Rendering
// ============================================================

/// Prints the player's hand and running total. Shared by the in-progress
/// and end-of-round views.
fn render_player_hand(game: &GameState) {
  println!("--- YOUR HAND ---");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("Total Value: {}", game.player_score());
}

/// Prints the dealer's hand. While a round is in progress, all but the
/// dealer's first card are hidden; once the round has ended, the full
/// hand and total are revealed.
fn render_dealer_hand(game: &GameState) {
  println!("--- DEALERS HAND ---");

  if game.status() == GameStatus::InProgress {
    let dealers_hand = game.dealer_hand();
    for card in dealers_hand {
      if dealers_hand.first() == Some(card) {
        println!("{}", card);
      } else {
        println!("[Hidden Card]");
      }
    }
  } else {
    for card in game.dealer_hand() {
      println!("{}", card);
    }
    println!("Dealer Value: {}", game.dealer_score());
  }
}

/// Prints cumulative session win/loss/push counts.
fn render_stats(game: &GameState) {
  let (wins, losses, ties) = game.stats();
  println!("--- SESSION STATS ---");
  println!("Wins: {wins} | Losses: {losses} | Pushes: {ties}\n");
}

/// Renders the in-progress round view. The dealer's second card onwards are hidden.
fn render_round(game: &GameState) {
  clear();
  render_player_hand(game);
  println!();
  render_dealer_hand(game);
  println!();
}

/// Renders the end-of-round result screen with both full hands and session stats.
/// Must be called before `game.reset_status()`, since it reads the round's
/// terminal `GameStatus` to decide which result message to print.
fn render_round_result(game: &GameState) {
  clear();
  println!("=== ROUND OVER ===");
  render_player_hand(game);
  println!();
  render_dealer_hand(game);
  println!();

  match game.status() {
    GameStatus::PlayerBlackjack => println!("Natural Blackjack!!"),
    GameStatus::PlayerBusted => println!("You Busted!!"),
    GameStatus::PlayerWon => println!("You Won!!"),
    GameStatus::DealerWon => println!("Dealer Won!!"),
    GameStatus::Push => println!("Push!! (Tie)"),
    _ => {},
  }
  render_stats(game);
}

/// Renders the pre-round betting screen showing the player's current balance.
fn render_betting(game: &GameState) {
  clear();
  println!("--- Betting Time ---");
  println!("You have: ${}", game.balance());
}

// ============================================================
// Game Loop
// ============================================================

/// Prompts for a bet until one is placed successfully.
///
/// Requires `game.status()` to be `AwaitingBet` on entry (guaranteed by
/// `main`'s loop, which resets status after each round resolves).
/// `WrongStatus` should be unreachable given that invariant, so it panics
/// loudly instead of failing silently if it's ever hit.
fn betting_loop(game: &mut GameState) {
  loop {
    render_betting(game);
    println!();
    let amount_raw = get_input("How much to bet? ");
    let amount: u32 = match amount_raw.parse() {
      Ok(num) => num,
      Err(_) => {
        println!("Please input a valid number");
        continue;
      },
    };

    match game.place_bet(amount) {
      Err(BetError::ZeroAmount) => {
        println!("You can't place a bet of $0!");
      },
      Err(BetError::InsufficientBalance) => {
        println!("You don't have enough money!");
      },
      Err(BetError::WrongStatus) => {
        panic!("bet placed with wrong status: {:?}", game.status());
      },
      Ok(_) => {
        println!("Bet placed for ${}!", amount);
        break;
      },
    }
  }
}

/// Runs a single round: deals, prompts for actions, and loops until the round ends.
///
/// Leaves `game.status()` set to the round's terminal outcome (e.g.
/// `PlayerWon`, `PlayerBusted`, `PlayerBlackjack`) — the caller is
/// responsible for rendering that result and calling `reset_status()`
/// afterwards, so status stays valid for `render_round_result` to read.
fn round_loop(game: &mut GameState) {
  game.setup_round();

  // A natural blackjack is already resolved by `setup_round`; skip
  // straight past the hit/stand prompt instead of asking for an action
  // on a round that's already over.
  if game.status() == GameStatus::PlayerBlackjack {
    return;
  }

  loop {
    render_round(game);
    println!("Hit or Stand:");
    println!("1) Hit");
    println!("2) Stand");
    let choice_raw = get_input("Action: ");
    let choice: u8 = match choice_raw.parse() {
      Ok(1) => 1,
      Ok(2) => 2,
      _ => {
        println!("Please input 1 or 2");
        continue;
      },
    };
    let action = match choice {
      1 => Action::Hit,
      2 => Action::Stand,
      _ => unreachable!(),
    };

    if game.update(action) != GameStatus::InProgress {
      break;
    }
  }
}

// ============================================================
// Entry Point
// ============================================================

fn main() {
  let mut game = GameState::new_game();

  loop {
    betting_loop(&mut game);
    round_loop(&mut game);
    render_round_result(&game);
    game.reset_status();

    let keep_playing = loop {
      let choice_raw = get_input("Play again? (y/n): ");
      match choice_raw.to_uppercase().as_str() {
        "Y" => break true,
        "N" => break false,
        _ => println!("Please enter either 'y' or 'n'"),
      };
    };

    if !keep_playing {
      println!("Thanks for Playing!!");
      break;
    }
  }
}
