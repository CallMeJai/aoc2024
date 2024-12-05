use regex::Regex;

fn main() {
    let mul = Regex::new(r"(?-i)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let input = include_str!("../rsrc/input.txt");
    let mut total: usize = 0;
    for (_, [left, right]) in mul.captures_iter(input).map(|c| c.extract()) {
        total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
    }
    println!("part 1: {total}");
    let input = disable_instructions(&input);
    total = 0;
    for (_, [left, right]) in mul.captures_iter(&input).map(|c| c.extract()) {
        total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
    }
    println!("part 2: {total}");
}

fn disable_instructions(input: &str) -> String {
    let dont_do = Regex::new(r"(?s)don't\(\).*?(do\(\))|$").unwrap();
    dont_do.replace_all(input, "").to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_03_part_1_test() {
        let mul = Regex::new(r"(?-i)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let input = include_str!("../rsrc/test.txt");
        let mut total: usize = 0;
        for (_, [left, right]) in mul.captures_iter(input).map(|c| c.extract()) {
            total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
        }
        assert_eq!(total, 161);
    }

    #[test]
    fn day_03_part_2_test() {
        let mul = Regex::new(r"(?-i)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let input = include_str!("../rsrc/test_part_2.txt");
        let input = disable_instructions(&input);
        let mut total: usize = 0;
        for (_, [left, right]) in mul.captures_iter(&input).map(|c| c.extract()) {
            total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
        }
        assert_eq!(total, 48);
    }
}
