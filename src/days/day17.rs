use itertools::Itertools;
use nom::{character::complete::anychar, combinator::map, multi::many1, IResult};

use crate::days::Day;

#[derive(Debug)]
pub enum Push {
    Left,
    Right,
}

pub struct Piece {
    data: [u8; 4],
    z: usize,
}

impl Piece {
    fn new(kind: u8, z: usize) -> Self {
        let data = match kind {
            0 => [0b0, 0b0, 0b0, 0b11110],
            1 => [0b0, 0b1000, 0b11100, 0b1000],
            2 => [0b0, 0b100, 0b100, 0b11100],
            3 => [0b10000; 4],
            4 => [0b0, 0b0, 0b11000, 0b11000],
            _ => unimplemented!("only 5 pieces types are available"),
        };
        Self { data, z }
    }
}

pub struct Day17;

impl Day for Day17 {
    type Input = Vec<Push>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        many1(map(anychar, |c| match c {
            '<' => Push::Left,
            _ => Push::Right,
        }))(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let push = input.iter().cycle();

        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
