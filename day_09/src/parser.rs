use crate::filesystem_processor::FREE_SPACE;

pub fn parse_input(text: &str) -> Vec<String> {
    let mut id = 0_u32;
    let mut file_blocks = Vec::new();

    for (index, character) in text.chars().enumerate() {
        let number = character.to_digit(10).unwrap();

        for _ in 0..number {
            if index % 2 == 0 {
                file_blocks.push(id.to_string());
            } else {
                file_blocks.push(String::from(FREE_SPACE));
            }
        }

        if index % 2 == 0 {
            id += 1;
        }
    }

    file_blocks
}

#[cfg(test)]
mod tests {
    use crate::parser::parse_input;

    const INPUT: &str = "2333133121414131402";

    #[test]
    fn parses_input_correctly() {
        let result = parse_input(INPUT);

        assert_eq!(
            result.join(""),
            "00...111...2...333.44.5555.6666.777.888899"
        );
    }
}
