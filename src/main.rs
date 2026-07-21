use blackjack_rs::{self, Action, GameState, GameStatus};
use std::{
  io::{self, Write},
  process::Command,
};

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
  if cfg!(target_os = "windows") {
    Command::new("cls");
  } else {
    Command::new("clear");
  }
}

fn render_round(game: &GameState) {
  clear();
  println!("Your hand:\n");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("\nTotal Value: {}\n", game.player_score());
  println!("Dealers Hand:\n");
  let dealers_hand = game.dealer_hand();
  for card in dealers_hand {
    if dealers_hand.first() == Some(card) {
      println!("{}", card);
    } else {
      println!("???");
    }
  }
}

fn render_round_result(game: &GameState) {
  clear();
  println!("Your Hand: \n");
  for card in game.player_hand() {
    println!("{}", card);
  }
  println!("\n Total Value: {}\n", game.player_score());
  println!("Dealers Hand:\n");
  for card in game.dealer_hand() {
    println!("{}", card);
  }
  println!("\n Dealer Value: {}", game.dealer_score());
  match game.status() {
    GameStatus::PlayerBusted => println!("You Busted!!"),
    GameStatus::PlayerWon => println!("You Won!!"),
    GameStatus::DealerWon => println!("Dealer Won!!"),
    GameStatus::Push => println!("Push!! (Tie)"),
    GameStatus::InProgress => print!(""),
  }
  let (wins, losses, ties) = game.stats();
  println!("Stats:");
  println!("Wins: {wins}\nLosses: {losses}\nPushes: {ties}");
}

fn round_loop(game: &mut GameState) {
  game.setup_round();
  render_round(game);

  let mut status = game.status();

  loop {
    println!("Hit or Stand:");
    println!("1) Hit");
    println!("2) Stand");
    let prompt = "Action";
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
    render_round(game);

    if status != GameStatus::InProgress {
      break;
    }
  }
}

fn main() {
  let mut game = GameState::new_game();
  loop {
    round_loop(&mut game);
    render_round_result(&game);
    let mut keep_playing = false;
    loop {
      let prompt = "Play again? (y/n): ";
      let choice_raw = get_input(prompt);
      match choice_raw.to_uppercase().as_str() {
        "Y" => {
          keep_playing = true;
          break;
        },
        "N" => {
          keep_playing = false;
          break;
        },
        _ => println!("Please enter either 'y' or 'n'"),
      };
    }
    if !keep_playing {
      println!("Thanks for Playing!!");
      break;
    }
  }
}
