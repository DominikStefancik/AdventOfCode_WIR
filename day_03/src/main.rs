mod common;
mod extract_all_mul_numbers;
mod extract_enabled_mul_numbers;

use crate::extract_all_mul_numbers::sum_all_valid_mul_expressions;
use crate::extract_enabled_mul_numbers::sum_enabled_valid_mul_expressions;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let memory_content =
        fs::read_to_string("./data/input.txt").expect("Error while reading the input file");

    let sum_of_all_mul_expressions = sum_all_valid_mul_expressions(&memory_content);
    let sum_of_enabled_mul_expressions = sum_enabled_valid_mul_expressions(&memory_content);

    println!("The sum of all multiplications is: {sum_of_all_mul_expressions}");
    println!("The sum of only enabled multiplications is: {sum_of_enabled_mul_expressions}");

    Ok(())
}
