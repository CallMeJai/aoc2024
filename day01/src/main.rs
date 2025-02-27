use std::collections::{BTreeMap, BinaryHeap};

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let (left, right) = parse_to_heaps(input);
    let result = total_distance(left, right);
    println!("part 1: {result}");

    let input = include_str!("../rsrc/input.txt");
    let (left, right) = parse_to_hists(input);
    let result = similarity_score(left, right);
    println!("part 2: {result}")
}

fn parse_to_heaps(input: &str) -> (BinaryHeap<usize>, BinaryHeap<usize>) {
    let mut left_heap = BinaryHeap::new();
    let mut right_heap = BinaryHeap::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            if let Ok(l) = left.parse::<usize>() {
                left_heap.push(l);
            } else {
                eprintln!("Could not parse {left}!");
            }
            if let Ok(r) = right.parse::<usize>() {
                right_heap.push(r);
            } else {
                eprintln!("Could not parse {right}!");
            }
        } else {
            eprintln!("Malformed input!\n{line}");
        }
    }
    (left_heap, right_heap)
}

fn total_distance(left: BinaryHeap<usize>, right: BinaryHeap<usize>) -> usize {
    let mut sum= 0;
    for (l, r) in left.into_sorted_vec().into_iter().zip(right.into_sorted_vec()) {
        sum += l.abs_diff(r);
    }
    sum
}

fn parse_to_hists(input: &str) -> (BTreeMap<usize, usize>, BTreeMap<usize, usize>) {
    let mut left_map = BTreeMap::new();
    let mut right_map = BTreeMap::new();

    for line in input.lines() {
        if let Some((left, right)) = line.split_once("   ") {
            if let Ok(l) = left.parse::<usize>() {
                left_map.entry(l)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            } else {
                eprintln!("Could not parse {left}!");
            }
            if let Ok(r) = right.parse::<usize>() {
                right_map.entry(r)
                    .and_modify(|n| *n += 1)
                    .or_insert(1);
            } else {
                eprintln!("Could not parse {right}!");
            }
        } else {
            eprintln!("Malformed input!\n{line}");
        }
    }
    (left_map, right_map)
}

fn similarity_score(l_hist: BTreeMap<usize, usize>, r_hist: BTreeMap<usize, usize>) -> usize {
    let mut score = 0;
    for (location_id, freq) in l_hist.iter() {
        score += location_id * r_hist.get(location_id).unwrap_or(&0) * freq;
    }
    score
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        let (left, right) = parse_to_heaps(input);
        let result = total_distance(left, right);
        assert_eq!(result, 11);
    }

    #[test]
    fn day_01_part_2_test() {
        let input = include_str!("../rsrc/test.txt");
        let (left, right) = parse_to_hists(input);
        let result = similarity_score(left, right);
        assert_eq!(result, 31);
    }
}