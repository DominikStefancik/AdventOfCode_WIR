use std::fs;
use std::io::Error;

fn main() -> Result<(), Error> {
    let (vector1, vector2) = parse_input().expect("Error parsing the input file");

    if vector1.len() != vector2.len() {
        panic!("Total number of left numbers is not equal to the total number of right numbers")
    }

    let sum_of_differences = get_sum_of_differences(vector1, vector2);

    println!("The sum of differences is: {sum_of_differences}");

    Ok(())
}

fn parse_input() -> Result<(Vec<u32>, Vec<u32>), Error> {
    let input_string =
        fs::read_to_string("./input.txt").expect("Error while reading the input file");
    let mut vector1: Vec<u32> = Vec::new();
    let mut vector2: Vec<u32> = Vec::new();

    for line in input_string.lines() {
        let mut split_iter = line.split("   "); // number are separated by 3 spaces

        let left_number: u32 = split_iter
            .next()
            .expect("Missing left number")
            .trim()
            .parse()
            .expect("Error parsing left number");

        let right_number: u32 = split_iter
            .next()
            .expect("Missing right number")
            .trim()
            .parse()
            .expect("Error parsing right number");

        vector1.push(left_number);
        vector2.push(right_number);
    }

    Ok((vector1, vector2))
}

fn get_sum_of_differences(mut vector1: Vec<u32>, mut vector2: Vec<u32>) -> u32 {
    vector1.sort();
    vector2.sort();

    vector1
        .iter()
        .zip(vector2.iter())
        .map(|pair| pair.0.abs_diff(*pair.1))
        .sum()
}
