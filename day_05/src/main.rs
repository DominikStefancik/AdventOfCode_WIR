mod correctly_ordered_updates;
mod error;
mod parse;

use crate::correctly_ordered_updates::get_middle_page_sum_of_correctly_ordered_updates;
use crate::error::AdventOfCodeError;
use crate::parse::parse_input;
use std::fs;
fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let (rules, updates) = parse_input(&input)?;
    let middle_pages_sum = get_middle_page_sum_of_correctly_ordered_updates(rules, updates);

    println!("The sum of all correctly ordered updates: {middle_pages_sum}");

    Ok(())
}
