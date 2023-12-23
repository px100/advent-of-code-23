use itertools::Itertools;

fn main(input: &str) -> (usize, usize) {
  let mut hands = input
    .split('\n')
    .map(|line| {
      let (cards, bid) = line.split_once(' ').unwrap();
      let p1_key = strength(cards, false);
      let p2_key = strength(cards, true);
      (cards, bid.parse().unwrap(), p1_key, p2_key)
    })
    .collect::<Vec<_>>();
  hands.sort_unstable_by_key(|&(_, _, key, _)| key);
  let p1_score = hands.iter().enumerate().map(|(i, (_, bid, _, _))| (i + 1) * bid).sum();
  hands.sort_unstable_by_key(|&(_, _, _, key)| key);
  let p2_score = hands.iter().enumerate().map(|(i, (_, bid, _, _))| (i + 1) * bid).sum();
  (p1_score, p2_score)
}

fn strength(cards: &str, p2: bool) -> (usize, usize) {
  let card_counts = cards
    .chars()
    .counts();
  let card_values = card_counts
    .iter()
    .filter(|&(card, _)| *card != 'J' || !p2)
    .map(|(_, &count)| count)
    .collect::<Vec<_>>();
  let joker_count = if !p2 { 0 } else { *card_counts.get(&'J').unwrap_or(&0) };
  let index = cards
    .chars()
    .fold(0, |acc, card| (acc << 4) + index(card, p2));
  let hand_type = match (card_values.iter().max().copied().unwrap_or(0), joker_count) {
    (max_value, jokers) if max_value + jokers == 5 => 6,
    (max_value, jokers) if max_value + jokers == 4 => 5,
    (3, 0) if card_values.contains(&2) => 4,
    (3, 0) => 3,
    (2, _) => match (
      card_values.iter().filter(|&&count| count == 2).count(),
      joker_count,
    ) {
      (2, 1) => 4,
      (1, 1) => 3,
      (2, 0) => 2,
      _ => 1,
    },
    (1, 2) => 3,
    (1, 1) => 1,
    _ => 0,
  };
  (hand_type, index)
}

fn index(c: char, p2: bool) -> usize {
  match c {
    'A' => 14,
    'K' => 13,
    'Q' => 12,
    'J' if p2 => 0,
    'J' => 11,
    'T' => 10,
    '9' | '8' | '7' | '6' | '5' | '4' | '3' | '2' => c.to_digit(10).unwrap() as usize,
    _ => unreachable!(),
  }
}
