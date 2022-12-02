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

fn parse_move(input: char) -> isize {
    match input {
        'A' | 'X' => 1,
        'B' | 'Y' => 2,
        'C' | 'Z' => 3,
        _ => unreachable!(),
    }
}

fn parse_outcome(input: char) -> isize {
    match input {
        'X' => 0,
        'Y' => 3,
        'Z' => 6,
        _ => unreachable!(),
    }
}

fn parse_chars1(input: &[(char, char)]) -> impl Iterator<Item = (isize, isize)> + '_ {
    input.iter().map(|(a, x)| (parse_move(*a), parse_move(*x)))
}

fn parse_chars2(input: &[(char, char)]) -> impl Iterator<Item = (isize, isize)> + '_ {
    input
        .iter()
        .map(|(a, x)| (parse_move(*a), parse_outcome(*x)))
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

    /// Part 1 took 0.033ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut score = 0;
        for (elf, me) in parse_chars1(input) {
            score += match me - elf {
                -2 | 1 => 6, // win
                0 => 3,      // draw if both values are the same
                _ => 0,      // loss
            } + me; // don't forget to add the score for my chosen move
        }
        score
    }

    type Output2 = isize;

    /// Part 2 took 0.0178ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        // encoding the move to play in order to win or lose
        // index + 1 = elf's move, value at that index = my move
        let win = [2, 3, 1];
        let lose = [3, 1, 2];
        let mut score = 0;
        // we parse the input with each entry being a tuple containing the elf's move and the outcome's score
        for (elf, outcome) in parse_chars2(input) {
            score += match outcome {
                // check which move to play
                0 => lose[(elf - 1) as usize],
                3 => elf, // for a draw, we have to play the same move as the elf
                6 => win[(elf - 1) as usize],
                _ => unreachable!(),
            } + outcome; // and add the outcome score
        }
        score
    }
}
