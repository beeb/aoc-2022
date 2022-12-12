use itertools::Itertools;
use nom::{
    character::complete::{line_ending, not_line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day12;

fn find_start_end(input: &mut <Day12 as Day>::Input) -> ((usize, usize), (usize, usize)) {
    let mut start = (0, 0);
    let mut end = (0, 0);
    for (x, row) in input.iter_mut().enumerate() {
        for (y, cell) in row.iter_mut().enumerate() {
            if *cell == 'S' as usize {
                start = (x, y);
                *cell = 'a' as usize;
            } else if *cell == 'E' as usize {
                end = (x, y);
                *cell = 'z' as usize;
            }
        }
    }
    (start, end)
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
        println!("{:?}", input[0][0]);
        println!("{:?}", input[start.0][start.1]);
        0
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
