use std::ops::Neg;
use std::ops::Index;
use std::str::FromStr;
use std::slice::Iter;
use self::Direction::*;

fn main() {
    let input = include_str!("../rsrc/input.txt");
    let result = part_1(input);
    println!("part 1: {result}");
    let result = part_2(input);
    println!("part 2: {result}");
}

#[derive(Clone)]
enum Direction {
    North,
    South,
    East,
    West,
    Northeast,
    Northwest,
    Southeast,
    Southwest,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [North, South, East, West, Northeast, Northwest, Southeast, Southwest];
        DIRECTIONS.iter()
    }
}

impl Neg for Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            North => South,
            South => North,
            East => West,
            West => East,
            Northeast => Southwest,
            Northwest => Southeast,
            Southeast => Northwest,
            Southwest => Northeast,
        }
    }
}

impl Neg for &Direction {
    type Output = Self;
    fn neg(self) -> Self::Output {
        match self {
            North => &South,
            South => &North,
            East => &West,
            West => &East,
            Northeast => &Southwest,
            Northwest => &Southeast,
            Southeast => &Northwest,
            Southwest => &Northeast,
        }
    }
}

#[derive(Clone)]
struct Position {
    x: usize,
    y: usize,
}

struct Grid {
    grid: Vec<Vec<char>>,
}

struct GridSize {
    x: usize,
    y: usize,
}

impl Grid {
    fn len(&self) -> GridSize {
        if self.grid.len() == 0 {
            GridSize{x: 0, y: 0}
        } else {
            GridSize{x: self.grid[0].len(), y: self.grid.len()}
        }
        
    }
}

#[derive(Debug)]
struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for l in s.split('\n') {
            grid.push(l.chars().collect());
        }
        Ok(Grid{ grid })
    }
}

impl Index<Position> for Grid {
    type Output = char;
    
    fn index(&self, pos: Position) -> &Self::Output {
        &self.grid[pos.y][pos.x]
    }
}

impl Index<&Position> for Grid {
    type Output = char;
    
    fn index(&self, pos: &Position) -> &Self::Output {
        &self.grid[pos.y][pos.x]
    }
}

struct TraversalError {
    pos: Position,
    dir: Direction,
}

fn traverse_grid(grid: &Grid, pos: &Position, dir: &Direction) -> Result<(Position, char), TraversalError> {
    let p = match dir {
        North => if pos.y == 0 {Err(TraversalError{pos: pos.clone(), dir: dir.clone()})?} else {Position{y: pos.y - 1, x: pos.x}},
        South => Position{y: pos.y + 1, x: pos.x},
        East => Position{x: pos.x + 1, y: pos.y},
        West => if pos.x == 0 {Err(TraversalError{pos: pos.clone(), dir: dir.clone()})?} else {Position{x: pos.x - 1, y: pos.y}},
        Northeast => if pos.y == 0 {Err(TraversalError{pos: pos.clone(), dir: dir.clone()})?} else {Position{x: pos.x + 1, y: pos.y - 1}},
        Southeast => Position{x: pos.x + 1, y: pos.y + 1},
        Northwest => if pos.x == 0 || pos.y == 0 {Err(TraversalError{pos: pos.clone(), dir: dir.clone()})?} else {Position{x: pos.x - 1, y: pos.y - 1}},
        Southwest => if pos.x == 0 {Err(TraversalError{pos: pos.clone(), dir: dir.clone()})?} else {Position{x: pos.x - 1, y: pos.y + 1}},
    };
    let g = grid.len();
    if (0..g.x).contains(&p.x) && (0..g.y).contains(&p.y) {
        Ok((p.clone(), grid[p]))
    } else {
        Err(TraversalError{pos: pos.clone(), dir: dir.clone()})
    }
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