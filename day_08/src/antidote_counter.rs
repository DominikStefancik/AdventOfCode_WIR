use std::collections::{HashMap, HashSet};

use crate::antenna::Antenna;

const FREE_SPACE: char = '.';

#[derive(Debug)]
enum AntidoteDirection {
    FromAntenna1,
    FromAntenna2,
}

pub fn create_antennas_map(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Antenna>> {
    let mut antennas_map: HashMap<char, Vec<Antenna>> = HashMap::new();

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, character) in row.iter().enumerate() {
            if *character != FREE_SPACE {
                if !antennas_map.contains_key(character) {
                    antennas_map.insert(*character, Vec::new());
                }

                let antenna = Antenna {
                    character: *character,
                    row_index,
                    column_index,
                };
                antennas_map.get_mut(character).unwrap().push(antenna);
            }
        }
    }

    antennas_map
}

fn create_antenna_pairs(antennas: &Vec<Antenna>) -> Vec<(&Antenna, &Antenna)> {
    let mut antenna_pairs: Vec<(&Antenna, &Antenna)> = Vec::new();

    for (index, first_antenna) in antennas.iter().enumerate() {
        for second_antenna in antennas.iter().skip(index + 1) {
            antenna_pairs.push((first_antenna, second_antenna));
        }
    }

    antenna_pairs
}

fn get_antidote_position(
    pair: (&Antenna, &Antenna),
    direction: AntidoteDirection,
    row_difference: isize,
    column_difference: isize,
) -> (isize, isize) {
    let (antenna1, antenna2) = pair;
    let antidote_x;
    let antidote_y;

    match direction {
        AntidoteDirection::FromAntenna1 => {
            antidote_x = antenna1.row_index as isize - row_difference;
            if antenna1.column_index <= antenna2.column_index {
                antidote_y = antenna1.column_index as isize - column_difference;
            } else {
                antidote_y = antenna1.column_index as isize + column_difference;
            }
        }
        AntidoteDirection::FromAntenna2 => {
            antidote_x = antenna2.row_index as isize + row_difference;
            if antenna1.column_index <= antenna2.column_index {
                antidote_y = antenna2.column_index as isize + column_difference;
            } else {
                antidote_y = antenna2.column_index as isize - column_difference;
            }
        }
    }

    (antidote_x, antidote_y)
}

fn is_antidote_in_map(antidote: (isize, isize), territory_dimension: (usize, usize)) -> bool {
    let (antidote_x, antidote_y) = antidote;
    let is_antidote_x_in_map = antidote_x >= 0 && antidote_x < territory_dimension.0 as isize;
    let is_antidote_y_in_map = antidote_y >= 0 && antidote_y < territory_dimension.1 as isize;

    is_antidote_x_in_map && is_antidote_y_in_map
}

fn get_antidote(
    territory_dimension: (usize, usize),
    pair: (&Antenna, &Antenna),
    direction: AntidoteDirection,
    row_difference: isize,
    column_difference: isize,
) -> Option<(isize, isize)> {
    let antidote = get_antidote_position(pair, direction, row_difference, column_difference);

    if is_antidote_in_map(antidote, territory_dimension) {
        return Some(antidote);
    }

    None
}

pub fn calculate_antidotes_count(
    antennas_map: &HashMap<char, Vec<Antenna>>,
    territory_dimension: (usize, usize),
) -> usize {
    let mut unique_antidotes = HashSet::new();

    for key in antennas_map.keys() {
        let antennas = antennas_map.get(key).unwrap();
        let antenna_pairs = create_antenna_pairs(antennas);

        for pair in antenna_pairs {
            let (antenna1, antenna2) = pair;
            let row_difference = antenna1.row_index.abs_diff(antenna2.row_index) as isize;
            let column_difference = antenna1.column_index.abs_diff(antenna2.column_index) as isize;

            if let Some(antidote) = get_antidote(
                territory_dimension,
                pair,
                AntidoteDirection::FromAntenna1,
                row_difference,
                column_difference,
            ) {
                unique_antidotes.insert(antidote);
            }

            if let Some(antidote) = get_antidote(
                territory_dimension,
                pair,
                AntidoteDirection::FromAntenna2,
                row_difference,
                column_difference,
            ) {
                unique_antidotes.insert(antidote);
            }
        }
    }

    unique_antidotes.len()
}

pub fn calculate_antidotes_at_any_grid_count(
    antennas_map: &HashMap<char, Vec<Antenna>>,
    territory_dimension: (usize, usize),
) -> usize {
    let mut unique_antidotes = HashSet::new();

    for key in antennas_map.keys() {
        let antennas = antennas_map.get(key).unwrap();
        let antenna_pairs = create_antenna_pairs(antennas);

        for pair in antenna_pairs {
            let (antenna1, antenna2) = pair;
            let row_difference = antenna1.row_index.abs_diff(antenna2.row_index) as isize;
            let column_difference = antenna1.column_index.abs_diff(antenna2.column_index) as isize;

            // In the "at any grid" case, all antennas are also antidotes
            unique_antidotes.insert((antenna1.row_index as isize, antenna1.column_index as isize));
            unique_antidotes.insert((antenna2.row_index as isize, antenna2.column_index as isize));

            let mut multiplier = 1_isize;
            // continue increasing the row_difference and column_difference until a possible
            // position of an antidote is outside the map boundaries
            while let Some(antidote) = get_antidote(
                territory_dimension,
                pair,
                AntidoteDirection::FromAntenna1,
                row_difference * multiplier,
                column_difference * multiplier,
            ) {
                unique_antidotes.insert(antidote);
                multiplier += 1;
            }

            multiplier = 1;
            while let Some(antidote) = get_antidote(
                territory_dimension,
                pair,
                AntidoteDirection::FromAntenna2,
                row_difference * multiplier,
                column_difference * multiplier,
            ) {
                unique_antidotes.insert(antidote);
                multiplier += 1;
            }
        }
    }

    unique_antidotes.len()
}

