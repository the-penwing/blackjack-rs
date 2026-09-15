use blackjack_rs::{self, Action, BetError, GameState, GameStatus};
use std::io::{self, Write};

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
  clearscreen::clear().expect("Failed to clear screen")
}

fn render_player_hand(game: &GameState) {
  println!("--- YOUR HAND ---");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("Total Value: {}", game.player_score());
}

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

fn render_stats(game: &GameState) {
  let (wins, losses, ties) = game.stats();
  println!("--- SESSION STATS ---");
  println!("Wins: {wins} | Losses: {losses} | Pushes: {ties}\n");
}

fn render_round(game: &GameState) {
  clear();
  render_player_hand(game);
  println!();
  render_dealer_hand(game);
  println!();
}

fn render_round_result(game: &GameState) {
  clear();
  println!("=== ROUND OVER ===");
  render_player_hand(game);
  println!();
  render_dealer_hand(game);
  println!();

  match game.status() {
    GameStatus::PlayerBlackjack => println!("Natural Blackjack!!"),
    GameStatus::DealerBlackjack => println!("Dealer Blackjack!!"),
    GameStatus::BlackjackPush => println!("Blackjack Push!!"),
    GameStatus::PlayerBusted => println!("You Busted!!"),
    GameStatus::PlayerWon => println!("You Won!!"),
    GameStatus::DealerWon => println!("Dealer Won!!"),
    GameStatus::Push => println!("Push!! (Tie)"),
    _ => {},
  }
  render_stats(game);
  if game.balance() == 0 {
    println!();
    println!("You're out of cash!")
  }
}

fn render_betting(game: &GameState) {
  clear();
  println!("--- Betting Time ---");
  println!("You have: ${}", game.balance());
}

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

fn round_loop(game: &mut GameState) {
  game.setup_round();

  if game.status() != GameStatus::InProgress {
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

fn playing_again(prompt: &str) -> bool {
  loop {
    let choice_raw = get_input(prompt);
    match choice_raw.to_uppercase().as_str() {
      "Y" => break true,
      "N" => break false,
      _ => println!("Please enter either 'y' or 'n'"),
    };
  }
}

fn main() {
  let mut game = GameState::new_game();
  let non_broke_prompt: String = String::from("Play again? (y/n): ");
  let broke_prompt: String = String::from("Restart from scratch? (y/n): ");

  loop {
    betting_loop(&mut game);
    round_loop(&mut game);
    render_round_result(&game);
    game.reset_status();

    let is_broke: bool = game.balance() == 0;

    let is_playing_again: bool = if is_broke {
      playing_again(&broke_prompt)
    } else {
      playing_again(&non_broke_prompt)
    };
    if !is_playing_again {
      println!("Thanks for Playing!!");
      break;
    }
    if is_broke {
      game = GameState::new_game();
    }
  }
}
