use aoc_utils::grid::*;
use std::str::FromStr;

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
    let result = part_2(input);
    println!("part 2: {result}");
}

fn check_xmas(grid: &Grid, pos: Position) -> usize {
    let mut num_found = 0;
    if grid[&pos] == 'X' {
        for dir in Direction::iterator() {
            if let Ok((pos, c)) = traverse_grid(grid, &pos, dir) {
                if c == 'M' {
                    if let Ok((pos, c)) = traverse_grid(grid, &pos, dir) {
                        if c == 'A' {
                            if let Ok((_, c)) = traverse_grid(grid, &pos, dir) {
                                if c == 'S' {
                                    num_found += 1
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    num_found
}

fn check_mas_cross(grid: &Grid, pos: Position) -> usize {
    let mut num_found = 0;
    let mut first_mas = true;
    if grid[&pos] == 'A' {
        for dir in vec![Direction::Northeast, Direction::Southeast, Direction::Northwest, Direction::Southwest].into_iter() {
            if let Ok((_, c)) = traverse_grid(grid, &pos, &dir) {
                if c == 'M' {
                    if let Ok((_, c)) = traverse_grid(grid, &pos, &-dir) {
                        if c == 'S' {
                            if first_mas {
                                first_mas = false;
                            } else {
                                num_found += 1;
                                first_mas = true;
                            } 
                        }
                    }
                }
            }
        }
        first_mas = true;
    }
    num_found
}

fn part_1(input: &str) -> usize {
    let mut xmas_count = 0;
    let grid = Grid::from_str(&input).expect("Cannot parse to grid");
    for y in 0..grid.len().y {
        for x in 0..grid.len().x {
            xmas_count += check_xmas(&grid, Position{x, y});
        }
    }
    xmas_count
}

fn part_2(input: &str) -> usize {
    let mut mas_cross_count = 0;
    let grid = Grid::from_str(&input).expect("Cannot parse to grid");
    for y in 0..grid.len().y {
        for x in 0..grid.len().x {
            mas_cross_count += check_mas_cross(&grid, Position{x, y});
        }
    }
    mas_cross_count
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_04_part_1_simple_test() {
        let input = include_str!("../rsrc/simple_test.txt");
        let result = part_1(input);
        assert_eq!(result, 4);
    }

    #[test]
    fn day_04_part_1_filtered_test() {
        let input = include_str!("../rsrc/filtered_test.txt");
        let result = part_1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn day_04_part_1_full_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_1(input);
        assert_eq!(result, 18);
    }

    #[test]
    fn day_04_part_2_simple_test() {
        let input = include_str!("../rsrc/simple_test_part_2.txt");
        let result = part_2(input);
        assert_eq!(result, 1);
    }

    #[test]
    fn day_04_part_2_filtered_test() {
        let input = include_str!("../rsrc/filtered_test_part_2.txt");
        let result = part_2(input);
        assert_eq!(result, 9);
    }

    #[test]
    fn day_04_part_2_full_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_2(input);
        assert_eq!(result, 9);
    }
}