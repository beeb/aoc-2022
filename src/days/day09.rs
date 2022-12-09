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
    fn dist(&self) -> isize {
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

/// we will use interior mutability to avoid having to borrow mutably
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

    /// Move this point by a signed increment in x and y
    fn mov(&self, x: isize, y: isize) -> &Self {
        *self.x.borrow_mut() += x;
        *self.y.borrow_mut() += y;
        self
    }

    /// Apply a move increment (distance = 1)
    fn apply_move(&self, mov: &Move) -> &Self {
        match mov {
            Move::Up(_) => {
                self.mov(0, 1);
            }
            Move::Right(_) => {
                self.mov(1, 0);
            }
            Move::Down(_) => {
                self.mov(0, -1);
            }
            Move::Left(_) => {
                self.mov(-1, 0);
            }
        }
        self
    }

    /// Move this point towards another point
    fn move_towards(&self, other: &Point) -> &Self {
        if other.x() > self.x() {
            *self.x.borrow_mut() += 1;
        }
        if other.x() < self.x() {
            *self.x.borrow_mut() -= 1;
        }
        if other.y() > self.y() {
            *self.y.borrow_mut() += 1;
        }
        if other.y() < self.y() {
            *self.y.borrow_mut() -= 1;
        }
        self
    }

    /// Compte the distance between two points
    ///
    /// The actual distance is computed as follows:
    ///
    /// ```
    /// fn dist_real(&self, other: &Point) -> isize {
    ///     let dx = self.x() - other.x();
    ///     let dy = self.y() - other.y();
    ///     let m = (dx.pow(2) + dy.pow(2)) as f64;
    ///     m.sqrt().round() as isize
    /// }
    /// ```
    ///
    /// But we can simplify it in our case, since we can't move more than 1
    /// position at a time
    ///
    fn dist(&self, other: &Point) -> isize {
        let dx = self.x() - other.x();
        let dy = self.y() - other.y();
        /* if dx == 0 && dy == 0 {
            // this is not even needed as we just want to know if the distance is larger than 1
            return 0;
        } */
        if dx.abs() > 1 || dy.abs() > 1 {
            return 2;
        }
        1
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

    /// Part 1 took 0.4943ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        // keep track of all the visited coordinates
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let head = Point::default();
        let tail = Point::default();
        for mov in input {
            for _ in 0..mov.dist() {
                // for each movement step
                head.apply_move(mov); // only moves by 1 unit
                if head.dist(&tail) > 1 {
                    // the head moved too far, we need to move the tail too
                    tail.move_towards(&head);
                }
                visited.insert((tail.x(), tail.y()));
            }
        }
        visited.len()
    }

    type Output2 = usize;

    /// Part 2 took 0.6268ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // keep track of all the visited coordinates
        let mut visited: HashSet<(isize, isize)> = HashSet::new();
        let knots = vec![Point::default(); 10];
        for mov in input {
            for _ in 0..mov.dist() {
                // for each movement step
                knots[0].apply_move(mov); // only moves by 1 unit
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
