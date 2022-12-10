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

/// Process the instructions during a cycle.
///
/// Returns the updated `x` and the updated `add_val`, and a boolean that is false when there are no more instructions
fn process_cycle(
    x: isize,
    add_val: isize,
    instructions: &mut Vec<&Instruction>,
) -> (isize, isize, bool) {
    if add_val == 0 {
        // we need to proceed to a new instruction
        match instructions.pop() {
            Some(Instruction::Noop) => {
                return (x, 0, true); // x unchanged, pop the next instruction
            }
            Some(Instruction::Addx(val)) => {
                return (x, *val, true); // next cycle we're still processing addx, at the end we'll update the register
            }
            None => {
                // vec is empty
                return (x, 0, false);
            }
        }
    }
    // increment the register
    // next cycle we pop a new instruction (add_val = 0)
    (x + add_val, 0, true)
}

fn sprite_in_range(cycle: usize, x: isize) -> bool {
    let crt_row_pos = (cycle as isize - 1) % 40;
    crt_row_pos >= x - 1 && crt_row_pos <= x + 1
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

    /// Part 1 took 0.002ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut input = input.iter().rev().collect_vec(); // reverse so we can pop
        let mut x = 1; // the register
        let mut signal_sum = 0; // our output
        let mut add_val = 0; // temp var to hold the addx value over to the next cycle
        for cycle in 1.. {
            // during cycle, we check counter to see if we need to measure the signal
            if cycle % 40 == 20 {
                signal_sum += cycle * x;
            }
            let cont: bool;
            (x, add_val, cont) = process_cycle(x, add_val, &mut input);
            if !cont {
                break;
            }
        }
        signal_sum
    }

    type Output2 = String;

    /// Part 2 took 0.0061ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut input = input.iter().rev().collect_vec(); // reverse so we can pop
        let mut crt: Vec<char> = vec!['.'; 40 * 6]; // 6 rows of 40 pixels
        let mut x = 1isize; // the register
        let mut add_val = 0; // temp var
        for cycle in 1usize.. {
            // during cycle, we draw the crt
            // check if the sprite is in range
            if sprite_in_range(cycle, x) {
                // we turn the pixel on
                crt[cycle - 1] = '#';
            }
            let cont: bool;
            (x, add_val, cont) = process_cycle(x, add_val, &mut input);
            if !cont {
                break;
            }
        }
        let mut res = crt
            .chunks_exact(40)
            .map(|row| row.iter().collect::<String>())
            .join("\n");
        res.insert(0, '\n');
        res
    }
}
