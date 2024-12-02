fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
}

enum Direction {
    Decreasing,
    Increasing,
    Unset
}

fn part_1(input: &str) -> usize {
    let mut safe_count = 0;
    'line: for line in input.lines() {
        let mut direction = Direction::Unset;
        let mut splits = line.split(' ');
        let mut current = splits.next()
            .expect("split doesn't have a first element").parse::<i64>()
            .expect("couldn't parse value");
        for split in splits {
            let next = split.parse::<i64>().expect("couldn't parse value");
            match current - next {
                ..-3 => continue 'line,
                -3..0 => match direction {
                            Direction::Unset => direction = Direction::Decreasing,
                            Direction::Increasing => continue 'line,
                            Direction::Decreasing => (),
                        },
                0 => continue 'line,
                1..4 => match direction {
                            Direction::Unset => direction = Direction::Increasing,
                            Direction::Increasing => (),
                            Direction::Decreasing => continue 'line,
                        },
                4.. => continue 'line,
            }
            current = next;
        }
        safe_count += 1;
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
}
