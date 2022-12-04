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

    /// Part 1 took 0.0173ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        sums(input).max().unwrap_or(0)
    }

    type Output2 = usize;

    /// Documenting a very nice solution provided by Jayjader at
    /// https://github.com/Jayjader/rust-advent-of-code-2022/blob/main/src/main.rs
    /// which is twice as fast, runs in 0.0042ms on my machine.
    ///
    /// ```
    /// let mut max = [0, 0, 0];
    /// for calories in sums(input) {
    ///     if calories > max[0] {
    ///         // sorting guarantees that if calories is bigger than max[0] then it is among the top 3.
    ///         // conversely, sorting guarantees that max[0] is the smallest and thus should always be dropped
    ///         // when a new max is found.
    ///         max[0] = calories;
    ///         max.sort();
    ///     }
    /// }
    /// max.iter().sum()
    /// ```
    ///
    /// This is my "naive" solution
    /// Part 2 took 0.0103ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        sums(input).sorted().rev().take(3).sum()
    }
}

fn sums(input: &<Day01 as Day>::Input) -> impl Iterator<Item = usize> + '_ {
    input.iter().map(|e| e.iter().sum())
}
