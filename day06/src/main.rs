use aoc_utils::grid::*;
use aoc_utils::grid::{RelativeDirection::Right, Direction::North};
use std::collections::HashSet;

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
    let result = part_2(input);
    println!("part 2: {result}");
}

fn part_1(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Failed to parse to grid");
    draw_path(&mut grid);
    grid.grid.into_iter().flatten().filter(|x| *x == 'X').count()
}

fn part_2(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Failed to parse to grid");
    let mut loop_spots = 0;
    let candidates = enumerate_path(&grid);
    for candidate in candidates.into_iter().collect::<HashSet<Position>>() {
        if grid[candidate] == '^' {
            continue;
        }
        grid[candidate] = '#';
        if path_loops(&grid) {
            loop_spots += 1;
        }
        grid[candidate] = '.';
    }
    loop_spots
}

fn draw_path(grid: &mut Grid) {
    if let Some(mut pos) = grid.find(&'^') {
        let mut dir = North;
        grid[&pos] = 'X';
        while let Ok((next_pos, c)) = traverse_grid(grid, &pos, &dir) {
            if c == '#' {
                dir = dir.turn(&Right);
            } else {
                grid[&next_pos] = 'X';
                pos = next_pos;
            }
        }
        grid[&pos] = 'X'
    }
}

fn enumerate_path(grid: &Grid) -> Vec<Position> {
    let mut path = Vec::new();
    if let Some(mut pos) = grid.find(&'^') {
        let mut dir = North;
        while let Ok((next_pos, c)) = traverse_grid(grid, &pos, &dir) {
            if c == '#' {
                dir = dir.turn(&Right);
            } else {
                path.push(next_pos.clone());
                pos = next_pos;
            }
        }
    }
    path
}

fn path_loops(grid: &Grid) -> bool {
    if let Some(mut pos) = grid.find(&'^') {
        let mut dir = North;
        let mut states = HashSet::new();
        states.insert((pos, dir));
        while let Ok((next_pos, c)) = traverse_grid(grid, &pos, &dir) {
            if c == '#' {
                dir = dir.turn(&Right);
            } else {
                pos = next_pos;
            }
            if states.contains(&(pos, dir)) {
                return true;
            }
            states.insert((pos, dir));
        }
    }
    false
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_06_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_1(input);
        assert_eq!(result, 41);
    }

    #[test]
    fn day_06_part_2_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_2(input);
        assert_eq!(result, 6);
    }
}