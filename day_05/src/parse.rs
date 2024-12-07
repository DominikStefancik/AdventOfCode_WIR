use crate::error::AdventOfCodeError;

const RULES_SEPARATOR: &str = "|";
const UPDATES_SEPARATOR: &str = ",";

pub fn parse_input(text: &str) -> Result<(Vec<Vec<u8>>, Vec<Vec<u8>>), AdventOfCodeError> {
    let mut rules: Vec<Vec<u8>> = vec![];
    let mut updates: Vec<Vec<u8>> = vec![];

    for line in text.lines() {
        if line.contains(RULES_SEPARATOR) {
            let parts = line
                .split(RULES_SEPARATOR)
                .map(|item| {
                    item.trim()
                        .parse()
                        .map_err(|err| AdventOfCodeError::ParseError(err))
                })
                .collect::<Result<Vec<u8>, AdventOfCodeError>>()?;
            rules.push(parts);
        } else if line.contains(UPDATES_SEPARATOR) {
            let parts = line
                .split(UPDATES_SEPARATOR)
                .map(|item| {
                    item.trim()
                        .parse()
                        .map_err(|err| AdventOfCodeError::ParseError(err))
                })
                .collect::<Result<Vec<u8>, AdventOfCodeError>>()?;
            updates.push(parts);
        }
    }

    Ok((rules, updates))
}
