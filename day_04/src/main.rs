mod error;
mod find_xmas_shapes;
mod find_xmas_substrings;
mod xmas;

use crate::error::AdventOfCodeError;
use crate::find_xmas_shapes::find_all_xmas_shapes;
use crate::find_xmas_substrings::find_all_xmas_substrings;
use std::fs;

fn main() -> Result<(), AdventOfCodeError> {
    let text = fs::read_to_string("./data/input.txt")?;
    let xmas = find_all_xmas_substrings(&text).unwrap();

    println!(
        "Horizontal left to right search count: {}",
        xmas.horizontal_left_to_right_search_count
    );
    println!(
        "Horizontal right to left search count: {}",
        xmas.horizontal_right_to_left_search_count
    );
    println!(
        "Vertical top to bottom search count: {}",
        xmas.vertical_top_to_bottom_search_count
    );
    println!(
        "Vertical bottom to top search count: {}",
        xmas.vertical_bottom_to_top_search_count
    );
    println!(
        "Diagonal left to right top to bottom search count: {}",
        xmas.diagonal_left_to_right_top_to_bottom_search_count
    );
    println!(
        "Diagonal left to right bottom to top search count: {}",
        xmas.diagonal_left_to_right_bottom_to_top_search_count
    );
    println!(
        "Diagonal right to left top to bottom search count: {}",
        xmas.diagonal_right_to_left_top_to_bottom_search_count
    );
    println!(
        "Diagonal right to left bottom to top search count: {}",
        xmas.diagonal_right_to_left_bottom_to_top_search_count
    );
    println!("Total count of xmas substring: {}", xmas.total_count);

    println!("---------------------------------------");
    let xmas_shapes_count = find_all_xmas_shapes(&text);
    println!("Total count of x-mas shapes: {}", xmas_shapes_count);

    Ok(())
}
