use crate::common::{extract_numbers, sum_mul_pairs};
use regex::Regex;

fn get_regex_indices(text: &str, regex: &str) -> Vec<usize> {
    let regex = Regex::new(regex).expect("Error while creating a regex");

    regex
        .find_iter(text)
        .map(|string_match| string_match.start())
        .collect()
}

fn extract_mul_expressions_with_indices(text: &str) -> Vec<(usize, &str)> {
    let regex = Regex::new(r"mul\([0-9]+,[0-9]+\)").expect("Error while creating a regex");

    regex
        .find_iter(text)
        .map(|string_match| (string_match.start(), string_match.as_str()))
        .collect()
}

fn create_enabled_intervals(
    do_indices: &Vec<usize>,
    dont_indices: &Vec<usize>,
) -> Vec<(usize, usize)> {
    let mut enabled_intervals: Vec<(usize, usize)> = vec![];
    let mut index1 = 0_usize;
    let mut index2 = 0_usize;

    while index1 < do_indices.len() {
        let start = do_indices[index1];

        while index2 < dont_indices.len() {
            let end = dont_indices[index2];

            if start < end {
                enabled_intervals.push((start, end));
                break;
            }

            index2 += 1;
        }

        index1 += 1;
    }

    enabled_intervals
}

fn get_enabled_mull_expressions<'a>(
    mul_expressions: &'a Vec<(usize, &str)>,
    enabled_intervals: &Vec<(usize, usize)>,
) -> Vec<&'a str> {
    let mut enabled_mul_expressions: Vec<&str> = vec![];

    for expression in mul_expressions {
        let expression_index = expression.0;
        let enabled_interval = enabled_intervals
            .iter()
            .find(|interval| expression_index > interval.0 && expression_index < interval.1);

        if enabled_interval.is_some() {
            enabled_mul_expressions.push(expression.1);
        }
    }

    enabled_mul_expressions
}

pub fn sum_enabled_valid_mul_expressions(text: &str) -> u32 {
    // 1. First create a list of indices of all "do()" expressions
    let mut do_indices = get_regex_indices(text, r"do\(\)");
    // 2. Then create a list of indices of all "don't()" expressions
    let mut dont_indices = get_regex_indices(text, r"don't\(\)");

    // since at the beginning of the program, mul instructions are enabled,
    // it's like there is a "do()" expression at the very beginning of the text -> it index=0
    if !do_indices.contains(&0_usize) {
        do_indices.insert(0, 0_usize);
    }

    // simulate the fact that at the very end of the text, there is a "don't()" expression
    // -> it's index = text.len() - 1
    if !dont_indices.contains(&(text.len() - 1)) {
        dont_indices.push(text.len() - 1);
    }

    // 3. Create intervals of indices where text is between "do()" and "don't()" expressions ->
    // in these intervals are valid mul expressions
    let enabled_intervals = create_enabled_intervals(&do_indices, &dont_indices);
    // 4. Get all mul expressions with their indices from the text
    let mul_expressions = extract_mul_expressions_with_indices(text);
    // 5. From all mul expressions pick only those which are in one of the enabled intervals
    let enabled_mul_expressions =
        get_enabled_mull_expressions(&mul_expressions, &enabled_intervals);
    // 6. Extract numbers from those mul expressions which are in an enabled interval
    let numbers_list = extract_numbers(enabled_mul_expressions);

    // 7. Finally, sum the numbers
    sum_mul_pairs(numbers_list)
}
