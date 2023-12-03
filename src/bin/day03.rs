#![allow(dead_code)]

#[derive(Clone, Debug)]
struct ProductNumber {
    id: u32,
    symbols: Vec<char>,
}

fn extract_products(matrix: &Vec<Vec<char>>) -> Vec<ProductNumber> {
    let mut buffer = String::new();
    let mut products = vec![];

    for (row_idx, row) in matrix.iter().enumerate() {
        for (col_idx, cur_char) in row.iter().enumerate() {
            let is_end_of_row = col_idx == row.len() - 1;
            let cur_char_is_digit = cur_char.is_digit(10);

            if cur_char_is_digit {
                buffer.push(*cur_char);
            }

            if is_end_of_row || !cur_char_is_digit {
                // Complete active product
                if !buffer.is_empty() {
                    let id: u32 = buffer.parse().unwrap();
                    let starting_col = (if cur_char_is_digit { col_idx + 1 } else { col_idx }) - buffer.len();
                    let ending_col = starting_col + buffer.len() - 1;

                    let symbols = aoc2023::get_bordering(
                        &matrix,
                        starting_col,
                        row_idx,
                        ending_col,
                        row_idx,
                    )
                    .iter()
                    .filter(|&&c| c != '.' && c.is_ascii_punctuation())
                    .copied()
                    .collect::<Vec<char>>();

                    if !symbols.is_empty() {
                      products.push(ProductNumber { id, symbols });
                    }
                    buffer.clear();
                }
            }
        }
    }

    products
}

fn main() {
    let input = aoc2023::read_input(3);
    let matrix = aoc2023::to_matrix(&input);

    let products = extract_products(&matrix);

    // Part 1 - What is the sum of all of the part numbers in the engine schematic?
    let sum_of_products: u32 = products.iter().map(|p| p.id).sum();
    println!("Part 1: {}", sum_of_products);
}
