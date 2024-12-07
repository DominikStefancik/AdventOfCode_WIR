fn is_update_correctly_ordered(rules: &Vec<Vec<u8>>, update: &Vec<u8>) -> bool {
    let mut rule: Option<&Vec<u8>>;

    for (page_index, page_number) in update.iter().enumerate() {
        for following_number in update.iter().skip(page_index + 1) {
            rule = rules
                .iter()
                .find(|rule| rule[0] == *page_number && rule[1] == *following_number);

            if rule.is_none() {
                return false;
            }
        }
    }

    true
}

fn find_correctly_ordered_updates(rules: Vec<Vec<u8>>, updates: &Vec<Vec<u8>>) -> Vec<&Vec<u8>> {
    updates
        .iter()
        .filter(|update| is_update_correctly_ordered(&rules, update))
        .collect()
}

pub fn get_middle_page_sum_of_correctly_ordered_updates(
    rules: Vec<Vec<u8>>,
    updates: Vec<Vec<u8>>,
) -> usize {
    let correct_updates = find_correctly_ordered_updates(rules, &updates);

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
        is_update_correctly_ordered,
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
    fn update_is_correct() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = is_update_correctly_ordered(&rules, updates.get(0).unwrap());

        assert_eq!(result, true);
    }

    #[test]
    fn updates_are_incorrect() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result1 = is_update_correctly_ordered(&rules, updates.get(3).unwrap());
        let result2 = is_update_correctly_ordered(&rules, updates.get(4).unwrap());
        let result3 = is_update_correctly_ordered(&rules, updates.get(5).unwrap());

        assert_eq!(result1, false);
        assert_eq!(result2, false);
        assert_eq!(result3, false);
    }

    #[test]
    fn finds_all_correctly_ordered_updates() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = find_correctly_ordered_updates(rules, &updates);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &vec![75, 47, 61, 53, 29]);
        assert_eq!(result[1], &vec![97, 61, 53, 29, 13]);
        assert_eq!(result[2], &vec![75, 29, 13]);
    }

    #[test]
    fn gets_correct_sum() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = get_middle_page_sum_of_correctly_ordered_updates(rules, updates);

        assert_eq!(result, 143);
    }
}
