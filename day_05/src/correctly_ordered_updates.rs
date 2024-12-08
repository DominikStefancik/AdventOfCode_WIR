use crate::common::is_update_correctly_ordered;

fn find_correctly_ordered_updates<'a>(
    rules: &Vec<Vec<u8>>,
    updates: &'a Vec<Vec<u8>>,
) -> Vec<&'a Vec<u8>> {
    updates
        .iter()
        .filter(|update| is_update_correctly_ordered(rules, update))
        .collect()
}

pub fn get_middle_page_sum_of_correctly_ordered_updates(
    rules: &Vec<Vec<u8>>,
    updates: &Vec<Vec<u8>>,
) -> usize {
    let correct_updates = find_correctly_ordered_updates(rules, updates);

    correct_updates
        .iter()
        .map(|update| {
            let middle_index = update.len() / 2;

            update[middle_index] as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::correctly_ordered_updates::{
        find_correctly_ordered_updates, get_middle_page_sum_of_correctly_ordered_updates,
    };
    use crate::parse::parse_input;

    const INPUT: &str = "\
47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn finds_all_correctly_ordered_updates() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = find_correctly_ordered_updates(&rules, &updates);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &vec![75, 47, 61, 53, 29]);
        assert_eq!(result[1], &vec![97, 61, 53, 29, 13]);
        assert_eq!(result[2], &vec![75, 29, 13]);
    }

    #[test]
    fn gets_correct_sum() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = get_middle_page_sum_of_correctly_ordered_updates(&rules, &updates);

        assert_eq!(result, 143);
    }
}
