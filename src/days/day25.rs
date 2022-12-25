use itertools::Itertools;
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

fn snafu_to_decimal(input: &[i8]) -> i64 {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(i, &d)| 5i64.pow(i as u32) * d as i64)
        .sum()
}

fn decimal_to_snafu(input: u64) -> Vec<i8> {
    // 2022         1=11-2
    // 2022 % 1 = 0
    // 2022 / 1 = 2022
    // 2022 % 5 = 2
    // 2022 / 5 = 404
    // 2022 % 25 = 22
    // 2022 / 25 = 80
    // 2022 % 125 = 22
    // 2022 / 125 = 16
    // 2022 % 625 = 147
    // 2022 / 625 = 3
    // 2022 % 3125 = 2022
    // 2022 / 3125 = 0

    // ======== pos 0
    // 2022 % 5^(pos+1) = 2
    // 2 / 5^pos = 2
    // 2 <= 2 :: +2
    // 000002
    // ======== pos 1
    // 2020 % 25 = 20
    // 20 / 5 = 4
    // 4 > 2 :: 4 - 5 = -1
    // 4 > 2 :: next digit +1
    // 0001-2
    // ======== pos 2
    // 2000 % 125 = 0
    // 0 / 25 = 0
    // 0 <= 2 :: +0
    // 0001-2
    // ======== pos 3
    // 2000 % 625 = 125
    // 125 / 125 = 1
    // 1 < 2 :: 1
    // 0011-2
    // ======== pos 4
    // 1875 % 3125 = 1875
    // 1875 / 625 = 3
    // 3 > 2 :: 5 - 3 = -2
    // 3 > 2 :: next digit +1
    // 1=11-2
    let mut out = vec![0i8; 20]; // reversed (right to left)
    let mut remaining = input;
    for pos in 0.. {
        let rem = remaining % 5u64.pow(pos + 1);
        let div = rem / 5u64.pow(pos);
        out[pos as usize] += div as i8;
        if out[pos as usize] > 2 {
            out[pos as usize] -= 5;
            out[pos as usize + 1] += 1;
        }
        remaining -= rem;
        if remaining == 0 {
            break;
        }
    }
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

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let sum = input.iter().map(|s| snafu_to_decimal(s)).sum::<i64>();
        let snafu = decimal_to_snafu(sum as u64);
        String::from_iter(snafu.into_iter().map(|d| match d {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            _ => unreachable!(),
        }))
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        0
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
