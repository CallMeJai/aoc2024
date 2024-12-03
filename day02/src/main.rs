fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
    let result = part_2(input);
    println!("part 2: {result}");
}

enum Direction {
    Decreasing,
    Increasing,
    Unset
}

fn is_simply_safe(splits: &mut dyn Iterator<Item = &str>) -> bool {
    let mut direction = Direction::Unset;
    let mut current = splits.next()
            .expect("split doesn't have a first element").parse::<i64>()
            .expect("couldn't parse value");
    for split in splits {
        let next = split.parse::<i64>().expect("couldn't parse value");
        match current - next {
            ..-3 => return false,
            -3..0 => match direction {
                        Direction::Unset => direction = Direction::Decreasing,
                        Direction::Increasing => return false,
                        Direction::Decreasing => (),
                    },
            0 => return false,
            1..4 => match direction {
                        Direction::Unset => direction = Direction::Increasing,
                        Direction::Increasing => (),
                        Direction::Decreasing => return false,
                    },
            4.. => return false,
        }
        current = next;
    }
    true
}

fn part_1(input: &str) -> usize {
    let mut safe_count = 0;
    for line in input.lines() {
        let mut splits = line.split(' ');
        if is_simply_safe(&mut splits) {
            safe_count += 1;
        }
    }
    safe_count
}

fn part_2(input: &str) -> usize {
    let mut safe_count = 0;
    'line: for line in input.lines() {
        let splits = line.split(' ');
        if is_simply_safe(&mut splits.clone()) {
            safe_count += 1;
            continue 'line;
        }
        for i in 0..splits.clone().count() {
            if is_simply_safe(&mut splits.clone().take(i).chain(splits.clone().skip(i + 1))) {
                safe_count += 1;
                continue 'line;
            }
        }
    }
    safe_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_02_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        assert_eq!(part_1(input), 2);
    }

    #[test]
    fn day_02_part_2_test() {
        let input = include_str!("../rsrc/test.txt");
        assert_eq!(part_2(input), 4);
    }
}
