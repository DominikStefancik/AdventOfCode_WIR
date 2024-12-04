use regex::Regex;
use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let memory_content =
        fs::read_to_string("./data/input.txt").expect("Error while reading the input file");

    let mul_list = extract_mul_expressions(&memory_content);
    let numbers_list = extract_numbers(mul_list);
    let sum = sum_mul_pairs(numbers_list);

    println!("The sum of multiplications is: {sum}");

    Ok(())
}

fn extract_mul_expressions(text: &str) -> Vec<&str> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").expect("Error while creating a regex");

    return regex
        .find_iter(text)
        .map(|string_match| string_match.as_str())
        .collect();
}

fn extract_numbers(expressions: Vec<&str>) -> Vec<Vec<u16>> {
    let regex = Regex::new(r"[0-9]+").expect("Error while creating a regex");

    expressions
        .iter()
        .map(|expression| {
            regex
                .find_iter(expression)
                .map(|string_match| string_match.as_str())
                .map(|string| string.parse().expect("Error while parsing a number"))
                .collect()
        })
        .collect()
}

fn sum_mul_pairs(number_pairs: Vec<Vec<u16>>) -> u32 {
    number_pairs
        .iter()
        .map(|pair| pair[0] as u32 * pair[1] as u32)
        .sum()
}
