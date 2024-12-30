use core::fmt;
use std::ops::{Index, IndexMut, Neg};
use std::str::FromStr;
use std::slice::Iter;
use std::fmt::{Formatter, Display};
use crate::grid::Direction::*;
use crate::grid::RelativeDirection::*;

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
pub enum RelativeDirection {
    Right,
    Left,
    Forwards,
    Backwards,
}

impl Direction {
    pub fn iterator() -> Iter<'static, Direction> {
        static DIRECTIONS: [Direction; 8] = [North, South, East, West, Northeast, Northwest, Southeast, Southwest];
        DIRECTIONS.iter()
    }

    pub fn turn(&self, dir: &RelativeDirection) -> Direction {
        match dir {
            Right => match self {
                        North => East,
                        East => South,
                        South => West,
                        West => North,
                        Northeast => Southeast,
                        Southeast => Southwest,
                        Southwest => Northwest,
                        Northwest => Northeast,
                },
            Left => match self {
                        North => West,
                        West => South,
                        South => East,
                        East => North,
                        Northeast => Northwest,
                        Northwest => Southwest,
                        Southwest => Southeast,
                        Southeast => Northeast,
                },
            Forwards => self.clone(),
            Backwards => -self.clone()
        }
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

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
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

    pub fn find(&self, c: &char) -> Option<Position> {
        let sz = self.len();
        for y in 0..sz.y {
            for x in 0..sz.x {
                if self[Position {x, y}] == *c {
                    return Some(Position {x, y})
                }
            }
        }
        None
    }
}

#[derive(Debug)]
pub struct ParseGridError;

impl FromStr for Grid {
    type Err = ParseGridError;
    
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut grid = Vec::new();
        for l in s.split('\n') {
            if l.is_empty() {
                break;
            }
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

impl IndexMut<Position> for Grid {
    fn index_mut(&mut self, pos: Position) -> &mut Self::Output {
        &mut self.grid[pos.y][pos.x]
    }
}

impl IndexMut<&Position> for Grid {
    fn index_mut(&mut self, pos: &Position) -> &mut Self::Output {
        &mut self.grid[pos.y][pos.x]
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut Formatter<'_>) -> Result<(), fmt::Error> {
        for line in self.grid.iter() {
            writeln!(f, "{}", line.iter().cloned().collect::<String>())?;
        }
        Ok(())
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