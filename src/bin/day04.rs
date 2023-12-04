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

  just("Card")
      .ignore_then(number.clone().padded())
      .then_ignore(just(":").padded())
      .then(number.clone().padded().repeated())
      .then_ignore(just("|").padded())
      .then(number.clone().padded().repeated())
      .map(|((id, numbers), winning_numbers)| Card { id, numbers, winning_numbers })
}

fn get_score(matching: usize) -> u32 {
    match matching {
        0 => 0,
        n => 1 << n - 1,
    }
}

fn score_game(game: &Card) -> u32 {
  let number_set: BTreeSet<u32> = game.numbers.iter().cloned().collect();
  let winning_set: BTreeSet<u32> = game.winning_numbers.iter().cloned().collect();
  let num_winners = number_set.intersection(&winning_set).count();

  get_score(num_winners)
}

fn main() {
  let input = aoc2023::read_input(4);
  let parser = game_parser();

  let games = input.lines()
    .map(|line| parser.parse(line))
    .filter_map(|parsed| parsed.ok())
    .collect::<Vec<Card>>();

  //  How many points are they worth in total?
  let total_points: u32 = games.iter().map(score_game).sum();
  println!("Part 1: {}", total_points);
}