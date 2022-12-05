use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{anychar, char, line_ending, not_line_ending, u32},
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

#[derive(Debug, Clone, Default)]
pub struct State {
    /// Each stack is represented as a double-ended queue, where the front is the bottom and the back is the top
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

    /// Fill the stacks with the input data, from the front so the order is preserved
    pub fn push_crate_front(&mut self, stack: usize, id: char) -> &mut Self {
        self.stacks.get_mut(stack).unwrap().push_front(id);
        self
    }

    /// In the first part, we move the crates one-by-one, so pop from the back (top) and push to the back (top)
    pub fn move_crates_9000(&mut self, amount: usize, from: usize, to: usize) -> &mut Self {
        for _ in 0..amount {
            let c = self.stacks[from - 1].pop_back().unwrap();
            self.stacks[to - 1].push_back(c);
        }
        self
    }

    /// In the second part, we move the crates in batches of `amount`.
    ///
    /// Cleaner-looking version where an intermediary VecDeque is created to hold the moved crates.
    pub fn move_crates_9001b(&mut self, amount: usize, from: usize, to: usize) -> &mut Self {
        let from_len = self.stacks[from - 1].len();
        let mut mov = self.stacks[from - 1].split_off(from_len - amount);
        self.stacks[to - 1].append(&mut mov);
        self
    }

    /// In the second part, we move the crates in batches of `amount`.
    ///
    /// Alternative version where no intermediary VecDeque is created.
    /// In order to get mutable references to two different stacks in the slice of stacks,
    /// we use `split_at_mut`, which gives two mutable references to different parts of the slice.
    /// We choose to split at the highest index between `from` and `to`, so that the first element in the
    /// second slice is the stack referring to max(from, to) and both stacks of interest are in different slices.
    pub fn move_crates_9001(&mut self, amount: usize, from: usize, to: usize) -> &mut Self {
        if from > to {
            let (half_to, half_from) = self.stacks.split_at_mut(from - 1);
            let from_stack = half_from.first_mut().unwrap();
            let to_stack = half_to.get_mut(to - 1).unwrap();
            move_crates_with_stacks(from_stack, to_stack, amount);
        } else {
            let (half_from, half_to) = self.stacks.split_at_mut(to - 1);
            let to_stack = half_to.first_mut().unwrap();
            let from_stack = half_from.get_mut(from - 1).unwrap();
            move_crates_with_stacks(from_stack, to_stack, amount);
        }
        self
    }
}

/// Get an iterator for the last `amount` elements of the first stack, and push them directly onto the second stack
fn move_crates_with_stacks(
    from_stack: &mut VecDeque<char>,
    to_stack: &mut VecDeque<char>,
    amount: usize,
) {
    from_stack
        .drain(from_stack.len() - amount..)
        .for_each(|c| to_stack.push_back(c));
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
            for (i, (_, c, _)) in layer.iter().enumerate() {
                match c {
                    ' ' => {}
                    c => {
                        state.push_crate_front(i, *c);
                    }
                }
            }
        }
        let (rest, _) = tuple((not_line_ending, tag("\n\n")))(rest)?;
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

    /// Part 1 took 0.020906ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut state = input.clone(); // get a copy of the state
        for m in state.moves.clone() {
            state.move_crates_9000(m.amount, m.from, m.to);
        }
        state.stacks.iter().map(|s| s.back().unwrap()).collect()
    }

    type Output2 = String;

    /// Part 2 took 0.013784ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut state = input.clone(); // get a copy of the state
        for m in state.moves.clone() {
            state.move_crates_9001(m.amount, m.from, m.to);
        }
        state.stacks.iter().map(|s| s.back().unwrap()).collect()
    }
}
