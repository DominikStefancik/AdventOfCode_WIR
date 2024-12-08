mod common;
mod correctly_ordered_updates;
mod error;
mod incorrectly_ordered_updates;
mod parse;

use crate::correctly_ordered_updates::get_middle_page_sum_of_correctly_ordered_updates;
use crate::error::AdventOfCodeError;
use crate::incorrectly_ordered_updates::get_middle_page_sum_of_fixed_incorrectly_ordered_updates;
use crate::parse::parse_input;
use std::fs;

fn main() -> Result<(), AdventOfCodeError> {
    let input = fs::read_to_string("./data/input.txt")?;
    let (rules, updates) = parse_input(&input)?;
    let middle_pages_sum = get_middle_page_sum_of_correctly_ordered_updates(&rules, &updates);
    let middle_pages_sum_of_fixed_updates =
        get_middle_page_sum_of_fixed_incorrectly_ordered_updates(rules, updates);

    println!("The sum of all correctly ordered updates: {middle_pages_sum}");
    println!(
        "The sum of all incorrectly ordered updates after fix: {middle_pages_sum_of_fixed_updates}"
    );

    Ok(())
}