#[cfg(test)]
mod tests {
    use crate::antenna::Antenna;
    use crate::antidote_counter::{
        calculate_antidotes_at_any_grid_count, calculate_antidotes_count, create_antenna_pairs,
        create_antennas_map,
    };
    use crate::parser::parse_input;

    const INPUT: &str = "\
............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn gets_correct_antennas_map() {
        let mut map = parse_input(INPUT);
        let result = create_antennas_map(&mut map);

        assert_eq!(result.keys().count(), 2);
        assert_eq!(result.contains_key(&'0'), true);
        assert_eq!(result.contains_key(&'A'), true);
        assert_eq!(
            *result.get(&'0').unwrap(),
            vec![
                Antenna {
                    character: '0',
                    row_index: 1,
                    column_index: 8
                },
                Antenna {
                    character: '0',
                    row_index: 2,
                    column_index: 5
                },
                Antenna {
                    character: '0',
                    row_index: 3,
                    column_index: 7
                },
                Antenna {
                    character: '0',
                    row_index: 4,
                    column_index: 4
                }
            ]
        );
        assert_eq!(
            *result.get(&'A').unwrap(),
            vec![
                Antenna {
                    character: 'A',
                    row_index: 5,
                    column_index: 6
                },
                Antenna {
                    character: 'A',
                    row_index: 8,
                    column_index: 8
                },
                Antenna {
                    character: 'A',
                    row_index: 9,
                    column_index: 9
                },
            ]
        );
    }

    #[test]
    fn gets_correct_antenna_pairs() {
        let mut map = parse_input(INPUT);
        let result = create_antennas_map(&mut map);
        let antenna_0_pairs = create_antenna_pairs(result.get(&'0').unwrap());
        let antenna_A_pairs = create_antenna_pairs(result.get(&'A').unwrap());

        assert_eq!(
            antenna_0_pairs,
            vec![
                (
                    &Antenna {
                        character: '0',
                        row_index: 1,
                        column_index: 8
                    },
                    &Antenna {
                        character: '0',
                        row_index: 2,
                        column_index: 5
                    }
                ),
                (
                    &Antenna {
                        character: '0',
                        row_index: 1,
                        column_index: 8
                    },
                    &Antenna {
                        character: '0',
                        row_index: 3,
                        column_index: 7
                    }
                ),
                (
                    &Antenna {
                        character: '0',
                        row_index: 1,
                        column_index: 8
                    },
                    &Antenna {
                        character: '0',
                        row_index: 4,
                        column_index: 4
                    }
                ),
                (
                    &Antenna {
                        character: '0',
                        row_index: 2,
                        column_index: 5
                    },
                    &Antenna {
                        character: '0',
                        row_index: 3,
                        column_index: 7
                    }
                ),
                (
                    &Antenna {
                        character: '0',
                        row_index: 2,
                        column_index: 5
                    },
                    &Antenna {
                        character: '0',
                        row_index: 4,
                        column_index: 4
                    }
                ),
                (
                    &Antenna {
                        character: '0',
                        row_index: 3,
                        column_index: 7
                    },
                    &Antenna {
                        character: '0',
                        row_index: 4,
                        column_index: 4
                    }
                )
            ]
        );
        assert_eq!(
            antenna_A_pairs,
            vec![
                (
                    &Antenna {
                        character: 'A',
                        row_index: 5,
                        column_index: 6
                    },
                    &Antenna {
                        character: 'A',
                        row_index: 8,
                        column_index: 8
                    }
                ),
                (
                    &Antenna {
                        character: 'A',
                        row_index: 5,
                        column_index: 6
                    },
                    &Antenna {
                        character: 'A',
                        row_index: 9,
                        column_index: 9
                    }
                ),
                (
                    &Antenna {
                        character: 'A',
                        row_index: 8,
                        column_index: 8
                    },
                    &Antenna {
                        character: 'A',
                        row_index: 9,
                        column_index: 9
                    }
                )
            ]
        );
    }

    #[test]
    fn gets_correct_antidotes_count() {
        let mut map = parse_input(INPUT);
        let map_dimensions = (map.len(), map[0].len());
        let antennas_map = create_antennas_map(&mut map);
        let result = calculate_antidotes_count(&antennas_map, map_dimensions);

        assert_eq!(result, 14);
    }

    #[test]
    fn gets_correct_antidotes_at_any_grid_count() {
        let mut map = parse_input(INPUT);
        let map_dimensions = (map.len(), map[0].len());
        let antennas_map = create_antennas_map(&mut map);
        let result = calculate_antidotes_at_any_grid_count(&antennas_map, map_dimensions);

        assert_eq!(result, 34);
    }
}
