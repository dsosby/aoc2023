#![allow(dead_code)]

use std::cmp::Ordering;
use std::convert::TryInto;

use chumsky::{prelude::*, text::Character};
use itertools::{self, Itertools};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
enum Card {
    Ace,
    King,
    Queen,
    Jack,
    Number(u32),
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Hand([Card; 5]);

#[derive(Clone, Debug, Eq, PartialEq)]
struct Game {
    hand: Hand,
    bet: u32,
}

fn game_parser() -> impl Parser<char, Game, Error = Simple<char>> {
    let number =
        filter::<_, _, _>(|c: &char| c.is_digit(10)).map(|c| Card::Number(c.to_digit(10).unwrap()));

    let card = choice((
        just('A').to(Card::Ace),
        just('K').to(Card::King),
        just('Q').to(Card::Queen),
        just('J').to(Card::Jack),
        just('T').to(Card::Number(10)),
        number,
    ));

    let hand = card.clone().repeated().exactly(5);
    let bet = text::int(10).from_str::<u32>().unwrapped();

    hand.then_ignore(text::whitespace())
        .then(bet)
        .map(|(hand, bet)| Game {
            hand: Hand(hand.try_into().unwrap()),
            bet,
        })
}

fn compare_hands(left: &Hand, right: &Hand) -> Ordering {
    // TODO: Implement
    Ordering::Equal
}

fn total_winnings(games: &Vec<Game>) -> u32 {
    games
        .iter()
        .sorted_by(|lhv, rhv| compare_hands(&lhv.hand, &rhv.hand))
        .enumerate()
        .map(|(idx, game)| game.bet * (idx as u32 + 1))
        .sum()
}

fn main() {
    let parser = game_parser();
    let input = aoc2023::read_input(7);

    let games: Vec<Game> = input
        .lines()
        .map(|line| parser.parse(line).unwrap())
        .collect();

    let total_winnings = total_winnings(&games);
    println!("Part 1 {}", total_winnings);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_game_parser() {
        let parser = game_parser();

        assert_eq!(
            parser.parse("AKQT9 123"),
            Ok(Game {
                hand: Hand([
                    Card::Ace,
                    Card::King,
                    Card::Queen,
                    Card::Number(10),
                    Card::Number(9)
                ]),
                bet: 123
            })
        );
    }
}
