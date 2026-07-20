use blackjack_rs;
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

fn main() {}
