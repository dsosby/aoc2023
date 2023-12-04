#![allow(dead_code)]

use aoc2023::Coordinate;
use itertools::Itertools;

#[derive(Clone, Debug, Eq, PartialEq)]
struct Symbol {
    value: char,
    coordinate: Coordinate,
}

#[derive(Clone, Debug)]
struct ProductNumber {
    id: u32,
    symbols: Vec<Symbol>,
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
                    let starting_col = (if cur_char_is_digit {
                        col_idx + 1
                    } else {
                        col_idx
                    }) - buffer.len();
                    let ending_col = starting_col + buffer.len() - 1;

                    let symbols =
                        aoc2023::get_bordering(&matrix, starting_col, row_idx, ending_col, row_idx)
                            .iter()
                            .map(|coordinate| Symbol {
                                coordinate: coordinate.clone(),
                                value: *matrix
                                    .get(coordinate.1)
                                    .unwrap()
                                    .get(coordinate.0)
                                    .unwrap(),
                            })
                            .filter(|sym| sym.value != '.' && sym.value.is_ascii_punctuation())
                            .collect::<Vec<Symbol>>();

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
    // let sum_of_products: u32 = products.iter().map(|p| p.id).sum();
    // println!("Part 1: {}", sum_of_products);

    // Part 2 - What is the sum of all of the gear ratios in your engine schematic?
    // let count_of_multi_gears = products
    //     .iter()
    //     .map(|p| p.symbols.iter().filter(|s| s.value == '*').count())
    //     .counts();
    // println!("{:?}", count_of_multi_gears);
    // ^confirms there's only max one gear per product

    let geared_products = products.iter()
        .map(|p| ProductNumber { id: p.id, symbols: p.symbols.iter().filter(|&s| s.value == '*').map(|s| s.clone()).collect::<Vec::<Symbol>>()})
        .filter(|p| p.symbols.len() > 0)
        .collect::<Vec<ProductNumber>>();

    println!("{:?}", geared_products);

    let geared_collections= geared_products.iter()
        .group_by(|p| p.symbols.first().unwrap().clone());
    // This isn't returning full groups for some reason... some groups, but not all groups (e.g. Coordinate(8, 1) returns only 180, not 180 and 923)
    // GEEEEEEZUS WTF - https://github.com/rust-itertools/itertools/issues/374

    println!("{:?}", geared_collections);

    let geared_pairs = geared_collections
        .into_iter()
        .map(|grouping| grouping.1.map(|g| g.id).collect::<Vec::<u32>>())
        .filter(|product_ids| product_ids.len() == 2);

    let sum_of_geared_pair_ratios: u32 = geared_pairs
        .map(|product_ids| product_ids.iter().product::<u32>())
        .sum();

    println!("Part 2: {}", sum_of_geared_pair_ratios);
}
