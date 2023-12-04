#![allow(dead_code)]

use std::collections::BTreeSet;

use chumsky::prelude::*;

#[derive(Clone, Debug)]
struct Card {
  id: u32,
  numbers: Vec<u32>,
  winning_numbers: Vec<u32>,
}


/// Example:
/// Card   1: 17 15  5 75 36 13 16 66 92 39 | 13 92 16  5 87 78 15 94 21 48 30 62 70 41  3 39 22 17 77 58 75 52 83 34 24
fn game_parser() -> impl Parser<char, Card, Error = Simple<char>> {
  let number = text::digits(10).from_str::<u32>().unwrapped();
  let id = number.clone().padded();
  let numbers = number.clone().padded().repeated();

  just("Card")
      .ignore_then(id)
      .then_ignore(just(":").padded())
      .then(numbers)
      .then_ignore(just("|").padded())
      .then(numbers)
      .map(|((id, numbers), winning_numbers)| Card { id, numbers, winning_numbers })
}

fn winner_count(game: &Card) -> u32 {
  let number_set: BTreeSet<u32> = game.numbers.iter().cloned().collect();
  let winning_set: BTreeSet<u32> = game.winning_numbers.iter().cloned().collect();

  number_set.intersection(&winning_set).count() as u32
}

fn get_score(matching: u32) -> u32 {
    match matching {
        0 => 0,
        n => 1 << n - 1,
    }
}

fn score_card(game: &Card) -> u32 {
  get_score(winner_count(game))
}

fn process_cards(hand: &Vec<Card>) -> Vec<u32> {
  let mut counts : Vec<u32> = vec![1; hand.len()];

  for card in hand {
    let idx = card.id as usize - 1;
    let card_count = *counts.get(idx).unwrap();
    let num_winners = winner_count(card);

    for won_card_idx in (idx + 1)..(idx + 1 + num_winners as usize) {
      *counts.get_mut(won_card_idx as usize).unwrap() += 1 * card_count;
    }
  }

  counts
}

fn main() {
  let input = aoc2023::read_input(4);
  let parser = game_parser();

  let deck = input.lines()
    .map(|line| parser.parse(line))
    .filter_map(|parsed| parsed.ok())
    .collect::<Vec<Card>>();

  //  How many points are they worth in total?
  let total_points: u32 = deck.iter().map(score_card).sum();
  println!("Part 1: {}", total_points);

  // Achshually
  // We earn copies of those cards when we win
  // How many total scratchcards do you end up with?
  let card_counts = process_cards(&deck);
  let total_card_count = card_counts.iter().sum::<u32>();
  println!("Part 2: {}", total_card_count);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_game_parser() {
    let card = game_parser().parse("Card  1: 17 15  5 75 36 13 16 66 92 39 | 13 92 16  5 87 78 15 94 21 48 30 62 70 41  3 39 22 17 77 58 75 52 83 34 24").unwrap();
    assert_eq!(card.id, 1);
    assert_eq!(card.numbers, vec![17, 15, 5, 75, 36, 13, 16, 66, 92, 39]);
    assert_eq!(card.winning_numbers, vec![13, 92, 16, 5, 87, 78, 15, 94, 21, 48, 30, 62, 70, 41, 3, 39, 22, 17, 77, 58, 75, 52, 83, 34, 24]);
  }
}