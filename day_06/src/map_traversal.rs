use crate::direction::Direction;

const GUARD_SYMBOL: char = '^';
const OBSTACLE_SYMBOL: char = '#';
const VISITED_POSITION_SYMBOL: char = 'X';

fn find_guard_in_the_map(map: &mut Vec<Vec<char>>) -> (isize, isize) {
    for (row_index, row) in map.iter().enumerate() {
        for (column_index, symbol) in row.iter().enumerate() {
            if symbol == &GUARD_SYMBOL {
                return (row_index as isize, column_index as isize);
            }
        }
    }

    (0, 0)
}

fn change_guard_direction(direction: &Direction) -> Direction {
    match direction {
        Direction::UP => Direction::RIGHT,
        Direction::DOWN => Direction::LEFT,
        Direction::LEFT => Direction::UP,
        Direction::RIGHT => Direction::DOWN,
    }
}

fn next_move_position(direction: &Direction, guard_position: &(isize, isize)) -> (isize, isize) {
    match &direction {
        Direction::UP => (guard_position.0 - 1, guard_position.1),
        Direction::DOWN => (guard_position.0 + 1, guard_position.1),
        Direction::LEFT => (guard_position.0, guard_position.1 - 1),
        Direction::RIGHT => (guard_position.0, guard_position.1 + 1),
    }
}

fn is_next_step_the_way_out(
    map_size: (usize, usize),
    direction: &Direction,
    position_to_check: (isize, isize),
) -> bool {
    match direction {
        Direction::UP => position_to_check.0 == -1,
        Direction::DOWN => position_to_check.0 == map_size.0 as isize,
        Direction::LEFT => position_to_check.1 == -1,
        Direction::RIGHT => position_to_check.1 == map_size.1 as isize,
    }
}

pub fn number_of_moves_until_exit(mut map: Vec<Vec<char>>) -> usize {
    let map_size = (map.len(), map[0].len());
    let mut visited_positions_count = 1;
    let mut direction = Direction::default();
    let mut guard_position = find_guard_in_the_map(&mut map);
    let mut next_possible_move_position;
    let mut next_character;

    // at the very beginning, we mark the guard's starting position as visited
    map[guard_position.0 as usize][guard_position.1 as usize] = VISITED_POSITION_SYMBOL;

    loop {
        next_possible_move_position = next_move_position(&direction, &guard_position);

        if is_next_step_the_way_out(map_size, &direction, next_possible_move_position) {
            return visited_positions_count;
        }

        next_character =
            map[next_possible_move_position.0 as usize][next_possible_move_position.1 as usize];

        if next_character == OBSTACLE_SYMBOL {
            direction = change_guard_direction(&direction);
        } else {
            guard_position = next_possible_move_position;

            if next_character != VISITED_POSITION_SYMBOL {
                map[next_possible_move_position.0 as usize]
                    [next_possible_move_position.1 as usize] = VISITED_POSITION_SYMBOL;
                visited_positions_count += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::direction::Direction;
    use crate::map_traversal::{
        find_guard_in_the_map, is_next_step_the_way_out, number_of_moves_until_exit,
    };
    use crate::parse::parse_input;

    const INPUT: &str = "\
....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn gets_correct_initial_coordinates_of_guard() {
        let mut map = parse_input(INPUT);
        let result = find_guard_in_the_map(&mut map);

        assert_eq!(result, (6, 4));
    }

    #[test]
    fn checks_next_step_way_out_correctly() {
        let map_size = (10, 10);

        // middle of the map going UP
        let result1 = is_next_step_the_way_out(map_size, &Direction::UP, (7, 4));
        // on the left border going UP
        let result2 = is_next_step_the_way_out(map_size, &Direction::UP, (7, -1));
        // on the top border going UP
        let result3 = is_next_step_the_way_out(map_size, &Direction::UP, (-1, 4));
        // on the right border going DOWN
        let result4 = is_next_step_the_way_out(map_size, &Direction::DOWN, (7, 10));
        // on the bottom border going DOWN
        let result5 = is_next_step_the_way_out(map_size, &Direction::DOWN, (10, 5));
        // on the top border going LEFT
        let result6 = is_next_step_the_way_out(map_size, &Direction::LEFT, (-1, 3));
        // on the left border going LEFT
        let result7 = is_next_step_the_way_out(map_size, &Direction::LEFT, (4, -1));
        // on the bottom border going RIGHT
        let result8 = is_next_step_the_way_out(map_size, &Direction::RIGHT, (10, 9));
        // on the right border going RIGHT
        let result9 = is_next_step_the_way_out(map_size, &Direction::RIGHT, (10, 10));

        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, true);
        assert_eq!(result4, false);
        assert_eq!(result5, true);
        assert_eq!(result6, false);
        assert_eq!(result7, true);
        assert_eq!(result8, false);
        assert_eq!(result9, true);
    }

    #[test]
    fn gets_correct_number_of_moves() {
        let map = parse_input(INPUT);
        let result = number_of_moves_until_exit(map);

        assert_eq!(result, 41);
    }
}
