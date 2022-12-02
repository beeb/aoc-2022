use nom::{
    branch::alt,
    character::complete::{char, line_ending, space1},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day02;

fn parse_char(input: char) -> isize {
    match input {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => unreachable!(),
    }
}

fn parse_chars(input: &[(char, char)]) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.iter().map(|(a, x)| (parse_char(*a), parse_char(*x)))
}

impl Day for Day02 {
    type Input = Vec<(char, char)>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(
                tuple((
                    alt((char('A'), char('B'), char('C'))),
                    space1,
                    alt((char('X'), char('Y'), char('Z'))),
                )),
                |(a, _, x)| (a, x),
            ),
        )(input)
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut score = 0;
        for (elf, me) in parse_chars(input) {
            score += match me - elf {
                2 | -1 => 0,
                -2 | 1 => 6,
                0 => 3,
                _ => unreachable!(),
            };
            score += me;
        }
        score
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
