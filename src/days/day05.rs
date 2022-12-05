use std::collections::VecDeque;

use itertools::enumerate;
use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, u32},
    combinator::map,
    multi::separated_list0,
    sequence::tuple,
    IResult,
};

use crate::days::Day;

const NUM_STACKS: usize = 9;

pub struct Day05;

#[derive(Debug, Clone)]
pub struct Move {
    pub amount: usize,
    pub from: usize,
    pub to: usize,
}

#[derive(Default, Debug, Clone)]
pub struct State {
    pub stacks: [VecDeque<char>; NUM_STACKS],
    pub moves: Vec<Move>,
}

impl State {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn clone(&self) -> Self {
        Self {
            stacks: self.stacks.clone(),
            moves: self.moves.clone(),
        }
    }

    pub fn push_crate_front(&mut self, stack: usize, id: char) -> &mut Self {
        self.stacks.get_mut(stack).unwrap().push_front(id);
        self
    }

    pub fn move_crates_9000(&mut self, amount: usize, from: usize, to: usize) -> &mut Self {
        for _ in 0..amount {
            let c = self.stacks[from - 1].pop_back().unwrap();
            self.stacks[to - 1].push_back(c);
        }
        self
    }

    pub fn move_crates_9001(&mut self, amount: usize, from: usize, to: usize) -> &mut Self {
        let from_len = self.stacks[from - 1].len();
        let mut mov = self.stacks[from - 1].split_off(from_len - amount);
        self.stacks[to - 1].append(&mut mov);
        self
    }
}

impl Day for Day05 {
    type Input = State;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        let mut state = State::new();
        let (rest, stacks) = separated_list0(
            line_ending,
            separated_list0(
                char(' '),
                alt((
                    tuple((char(' '), char(' '), char(' '))),
                    tuple((char('['), anychar, char(']'))),
                )),
            ),
        )(input)?;
        for layer in stacks {
            for (i, (_, c, _)) in enumerate(layer) {
                match c {
                    ' ' => {}
                    c => {
                        state.push_crate_front(i, c);
                    }
                }
            }
        }
        let (rest, _) = tag(" 1   2   3   4   5   6   7   8   9 \n\n")(rest)?;
        let (rest, moves) = separated_list0(
            line_ending,
            map(
                tuple((tag("move "), u32, tag(" from "), u32, tag(" to "), u32)),
                |(_, amount, _, from, _, to)| Move {
                    amount: amount as usize,
                    from: from as usize,
                    to: to as usize,
                },
            ),
        )(rest)?;
        state.moves = moves;
        Ok((rest, state))
    }

    type Output1 = String;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut state = input.clone();
        for m in state.moves.clone() {
            state.move_crates_9000(m.amount, m.from, m.to);
        }
        state.stacks.iter().map(|s| s.back().unwrap()).collect()
    }

    type Output2 = String;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut state = input.clone();
        for m in state.moves.clone() {
            state.move_crates_9001(m.amount, m.from, m.to);
        }
        state.stacks.iter().map(|s| s.back().unwrap()).collect()
    }
}
