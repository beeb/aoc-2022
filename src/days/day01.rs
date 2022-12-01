use itertools::Itertools;
use nom::{
    character::complete::{line_ending, u32},
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
            separated_list0(line_ending, map(u32, |c| c as usize)),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        sums(input).into_iter().max().unwrap_or(0)
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        sums(input)
            .iter()
            .sorted()
            .rev()
            .take(3)
            .fold(0, |acc, s| acc + s)
    }
}

fn sums(input: &<Day01 as Day>::Input) -> Vec<usize> {
    input.iter().map(|e| e.iter().sum()).collect()
}
