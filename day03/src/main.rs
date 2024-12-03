use regex::Regex;

fn main() {
    let mul_p1 = Regex::new(r"(?-i)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
    let input = include_str!("../rsrc/input.txt");
    let mut total: usize = 0;
    for (_, [left, right]) in mul_p1.captures_iter(input).map(|c| c.extract()) {
        total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
    }
    println!("part 1: {total}");
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_03_part_1_test() {
        let mul_p1 = Regex::new(r"(?-i)mul\(([0-9]{1,3}),([0-9]{1,3})\)").unwrap();
        let input = include_str!("../rsrc/test.txt");
        let mut total: usize = 0;
        for (_, [left, right]) in mul_p1.captures_iter(input).map(|c| c.extract()) {
            total += left.parse::<usize>().unwrap() * right.parse::<usize>().unwrap();
        }
        assert_eq!(total, 161);
    }
}
