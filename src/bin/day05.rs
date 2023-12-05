use std::ops::Range;

use chumsky::prelude::*;

#[derive(Clone, Debug)]
struct MapTable {
  remaps: Vec<(u128, Range<u128>)>,
}

impl MapTable {
  fn map(&self, input: u128) -> u128 {
    let map_range = &self.remaps.iter().find(|(_, range)| range.contains(&input));

    match map_range {
      Some((dest_idx, range)) => dest_idx + (input - range.start),
      None => input,
    }
  }
}

fn seeds_parser() -> impl Parser<char, Vec<u128>, Error = Simple<char>> {
  let number = text::digits(10).from_str::<u128>().unwrapped();

  just("seeds: ")
    .ignore_then(number.separated_by(text::whitespace()))
}

fn maptable_parser() -> impl Parser<char, MapTable, Error = Simple<char>> {
  let number = text::digits(10).from_str::<u128>().unwrapped();

  let map_range = number
    .separated_by(just(" "))
    .map(|numbers| {
      let dest_start = *numbers.get(0).unwrap();
      let source_start = *numbers.get(1).unwrap();
      let source_len = *numbers.get(2).unwrap();

      (dest_start, source_start..(source_start + source_len))
    });

  // Ignore the map name for now
  take_until(text::newline())
    .ignore_then(map_range.separated_by(text::newline()))
    .map(|remaps| MapTable { remaps })
}

fn main() {
  let input = aoc2023::read_input(5);
  let mut parts = input.split("\n\n");
  let seeds_input = parts.next().unwrap();
  let maps_input = parts.collect::<Vec<&str>>();

  let seeds = seeds_parser().parse(seeds_input).unwrap();
  let maps = maps_input.iter()
    .map(|&mi| maptable_parser().parse(mi))
    .filter_map(|parsed| parsed.ok())
    .collect::<Vec<MapTable>>();

  // println!("{:?}", maps);

  // What is the lowest location number that corresponds to any of the initial seed numbers?
  let min_location = seeds.iter()
    .map(|&seed| {
      maps.iter().fold(seed, |current, remap| remap.map(current))
    })
    .min();
  println!("Part 1: {:?}", min_location);
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_maptable_map() {
    let map_table = MapTable {
      remaps: vec![(10, 0..(0 + 2))],
    };

    assert_eq!(map_table.map(0), 10);
    assert_eq!(map_table.map(1), 11);
    assert_eq!(map_table.map(2), 2);
  }

  #[test]
  fn test_maptable_parsing() {
    let sample = "foo-to-bar map:\n\
                        10 0 2\n\
                        20 10 3";
    let parsed = maptable_parser().parse(sample);
    println!("{:?}", parsed);

    assert!(parsed.is_ok());

    // assert_eq!(map_table.map(0), 10);
    // assert_eq!(map_table.map(1), 11);
    // assert_eq!(map_table.map(2), 2);
  }
}