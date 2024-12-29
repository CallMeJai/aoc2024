use std::ops::Neg;
use std::ops::Index;
use std::str::FromStr;
use std::slice::Iter;
use crate::grid::Direction::*;

#[derive(Clone)]
pub enum Direction {
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
pub struct Position {
    pub x: usize,
    pub y: usize,
}

pub struct Grid {
    pub grid: Vec<Vec<char>>,
}

pub struct GridSize {
    pub x: usize,
    pub y: usize,
}

impl Grid {
    pub fn len(&self) -> GridSize {
        if self.grid.len() == 0 {
            GridSize{x: 0, y: 0}
        } else {
            GridSize{x: self.grid[0].len(), y: self.grid.len()}
        }
        
    }
}

#[derive(Debug)]
pub struct ParseGridError;

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

pub struct TraversalError {
    pub pos: Position,
    pub dir: Direction,
}

pub fn traverse_grid(grid: &Grid, pos: &Position, dir: &Direction) -> Result<(Position, char), TraversalError> {
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