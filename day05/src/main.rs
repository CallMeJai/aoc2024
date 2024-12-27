use std::collections::HashMap;
use std::collections::HashSet;
use std::cmp::Ordering;

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
    let result = part_2(input);
    println!("part 2: {result}");
}

type PageNumber = usize;

fn parse_input(input: &str) -> Result<(HashMap<PageNumber, HashSet<PageNumber>>, Vec<Vec<PageNumber>>), &str> {
    let mut rules = HashMap::new();
    let mut updates = Vec::new();
    let mut first_section = true;
    for line in input.split('\n') {
        if first_section {
            if line.is_empty() {
                first_section = false;
                continue;
            }
            if let Some((first, second)) = line.split_once('|') {
                rules.entry(first.parse::<usize>().unwrap())
                    .and_modify(|e: &mut HashSet<PageNumber>| {e.insert(second.parse::<usize>().unwrap());})
                    .or_insert(HashSet::from([second.parse::<usize>().unwrap()]));
            } else {
                return Err("Failed to parse rule");
            }
        } else if line.is_empty() {
            continue;
        } else {
            updates.push(line.split(',').map(|x| x.parse::<usize>().unwrap()).collect());
        }
    }
    Ok((rules, updates))

}

fn breaks_rules(update: &Vec<PageNumber>, rules: &HashMap<PageNumber, HashSet<PageNumber>>) -> bool {
    for (index, second_page) in update.iter().rev().enumerate() {
        for first_page in update.iter().rev().skip(index + 1) {
            if first_page == second_page {
                continue;
            }
            if rules.get(second_page).is_some_and(|x| x.contains(first_page)) {
                return true;
            }
        }
    }
    false
}

fn part_1(input: &str) -> usize {
    let mut result = 0;
    let (rules, updates) = parse_input(input).unwrap();
    for update in updates.into_iter() {
        if breaks_rules(&update, &rules) {
            continue;
        } else {
            result += update[update.len() / 2];
        }
    }
    result
}

fn correct_order(update: &mut Vec<PageNumber>, rules: &HashMap<PageNumber, HashSet<PageNumber>>) {
    update.sort_by(|a, b| {
        if rules.get(a).is_some_and(|x| x.contains(b)) {
            Ordering::Less
        } else if rules.get(b).is_some_and(|x| x.contains(a)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    });
}

fn part_2(input: &str) -> usize {
    let mut result = 0;
    let (rules, updates) = parse_input(input).unwrap();
    let mut updates_to_reorder: Vec<Vec<PageNumber>> = Vec::new();
    for update in updates.into_iter() {
        if breaks_rules(&update, &rules) {
            updates_to_reorder.push(update);
        }
    }
    for mut update in updates_to_reorder.into_iter() {
        correct_order(&mut update, &rules);
        result += update[update.len() / 2];
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_05_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_1(input);
        assert_eq!(result, 143);
    }

    #[test]
    fn day_05_part_2_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_2(input);
        assert_eq!(result, 123);
    }
}