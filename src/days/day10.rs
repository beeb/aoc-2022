use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{i64, line_ending, space1},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

pub struct Day10;

#[derive(Debug)]
pub enum Instruction {
    Noop,
    Addx(isize),
}

impl Day for Day10 {
    type Input = Vec<Instruction>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            alt((
                map(tag("noop"), |_| Instruction::Noop),
                map(tuple((tag("addx"), space1, i64)), |(_, _, i)| {
                    Instruction::Addx(i as isize)
                }),
            )),
        )(input)
    }

    type Output1 = isize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut input = input.iter().rev().collect::<Vec<_>>();
        let measure_at = vec![20, 60, 100, 140, 180, 220];
        let mut add_val = 0;
        let mut x = 1;
        let mut signal_sum = 0;
        for cycle in 1..=*measure_at.last().unwrap() {
            // during cycle, we check counter
            if measure_at.contains(&cycle) {
                signal_sum += cycle * x;
            }
            if add_val == 0 {
                match input.pop() {
                    Some(Instruction::Noop) => {
                        continue; // increment cycle counter without changing value
                    }
                    Some(Instruction::Addx(val)) => {
                        add_val = *val; // next cycle we're still processing addx
                        continue; // increment cycle counter
                    }
                    None => {
                        break;
                    }
                }
            } else {
                // at the end of the cycle, we can increment value
                x += add_val;
                add_val = 0;
            }
        }
        signal_sum
    }

    type Output2 = isize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
