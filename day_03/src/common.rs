use regex::Regex;

pub fn extract_numbers(expressions: Vec<&str>) -> Vec<Vec<u16>> {
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

pub fn sum_mul_pairs(number_pairs: Vec<Vec<u16>>) -> u32 {
    number_pairs
        .iter()
        .map(|pair| pair[0] as u32 * pair[1] as u32)
        .sum()
}
