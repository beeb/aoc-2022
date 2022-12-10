use itertools::Itertools;
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
                // we need to proceed to a new instruction
                match input.pop() {
                    Some(Instruction::Noop) => {
                        continue; // increment cycle counter without changing value
                    }
                    Some(Instruction::Addx(val)) => {
                        add_val = *val; // next cycle we're still processing addx, at the end we'll update the register
                        continue; // increment cycle counter
                    }
                    None => {
                        // vec is empty
                        break;
                    }
                }
            } else {
                // at the end of the cycle, we can change the register
                x += add_val;
                add_val = 0; // next cycle we pop a new instruction
            }
        }
        signal_sum
    }

    type Output2 = String;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut input = input.iter().rev().collect::<Vec<_>>();
        let mut crt: Vec<bool> = vec![false; 40 * 6]; // 6 rows of 40 pixels
        let mut add_val = 0;
        let mut x = 1isize;
        for cycle in 1usize.. {
            // during cycle, we draw the crt
            let crt_pos = (cycle as isize - 1) % 40;
            if crt_pos >= x - 1 && crt_pos <= x + 1 {
                crt[cycle - 1] = true;
            }
            if add_val == 0 {
                // we need to proceed to a new instruction
                match input.pop() {
                    Some(Instruction::Noop) => {
                        continue; // increment cycle counter without changing value
                    }
                    Some(Instruction::Addx(val)) => {
                        add_val = *val; // next cycle we're still processing addx, at the end we'll update the register
                        continue; // increment cycle counter
                    }
                    None => {
                        // vec is empty
                        break;
                    }
                }
            } else {
                // at the end of the cycle, we can change the register
                x += add_val;
                add_val = 0; // next cycle we pop a new instruction
            }
        }
        let mut res = crt
            .chunks_exact(40)
            .map(|row| {
                row.iter()
                    .map(|on| if *on { '#' } else { '.' })
                    .collect::<String>()
            })
            .join("\n");
        res.insert(0, '\n');
        res
    }
}
