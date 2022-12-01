use nom::{
    character::complete::{line_ending, u16},
    combinator::map,
    multi::{count, separated_list0},
    IResult,
};

use crate::days::Day;

pub struct Day01;

impl Day for Day01 {
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            count(line_ending, 2),
            separated_list0(line_ending, map(u16, |c| c as usize)),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input.iter().map(|e| e.iter().sum()).max().unwrap_or(0)
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        0
    }
}
