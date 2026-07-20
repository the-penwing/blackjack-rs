#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Suit {
  Hearts,
  Diamonds,
  Spades,
  Clubs,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum Rank {
  Numeric(u8),
  Jack,
  Queen,
  King,
  Ace,
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

#[derive(Debug, Clone)]
struct Card {
  rank: Rank,
  suit: Suit,
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

fn deal_card(deck: &mut Vec<Card>) -> Option<Card> {
  deck.pop()
}
