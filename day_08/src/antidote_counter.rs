use std::collections::{HashMap, HashSet};

use crate::antenna::Antenna;

const FREE_SPACE: char = '.';

#[derive(Debug)]
enum AntidoteDirection {
    FromAntenna1,
    FromAntenna2,
}

fn create_antennas_map(map: &Vec<Vec<char>>) -> HashMap<char, Vec<Antenna>> {
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
    territory_dimension: (usize, usize),
    pair: (&Antenna, &Antenna),
    direction: AntidoteDirection,
) -> Option<(isize, isize)> {
    let (antenna1, antenna2) = pair;
    let row_difference = antenna1.row_index.abs_diff(antenna2.row_index) as isize;
    let column_difference = antenna1.column_index.abs_diff(antenna2.column_index) as isize;
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

    let is_antidote_x_in_map = antidote_x >= 0 && antidote_x < territory_dimension.0 as isize;
    let is_antidote_y_in_map = antidote_y >= 0 && antidote_y < territory_dimension.1 as isize;

    if is_antidote_x_in_map && is_antidote_y_in_map {
        return Some((antidote_x, antidote_y));
    }

    None
}

pub fn calculate_antidotes_count(territory_map: &Vec<Vec<char>>) -> usize {
    let territory_dimension = (territory_map.len(), territory_map[0].len());
    let antennas_map = create_antennas_map(territory_map);
    let mut unique_antidotes = HashSet::new();

    for key in antennas_map.keys() {
        let antennas = antennas_map.get(key).unwrap();
        let antenna_pairs = create_antenna_pairs(antennas);

        for pair in antenna_pairs {
            let antidote_position =
                get_antidote_position(territory_dimension, pair, AntidoteDirection::FromAntenna1);
            if antidote_position.is_some() {
                unique_antidotes.insert(antidote_position.unwrap());
            }

            let antidote_position =
                get_antidote_position(territory_dimension, pair, AntidoteDirection::FromAntenna2);
            if antidote_position.is_some() {
                unique_antidotes.insert(antidote_position.unwrap());
            }
        }
    }

    unique_antidotes.len()
}

#[cfg(test)]
mod tests {
    use crate::antenna::Antenna;
    use crate::antidote_counter::{
        calculate_antidotes_count, create_antenna_pairs, create_antennas_map,
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
        let result = calculate_antidotes_count(&mut map);

        assert_eq!(result, 14);
    }
}
