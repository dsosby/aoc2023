use chumsky::prelude::*;

#[derive(Clone, Debug)]
enum Color {
    Red,
    Green,
    Blue,
}

#[derive(Clone, Debug)]
struct ColorCount {
    color: Color,
    count: usize,
}

#[derive(Clone, Debug, Default)]
struct DicePull {
    red: usize,
    green: usize,
    blue: usize,
}

#[derive(Clone, Debug)]
struct Game {
    id: usize,
    pulls: Vec<DicePull>,
}

// Example
// Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green

fn parser() -> impl Parser<char, Game, Error = Simple<char>> {
    let color = just("red")
        .to(Color::Red)
        .or(just("green").to(Color::Green))
        .or(just("blue").to(Color::Blue));

    let int = text::int(10).map(|s: String| s.parse().unwrap()).boxed();
    let count = int.clone();
    let game_id = int.clone();

    let color_count = count
        .then_ignore(just(" "))
        .then(color.clone())
        .map(|(count, color)| ColorCount { color, count });

    let dice_pull = color_count.separated_by(just(",").padded()).map(|counts| {
        // TODO: This should be a from trait
        let mut pull: DicePull = Default::default();
        for count in counts {
            match count.color {
                Color::Red => pull.red = count.count,
                Color::Green => pull.green = count.count,
                Color::Blue => pull.blue = count.count,
            }
        }
        pull
    });

    let game = just("Game ")
        .ignore_then(game_id)
        .then_ignore(just(":").padded())
        .then(dice_pull.separated_by(just(";").padded()))
        .map(|(game_id, pulls)| Game { id: game_id, pulls });

    game.then_ignore(end())
}

fn is_impossible(bag: &DicePull, game: &Game) -> bool {
  // If any dice pulls in the game exceed the bag 
  game.pulls.iter().any(|pull| {
    pull.red > bag.red
    || pull.green > bag.green
    || pull.blue > bag.blue
  })
}

fn max_pull(game: &Game) -> DicePull {
  DicePull {
    red: game.pulls.iter().max_by(|&x, &y| x.red.cmp(&y.red)).unwrap().red,
    green: game.pulls.iter().max_by(|&x, &y| x.green.cmp(&y.green)).unwrap().green,
    blue: game.pulls.iter().max_by(|&x, &y| x.blue.cmp(&y.blue)).unwrap().blue,
  }
}

fn power_of_pull(pull: &DicePull) -> u32 {
  pull.red as u32 * pull.green as u32 * pull.blue as u32
}

fn main() {
  let bag = DicePull { red: 12, green: 13, blue: 14 };
  let parser = parser();
  let input = aoc2023::read_input(2);

  let games = input.lines()
    .map(|line| parser.parse(line))
    .filter_map(|parsed| parsed.ok());

  let possible_games = games.clone()
    .filter(|game| !is_impossible(&bag, game));

  // println!("{:?}", possible_games);
  let sum_of_possible_game_ids: u32 = possible_games.map(|g| g.id as u32).sum();
  println!("Sum of possible game Ids: {}", sum_of_possible_game_ids);

  let sum_of_game_powers: u32 = games.clone()
    .map(|game| max_pull(&game))
    .map(|pull| power_of_pull(&pull))
    .sum();
  println!("Sum of power of max pull: {}", sum_of_game_powers);
}