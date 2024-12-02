use std::collections::BinaryHeap;

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let (left, right) = parse_lists(input);
    let result = total_distance(left, right);
    println!("{result}");
}

fn parse_lists(input: &str) -> (BinaryHeap<usize>, BinaryHeap<usize>) {
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_01_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        let (left, right) = parse_lists(input);
        let result = total_distance(left, right);
        assert_eq!(result, 11);
    }
}