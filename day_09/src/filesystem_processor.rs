use crate::error::AdventOfCodeError;

pub const FREE_SPACE: &str = ".";

fn move_file_blocks(file_blocks: &Vec<String>) -> Vec<String> {
    let mut sorted_file_blocks = Vec::new();
    let mut right_index = file_blocks.len() - 1;

    for (left_index, block) in file_blocks.iter().enumerate() {
        if block == FREE_SPACE {
            while file_blocks[right_index] == FREE_SPACE {
                right_index -= 1;
            }
            sorted_file_blocks.push(file_blocks[right_index].clone());
            right_index -= 1;
        } else {
            sorted_file_blocks.push(block.clone())
        }

        if left_index >= right_index {
            break;
        }
    }

    sorted_file_blocks
}

pub fn check_sum(file_blocks: &Vec<String>) -> Result<usize, AdventOfCodeError> {
    let sorted_file_blocks = move_file_blocks(file_blocks);

    Ok(sorted_file_blocks
        .iter()
        .enumerate()
        .fold(0, |mut accumulator, item| {
            let (index, block) = item;

            accumulator + index * block.parse::<usize>().unwrap_or(0)
        }))
}

#[cfg(test)]
mod tests {
    use crate::filesystem_processor::{check_sum, move_file_blocks};
    use crate::parser::parse_input;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn moves_file_blocks_correctly() {
        let file_blocks = parse_input(INPUT);
        let result = move_file_blocks(&file_blocks);

        assert_eq!(result.join(""), "0099811188827773336446555566");
    }

    #[test]
    fn calculates_correct_check_sum() {
        let file_blocks = parse_input(INPUT);
        let result = check_sum(&file_blocks).unwrap();

        assert_eq!(result, 1928);
    }
}
