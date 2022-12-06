use itertools::Itertools;
use nom::IResult;

use crate::days::Day;

pub struct Day06;

impl Day for Day06 {
    type Input = String;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        Ok(("", String::from(input)))
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        for (i, (a, b, c, d)) in input.chars().tuple_windows::<(_, _, _, _)>().enumerate() {
            if [a, b, c, d].iter().all_unique() {
                return i + 4;
            }
        }
        0
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        for (i, seq) in input.as_bytes().windows(14).enumerate() {
            if seq.iter().all_unique() {
                return i + 14;
            }
        }
        0
    }
}
