use nom::{
    character::complete::{char, line_ending, u8},
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day04;

fn parse_range(input: &str) -> IResult<&str, Range> {
    let (rest, (start, _, end)) = tuple((u8, char('-'), u8))(input)?;
    Ok((rest, Range { start, end }))
}

fn parse_pair(input: &str) -> IResult<&str, Pair> {
    let (rest, (first, _, second)) = tuple((parse_range, char(','), parse_range))(input)?;
    Ok((rest, Pair { first, second }))
}

#[derive(Debug)]
pub struct Range {
    pub start: u8,
    pub end: u8,
}

#[derive(Debug)]
pub struct Pair {
    pub first: Range,
    pub second: Range,
}

impl Pair {
    pub fn fully_contained(&self) -> bool {
        (self.first.start <= self.second.start && self.first.end >= self.second.end)
            || (self.first.start >= self.second.start && self.first.end <= self.second.end)
    }

    pub fn overlaps(&self) -> bool {
        self.first.end >= self.second.start && self.first.start <= self.second.end
    }
}

impl Day for Day04 {
    type Input = Vec<Pair>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_pair)(input)
    }

    type Output1 = usize;

    /// Part 1 took 0.015ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .fold(0, |acc, p| acc + p.fully_contained() as usize)
    }

    type Output2 = usize;

    /// Part 2 took 0.0015ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        input.iter().fold(0, |acc, p| acc + p.overlaps() as usize)
    }
}
