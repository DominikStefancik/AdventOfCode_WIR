use crate::common::{extract_numbers, sum_mul_pairs};
use regex::Regex;

fn extract_mul_expressions(text: &str) -> Vec<&str> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").expect("Error while creating a regex");

    regex
        .find_iter(text)
        .map(|string_match| string_match.as_str())
        .collect()
}

pub fn sum_all_valid_mul_expressions(text: &str) -> u32 {
    let mul_list = extract_mul_expressions(text);
    let numbers_list = extract_numbers(mul_list);

    sum_mul_pairs(numbers_list)
}
