use std::{cell::RefCell, collections::HashSet};

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::pair,
    IResult,
};

use crate::days::Day;

pub struct Day09;

#[derive(Debug)]
pub enum Move {
    Up(isize),
    Right(isize),
    Down(isize),
    Left(isize),
}

impl Move {
    fn inner(&self) -> isize {
        match self {
            Self::Up(dist) => *dist,
            Self::Right(dist) => *dist,
            Self::Down(dist) => *dist,
            Self::Left(dist) => *dist,
        }
    }
}

impl From<(&str, i64)> for Move {
    fn from(p: (&str, i64)) -> Self {
        match p.0 {
            "U " => Self::Up(p.1 as isize),
            "R " => Self::Right(p.1 as isize),
            "D " => Self::Down(p.1 as isize),
            "L " => Self::Left(p.1 as isize),
            _ => unimplemented!("should not happen"),
        }
    }
}

#[derive(Clone)]
struct Point {
    x: RefCell<isize>,
    y: RefCell<isize>,
}

impl Default for Point {
    fn default() -> Self {
        Self {
            x: RefCell::new(0),
            y: RefCell::new(0),
        }
    }
}

/// x points to the right
/// y points to the top
impl Point {
    fn x(&self) -> isize {
        *self.x.borrow()
    }
    fn y(&self) -> isize {
        *self.y.borrow()
    }

    fn mov(&self, x: isize, y: isize) -> &Self {
        *self.x.borrow_mut() += x;
        *self.y.borrow_mut() += y;
        self
    }
    fn move_towards(&self, other: &Point) -> &Self {
        if self.x == other.x {
            // above or below or on top
            *self.y.borrow_mut() += (other.y > self.y) as isize;
            *self.y.borrow_mut() -= (other.y < self.y) as isize;
        } else if self.y == other.y {
            // left or right
            *self.x.borrow_mut() += (other.x > self.x) as isize;
            *self.x.borrow_mut() -= (other.x < self.x) as isize;
        } else if other.x > self.x && other.y > self.y {
            // north-east quadrant
            *self.x.borrow_mut() += 1;
            *self.y.borrow_mut() += 1;
        } else if other.x > self.x && other.y < self.y {
            // south-east quadrant
            *self.x.borrow_mut() += 1;
            *self.y.borrow_mut() -= 1;
        } else if other.x < self.x && other.y < self.y {
            // south-west quadrant
            *self.x.borrow_mut() -= 1;
            *self.y.borrow_mut() -= 1;
        } else {
            // north-west quadrant
            *self.x.borrow_mut() -= 1;
            *self.y.borrow_mut() += 1;
        }
        self
    }
    fn dist(&self, other: &Point) -> isize {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        let m = (dx.pow(2) + dy.pow(2)) as f64;
        m.sqrt().round() as isize
    }
}

impl Day for Day09 {
    type Input = Vec<Move>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            alt((
                map(pair(tag("U "), i64), Move::from),
                map(pair(tag("R "), i64), Move::from),
                map(pair(tag("D "), i64), Move::from),
                map(pair(tag("L "), i64), Move::from),
            )),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let head = Point::default();
        let tail = Point::default();
        for mov in input {
            for _ in 0..mov.inner() {
                match mov {
                    Move::Up(_) => {
                        head.mov(0, 1);
                    }
                    Move::Right(_) => {
                        head.mov(1, 0);
                    }
                    Move::Down(_) => {
                        head.mov(0, -1);
                    }
                    Move::Left(_) => {
                        head.mov(-1, 0);
                    }
                }
                if head.dist(&tail) > 1 {
                    tail.move_towards(&head);
                }
                visited.insert((tail.x(), tail.y()));
            }
        }
        visited.len()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let knots = vec![Point::default(); 10];
        for mov in input {
            for _ in 0..mov.inner() {
                match mov {
                    Move::Up(_) => {
                        knots[0].mov(0, 1);
                    }
                    Move::Right(_) => {
                        knots[0].mov(1, 0);
                    }
                    Move::Down(_) => {
                        knots[0].mov(0, -1);
                    }
                    Move::Left(_) => {
                        knots[0].mov(-1, 0);
                    }
                }
                for k in 1..10 {
                    if knots[k - 1].dist(&knots[k]) > 1 {
                        knots[k].move_towards(&knots[k - 1]);
                    }
                }
                visited.insert((knots[9].x(), knots[9].y()));
            }
        }
        visited.len()
    }
}
