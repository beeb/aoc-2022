use itertools::Itertools;
use nom::{
    character::complete::{digit1, line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day08;

fn transpose<T>(v: &Vec<Vec<T>>) -> Vec<Vec<T>>
where
    T: Clone,
{
    assert!(!v.is_empty());
    (0..v[0].len())
        .map(|i| v.iter().map(|inner| inner[i].clone()).collect_vec())
        .collect()
}

impl Day for Day08 {
    type Input = Vec<Vec<u8>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(digit1, |s: &str| {
                s.chars().map(|c| (c as u8) - 48).collect_vec()
            }),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        /*
        +-------> y
        |
        v
        x
        */
        let rows = input;
        let cols = transpose(input);
        let mut visible: Vec<Vec<usize>> = vec![vec![0; cols.len()]; rows.len()];
        for x in 1..rows.len() - 1 {
            for y in 1..cols.len() - 1 {
                let left = rows[x][0..y].iter().max().unwrap();
                if left < &rows[x][y] {
                    visible[x][y] = 1;
                    continue;
                }
                let top = cols[y][0..x].iter().max().unwrap();
                if top < &rows[x][y] {
                    visible[x][y] = 1;
                    continue;
                }
                let right = rows[x][y + 1..].iter().max().unwrap();
                if right < &rows[x][y] {
                    visible[x][y] = 1;
                    continue;
                }
                let bottom = cols[y][x + 1..].iter().max().unwrap();
                if bottom < &rows[x][y] {
                    visible[x][y] = 1;
                    continue;
                }
            }
        }
        let sides = 2 * rows.len() + 2 * (cols.len() - 2);
        visible.iter().flatten().sum::<usize>() + sides
    }

    type Output2 = usize;

    fn part_2(_input: &Self::Input) -> Self::Output2 {
        unimplemented!("part_2")
    }
}
