use itertools::Itertools;
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

/// Convert a snafu into decimal
///
/// We simply need to sum each position multiplied by 5 to the power of the position (starting at 0)
fn snafu_to_decimal(input: &[i8]) -> i64 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| 5i64.pow(i as u32) * d as i64)
        .sum()
}

/// Convert a decimal number into snafu (i8 representation, so no '=' and '-' just yet)
fn decimal_to_snafu(input: u64) -> Vec<i8> {
    let mut out = vec![0i8; 20]; // reversed (right to left)
    let mut remaining = input;
    for pos in 0.. {
        // "rem" is what we need to encode at the current position since
        // it's smaller than the resolution of the next position.
        // we thus take the remainder of the division by the next position's factor 5^(pos+1)
        let rem = remaining % 5u64.pow(pos + 1);
        // we divide this remainder by the current factor to know what to put in this position as value
        let div = rem / 5u64.pow(pos);
        out[pos as usize] += div as i8;
        // since we can only encode -2 to 2, we need to carry over in case the value is above 2
        if out[pos as usize] > 2 {
            out[pos as usize] -= 5; // return into the range
            out[pos as usize + 1] += 1; // carry over to the next position
        }
        // what's left to encode is decremented by what we just encoded into this position
        remaining -= rem;
        // we can stop when we have encoded the full value
        if remaining == 0 {
            break;
        }
    }
    // we expect the value to have the most significant digit first, so we reverse and skip all zeroes
    out.into_iter().rev().skip_while(|&i| i == 0).collect_vec()
}

pub struct Day25;

impl Day for Day25 {
    type Input = Vec<Vec<i8>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(not_line_ending, |s: &str| {
                s.chars()
                    .map(|c| match c {
                        '-' => -1,
                        '=' => -2,
                        d => ((d as u8) - 48) as i8,
                    })
                    .collect_vec()
            }),
        )(input)
    }

    type Output1 = String;

    /// Part 1 took 0.016806ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let sum = input.iter().map(|s| snafu_to_decimal(s)).sum::<i64>();
        let snafu = decimal_to_snafu(sum as u64);
        // convert -2 to +2 into characters and collect into a string
        String::from_iter(snafu.into_iter().map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        }))
    }

    type Output2 = String;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        "Congratulations!".to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn to_decimal() {
        assert_eq!(snafu_to_decimal(&[1i8, -2, -1, 0, -1, 2]), 1747);
    }

    #[test]
    fn to_snafu() {
        assert_eq!(decimal_to_snafu(2022), vec![1, -2, 1, 1, -1, 2]);
        assert_eq!(decimal_to_snafu(12345), vec![1, -1, 0, -1, -1, -1, 0]);
    }
}
