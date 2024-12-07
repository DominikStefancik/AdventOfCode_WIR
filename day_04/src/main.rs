mod error;
mod find_all_xmas;
mod xmas;

use crate::error::AdventOfCodeError;
use crate::find_all_xmas::get_all_xmas;
use std::fs;

fn main() -> Result<(), AdventOfCodeError> {
    let text = fs::read_to_string("./data/input.txt")?;
    let xmas = get_all_xmas(&text).unwrap();

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
    println!("Total count: {}", xmas.total_count);

    Ok(())
}
