use crate::common::is_update_correctly_ordered;
use std::collections::HashMap;

fn find_incorrectly_ordered_updates<'a>(
    rules: &Vec<Vec<u8>>,
    updates: &'a Vec<Vec<u8>>,
) -> Vec<&'a Vec<u8>> {
    updates
        .iter()
        .filter(|update| !is_update_correctly_ordered(rules, update))
        .collect()
}

fn create_rules_map(rules: &Vec<Vec<u8>>) -> HashMap<u8, Vec<u8>> {
    let mut pages_and_followers: HashMap<u8, Vec<u8>> = HashMap::new();

    for rule in rules.iter() {
        let page = rule[0];
        let follower = rule[1];

        if !pages_and_followers.contains_key(&page) {
            pages_and_followers.insert(page, vec![]);
        }

        let vector: &mut Vec<u8> = pages_and_followers.get_mut(&page).unwrap();
        vector.push(follower);
    }

    pages_and_followers
}

fn fix_incorrectly_ordered_update(rules_map: &HashMap<u8, Vec<u8>>, update: &Vec<u8>) -> Vec<u8> {
    let mut page_and_predecessors_count: Vec<(u8, usize)> = vec![];
    let mut fixed_update: Vec<u8> = vec![];

    let mut predecessors_count: usize;
    for page in update {
        predecessors_count = 0;

        for another_page in update {
            if page != another_page {
                let followers: &Vec<u8> = match rules_map.get(another_page) {
                    Some(vector) => vector,
                    None => &vec![],
                };

                if followers.contains(page) {
                    predecessors_count += 1;
                }
            }
        }

        page_and_predecessors_count.push((*page, predecessors_count));
    }

    page_and_predecessors_count.sort_by(|pair_a, pair_b| pair_a.1.cmp(&pair_b.1));
    for pair in page_and_predecessors_count {
        fixed_update.push(pair.0);
    }

    fixed_update
}

pub fn get_middle_page_sum_of_fixed_incorrectly_ordered_updates(
    rules: Vec<Vec<u8>>,
    updates: Vec<Vec<u8>>,
) -> usize {
    let incorrect_updates = find_incorrectly_ordered_updates(&rules, &updates);
    let rules_map = create_rules_map(&rules);

    incorrect_updates
        .iter()
        .map(|update| fix_incorrectly_ordered_update(&rules_map, &update))
        .map(|update| {
            let middle_index = update.len() / 2;

            update[middle_index] as usize
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::incorrectly_ordered_updates::{
        create_rules_map, find_incorrectly_ordered_updates, fix_incorrectly_ordered_update,
        get_middle_page_sum_of_fixed_incorrectly_ordered_updates,
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
    fn finds_all_incorrectly_ordered_updates() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = find_incorrectly_ordered_updates(&rules, &updates);

        assert_eq!(result.len(), 3);
        assert_eq!(result[0], &vec![75, 97, 47, 61, 53]);
        assert_eq!(result[1], &vec![61, 13, 29]);
        assert_eq!(result[2], &vec![97, 13, 75, 29, 47]);
    }

    #[test]
    fn creates_rules_map() {
        let (rules, _) = parse_input(INPUT).unwrap();
        let result = create_rules_map(&rules);
        let keys: Vec<u8> = result.keys().map(|item| *item).collect();

        assert_eq!(keys.contains(&47), true);
        assert_eq!(keys.contains(&97), true);
        assert_eq!(keys.contains(&75), true);
        assert_eq!(keys.contains(&61), true);
        assert_eq!(keys.contains(&29), true);
        assert_eq!(keys.contains(&53), true);
        assert_eq!(result.get(&47).unwrap(), &vec![53, 13, 61, 29]);
        assert_eq!(result.get(&97).unwrap(), &vec![13, 61, 47, 29, 53, 75]);
        assert_eq!(result.get(&75).unwrap(), &vec![29, 53, 47, 61, 13]);
        assert_eq!(result.get(&61).unwrap(), &vec![13, 53, 29]);
        assert_eq!(result.get(&29).unwrap(), &vec![13]);
        assert_eq!(result.get(&53).unwrap(), &vec![29, 13]);
    }

    #[test]
    fn fixes_incorrectly_ordered_update() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let incorrect_updates = find_incorrectly_ordered_updates(&rules, &updates);
        let rules_map = create_rules_map(&rules);
        let result1 = fix_incorrectly_ordered_update(&rules_map, &incorrect_updates[0]);
        let result2 = fix_incorrectly_ordered_update(&rules_map, &incorrect_updates[1]);
        let result3 = fix_incorrectly_ordered_update(&rules_map, &incorrect_updates[2]);

        assert_eq!(result1, vec![97, 75, 47, 61, 53]);
        assert_eq!(result2, vec![61, 29, 13]);
        assert_eq!(result3, vec![97, 75, 47, 29, 13]);
    }

    #[test]
    fn gets_correct_sum_after_fix() {
        let (rules, updates) = parse_input(INPUT).unwrap();
        let result = get_middle_page_sum_of_fixed_incorrectly_ordered_updates(rules, updates);

        assert_eq!(result, 123);
    }
}
