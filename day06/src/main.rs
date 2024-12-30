use aoc_utils::grid::{Grid, traverse_grid, RelativeDirection::Right, Direction::North};

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
}

fn part_1(input: &str) -> usize {
    let mut grid = input.parse::<Grid>().expect("Failed to parse to grid");
    draw_path(&mut grid);
    grid.grid.into_iter().flatten().filter(|x| *x == 'X').count()
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

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn day_06_part_1_test() {
        let input = include_str!("../rsrc/test.txt");
        let result = part_1(input);
        assert_eq!(result, 41);
    }
}