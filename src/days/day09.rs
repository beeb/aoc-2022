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
        println!("{input:?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
