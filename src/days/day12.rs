use std::{cmp::Ordering, collections::BinaryHeap};

use itertools::Itertools;
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

#[derive(Debug)]
pub struct OpenPos {
    x: usize,
    y: usize,
    cost: usize,
}

impl Eq for OpenPos {}

impl PartialEq for OpenPos {
    fn eq(&self, other: &Self) -> bool {
        self.cost == other.cost
    }
}

impl PartialOrd for OpenPos {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        other.cost.partial_cmp(&self.cost)
    }
}

impl Ord for OpenPos {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

pub struct Day12;

fn find_start_end(input: &mut <Day12 as Day>::Input) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (x, row) in input.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            if *cell == 'S' as usize {
                start = (x, y);
                let val = 'a' as usize;
                *cell = val;
            } else if *cell == 'E' as usize {
                end = (x, y);
                *cell = 'z' as usize;
            }
        }
    }
    (start, end)
}

fn distance_to_end(x: usize, y: usize, end_x: usize, end_y: usize) -> usize {
    (x.max(end_x) - x.min(end_x)) + (y.max(end_y) - y.min(end_y))
}

impl Day for Day12 {
    type Input = Vec<Vec<usize>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(not_line_ending, |s: &str| {
                s.chars().map(|c| c as usize).collect_vec()
            }),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut grid = input.clone();
        let (start, end) = find_start_end(&mut grid);
        println!("{:?}, {:?}", start, end);
        let mut open_set = BinaryHeap::<OpenPos>::new();
        open_set.push(OpenPos {
            x: start.0,
            y: start.1,
            cost: distance_to_end(start.0, start.1, end.0, end.1),
        });
        println!("{open_set:?}");
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
