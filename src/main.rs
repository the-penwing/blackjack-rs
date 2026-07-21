//! CLI entry point for blackjack_rs.
//!
//! Handles rendering, input, and the game loop.

// ============================================================
// Imports
// ============================================================

use blackjack_rs::{self, Action, GameState, GameStatus};
use std::io::{self, Write};

// ============================================================
// Input / Terminal Utilities
// ============================================================

fn get_input(prompt: &str) -> String {
  print!("{}", prompt);
  io::stdout().flush().unwrap();

  let mut input = String::new();
  io::stdin()
    .read_line(&mut input)
    .expect("Failed to read line");
  input.trim().to_string()
}

fn clear() {
  print!("\x1B[2J\x1B[1;1H");
  io::stdout().flush().unwrap();
}

// ============================================================
// Rendering
// ============================================================

/// Renders the in-progress round view. The dealer's second card onwards are hidden.
fn render_round(game: &GameState) {
  clear();
  println!("--- YOUR HAND ---");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("Total Value: {}\n", game.player_score());
  println!("--- DEALERS HAND ---");
  let dealers_hand = game.dealer_hand();
  for card in dealers_hand {
    if dealers_hand.first() == Some(card) {
      println!("{}", card);
    } else {
      println!("[Hidden Card]");
    }
  }
  println!();
}

/// Renders the end-of-round result screen with both full hands and session stats.
fn render_round_result(game: &GameState) {
  clear();
  println!("=== ROUND OVER ===");

  println!("--- YOUR HAND ---");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("Total Value: {}\n", game.player_score());
  println!("--- DEALERS HAND ---");
  for card in game.dealer_hand() {
    println!("{}", card);
  }
  println!("Dealer Value: {}\n", game.dealer_score());
  match game.status() {
    GameStatus::PlayerBusted => println!("You Busted!!"),
    GameStatus::PlayerWon => println!("You Won!!"),
    GameStatus::DealerWon => println!("Dealer Won!!"),
    GameStatus::Push => println!("Push!! (Tie)"),
    GameStatus::InProgress => print!(""),
  }
  let (wins, losses, ties) = game.stats();
  println!("--- SESSION STATS ---");
  println!("Wins: {wins} | Losses: {losses} | Pushes: {ties}\n");
}

// ============================================================
// Game Loop
// ============================================================

/// Runs a single round: deals, prompts for actions, and loops until the round ends.
fn round_loop(game: &mut GameState) {
  game.setup_round();
  let mut status;

  loop {
    render_round(game);
    println!("Hit or Stand:");
    println!("1) Hit");
    println!("2) Stand");
    let prompt = "Action: ";
    let choice_raw = get_input(prompt);
    let choice: u8 = match choice_raw.parse::<u8>() {
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

    status = game.update(action);

    if status != GameStatus::InProgress {
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
    round_loop(&mut game);
    render_round_result(&game);

    let keep_playing = loop {
      let prompt = "Play again? (y/n): ";
      let choice_raw = get_input(prompt);

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
