use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day03;

fn parse_line(input: &str) -> IResult<&str, (Vec<u8>, Vec<u8>)> {
    let (rest, s) = alpha1(input)?;
    let chars: Vec<u8> = s
        .chars()
        .map(|c| {
            if c.is_uppercase() {
                return (c as u8) - 38;
            }
            (c as u8) - 96
        })
        .collect();
    let mid = chars.len() / 2;
    Ok((rest, (chars[..mid].to_vec(), chars[mid..].to_vec())))
}

impl Day for Day03 {
    type Input = Vec<(Vec<u8>, Vec<u8>)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, parse_line)(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .flat_map(|rs| rs.0.iter().filter(|e| rs.1.contains(e)).dedup())
            .map(|e| *e as usize)
            .sum::<usize>()
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
