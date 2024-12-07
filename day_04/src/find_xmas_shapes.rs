/// Takes characters from 3 lines and checks if they form an x-mas shape
///
/// Returns:
/// True if characters on the lines form the x-mas shape
fn is_xmas_shape(
    first_line_chars: (&char, &char),
    second_line_char: &char,
    third_line_chars: (&char, &char),
) -> bool {
    // on the second line we are only interested in ona character which always has to be A
    // if it is not, we don't have to check characters on other lines
    if !second_line_char.eq(&'A') {
        return false;
    }

    let are_diagonals_top_to_bottom = first_line_chars.0.eq(&'M')
        && first_line_chars.1.eq(&'M')
        && third_line_chars.0.eq(&'S')
        && third_line_chars.1.eq(&'S');

    let are_diagonals_bottom_to_top = first_line_chars.0.eq(&'S')
        && first_line_chars.1.eq(&'S')
        && third_line_chars.0.eq(&'M')
        && third_line_chars.1.eq(&'M');

    let are_diagonals_different_directions = (first_line_chars.0.eq(&'M')
        && first_line_chars.1.eq(&'S')
        && third_line_chars.0.eq(&'M')
        && third_line_chars.1.eq(&'S'))
        || (first_line_chars.0.eq(&'S')
            && first_line_chars.1.eq(&'M')
            && third_line_chars.0.eq(&'S')
            && third_line_chars.1.eq(&'M'));

    are_diagonals_top_to_bottom || are_diagonals_bottom_to_top || are_diagonals_different_directions
}

/// Searches for all x-mas shapes in the text and returns their count
///
/// Returns:
/// A number of all x-mas shapes
pub fn find_all_xmas_shapes(text: &str) -> usize {
    let lines: Vec<Vec<char>> = text.lines().map(|line| line.chars().collect()).collect();
    let mut index = 0;
    let mut count = 0;

    while index < lines.len() - 2 {
        let first_line = lines.get(index).unwrap();
        let second_line = lines.get(index + 1).unwrap();
        let third_line = lines.get(index + 2).unwrap();

        let mut line_index = 0;
        while line_index < first_line.len() - 2 {
            let first_line_chars = (
                first_line.get(line_index).unwrap(),
                first_line.get(line_index + 2).unwrap(),
            );
            let second_line_char = second_line.get(line_index + 1).unwrap();
            let third_line_chars = (
                third_line.get(line_index).unwrap(),
                third_line.get(line_index + 2).unwrap(),
            );

            if is_xmas_shape(first_line_chars, second_line_char, third_line_chars) {
                count += 1;
            }

            line_index += 1;
        }

        index += 1;
    }

    count
}

#[cfg(test)]
mod tests {
    use crate::find_xmas_shapes::find_all_xmas_shapes;

    #[test]
    fn finds_all_xmas_shapes() {
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

        let result = find_all_xmas_shapes(input);
        assert_eq!(result, 9);
    }
}
