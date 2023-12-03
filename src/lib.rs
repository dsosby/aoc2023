use std::env;
use std::fs;

pub fn read_input(day: u32) -> String {
    let cwd = env::current_dir().unwrap();

    let filepath = cwd
        .join("src")
        .join("inputs")
        .join(format!("{:02}.txt", day));

    let f = fs::read_to_string(filepath);
    f.expect("could not open input file")
}

// Matrix stuff
pub type Matrix = Vec::<Vec::<char>>;

pub fn get_bordering(matrix: &Matrix, top_left_col: usize, top_left_row: usize, bot_right_col: usize, bot_right_row: usize) -> Vec::<char> {
    let height = matrix.len();
    let width = matrix.get(0).unwrap().len();

    let mut border: Vec::<char> = vec![];

    // Top & Bottom
    for col in (if top_left_col > 0 { top_left_col - 1 } else { 0 })..=(if bot_right_col < width - 2 { bot_right_col + 1 } else { bot_right_col }) {
        if top_left_row > 0 {
            border.push(*matrix.get(top_left_row - 1).unwrap().get(col).unwrap());
        }

        if bot_right_row < height - 2 {
            border.push(*matrix.get(bot_right_row + 1).unwrap().get(col).unwrap());
        }
    }

    // Left and Right
    for row in top_left_row..=bot_right_row {
        if top_left_col > 0 {
            border.push(*matrix.get(row).unwrap().get(top_left_col - 1).unwrap());
        }

        if bot_right_col < width - 2 {
            border.push(*matrix.get(row).unwrap().get(bot_right_col + 1).unwrap());
        }
    }

    border
}

pub fn to_matrix(input: &str) -> Matrix {
    input.lines().map(|line| line.chars().collect::<Vec<char>>()).collect()
}
