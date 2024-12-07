use crate::error::AdventOfCodeError;
use crate::xmas::Xmas;
use regex::Regex;

/// Searches for all substrings matching the given regex string in the text
///
/// Returns:
/// A Result containing number of substrings found by given regex in the horizontal direction
fn find_horizontal(text: &str, regex_string: &str) -> Result<usize, AdventOfCodeError> {
    let regex = Regex::new(regex_string)?;
    let matches_count = regex
        .find_iter(text)
        .map(|string_match| string_match.as_str())
        .count();

    Ok(matches_count)
}

/// Searches for all substrings matched in the text lines in the vertical direction
///
/// Returns:
/// A Result containing number of substrings found in the vertical direction
fn find_vertical(
    lines: &Vec<Vec<char>>,
    condition: fn(first: char, second: char, third: char, fourth: char) -> bool,
) -> usize {
    let mut index = 0;
    let mut count = 0;

    while index < lines.len() - 3 {
        let first_line = lines.get(index).unwrap();
        let second_line = lines.get(index + 1).unwrap();
        let third_line = lines.get(index + 2).unwrap();
        let fourth_line = lines.get(index + 3).unwrap();

        for (line_index, _) in first_line.iter().enumerate() {
            let first_char = first_line.get(line_index).unwrap();
            let second_char = second_line.get(line_index).unwrap();
            let third_char = third_line.get(line_index).unwrap();
            let fourth_char = fourth_line.get(line_index).unwrap();

            if condition(*first_char, *second_char, *third_char, *fourth_char) {
                count += 1;
            }
        }

        index += 1;
    }

    count
}

/// Searches for all substrings matched in the text lines in the diagonal direction from left to right
///
/// Returns:
/// A Result containing number of substrings found in the diagonal direction from left to right
fn find_diagonal_left_to_right(
    lines: &Vec<Vec<char>>,
    condition: fn(first: char, second: char, third: char, fourth: char) -> bool,
) -> usize {
    let mut index = 0;
    let mut count = 0;

    while index < lines.len() - 3 {
        let first_line = lines.get(index).unwrap();
        let second_line = lines.get(index + 1).unwrap();
        let third_line = lines.get(index + 2).unwrap();
        let fourth_line = lines.get(index + 3).unwrap();

        let mut line_index = 0;
        while line_index < first_line.len() - 3 {
            let first_char = first_line.get(line_index).unwrap();
            let second_char = second_line.get(line_index + 1).unwrap();
            let third_char = third_line.get(line_index + 2).unwrap();
            let fourth_char = fourth_line.get(line_index + 3).unwrap();

            if condition(*first_char, *second_char, *third_char, *fourth_char) {
                count += 1;
            }

            line_index += 1;
        }

        index += 1;
    }

    count
}

/// Searches for all substrings matched in the text lines in the diagonal direction from right to left
///
/// Returns:
/// A Result containing number of substrings found in the diagonal direction from right to left
fn find_diagonal_right_to_left(
    lines: &Vec<Vec<char>>,
    condition: fn(first: char, second: char, third: char, fourth: char) -> bool,
) -> usize {
    let mut index = 0;
    let mut count = 0;

    while index < lines.len() - 3 {
        let first_line = lines.get(index).unwrap();
        let second_line = lines.get(index + 1).unwrap();
        let third_line = lines.get(index + 2).unwrap();
        let fourth_line = lines.get(index + 3).unwrap();

        let mut line_index = 3;
        while line_index < first_line.len() {
            let first_char = first_line.get(line_index).unwrap();
            let second_char = second_line.get(line_index - 1).unwrap();
            let third_char = third_line.get(line_index - 2).unwrap();
            let fourth_char = fourth_line.get(line_index - 3).unwrap();

            if condition(*first_char, *second_char, *third_char, *fourth_char) {
                count += 1;
            }

            line_index += 1;
        }

        index += 1;
    }

    count
}

/// Gets an Xmas structure representing number of "XMAS" substrings found in each direction
///
/// Returns:
/// A Result containing number of substrings found in the diagonal direction from right to left
pub fn find_all_xmas_substrings(text: &str) -> Result<Xmas, AdventOfCodeError> {
    // Horizontal search
    let horizontal_left_to_right_search_count = find_horizontal(&text, r"XMAS")?;
    let horizontal_right_to_left_search_count = find_horizontal(&text, r"SAMX")?;

    let lines: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let forwards_search_condition = |first: char, second: char, third: char, fourth: char| {
        first.eq(&'X') && second.eq(&'M') && third.eq(&'A') && fourth.eq(&'S')
    };
    let backwards_search_condition = |first: char, second: char, third: char, fourth: char| {
        first.eq(&'S') && second.eq(&'A') && third.eq(&'M') && fourth.eq(&'X')
    };

    // Vertical search
    let vertical_top_to_bottom_search_count = find_vertical(&lines, forwards_search_condition);
    let vertical_bottom_to_top_search_count = find_vertical(&lines, backwards_search_condition);

    // Diagonal left to right
    let diagonal_left_to_right_top_to_bottom_search_count =
        find_diagonal_left_to_right(&lines, forwards_search_condition);
    let diagonal_left_to_right_bottom_to_top_search_count =
        find_diagonal_left_to_right(&lines, backwards_search_condition);

    // Diagonal right to left search
    let diagonal_right_to_left_top_to_bottom_search_count =
        find_diagonal_right_to_left(&lines, forwards_search_condition);
    let diagonal_right_to_left_bottom_to_top_search_count =
        find_diagonal_right_to_left(&lines, backwards_search_condition);

    Ok(Xmas::new(
        horizontal_left_to_right_search_count,
        horizontal_right_to_left_search_count,
        vertical_top_to_bottom_search_count,
        vertical_bottom_to_top_search_count,
        diagonal_left_to_right_top_to_bottom_search_count,
        diagonal_left_to_right_bottom_to_top_search_count,
        diagonal_right_to_left_top_to_bottom_search_count,
        diagonal_right_to_left_bottom_to_top_search_count,
    ))
}

#[cfg(test)]
mod tests {
    use crate::find_xmas_substrings::find_all_xmas_substrings;

    #[test]
    fn finds_all_xmas_substrings() {
        let input = "\
MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

        let result = find_all_xmas_substrings(input).unwrap();

        assert_eq!(result.horizontal_left_to_right_search_count, 3);
        assert_eq!(result.horizontal_right_to_left_search_count, 2);
        assert_eq!(result.vertical_top_to_bottom_search_count, 1);
        assert_eq!(result.vertical_bottom_to_top_search_count, 2);
        assert_eq!(result.diagonal_left_to_right_top_to_bottom_search_count, 1);
        assert_eq!(result.diagonal_left_to_right_bottom_to_top_search_count, 4);
        assert_eq!(result.diagonal_right_to_left_top_to_bottom_search_count, 1);
        assert_eq!(result.diagonal_right_to_left_bottom_to_top_search_count, 4);
        assert_eq!(result.total_count, 18);
    }
}
