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
        for (x, row) in rows.iter().enumerate().take(rows.len() - 1).skip(1) {
            for (y, col) in cols.iter().enumerate().take(cols.len() - 1).skip(1) {
                let left = row[0..y].iter().max().unwrap();
                if left < &row[y] {
                    visible[x][y] = 1;
                    continue;
                }
                let top = col[0..x].iter().max().unwrap();
                if top < &row[y] {
                    visible[x][y] = 1;
                    continue;
                }
                let right = row[y + 1..].iter().max().unwrap();
                if right < &row[y] {
                    visible[x][y] = 1;
                    continue;
                }
                let bottom = col[x + 1..].iter().max().unwrap();
                if bottom < &row[y] {
                    visible[x][y] = 1;
                    continue;
                }
            }
        }
        let sides = 2 * rows.len() + 2 * (cols.len() - 2);
        visible.iter().flatten().sum::<usize>() + sides
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let rows = input;
        let cols = transpose(input);
        let mut score: Vec<Vec<usize>> = vec![vec![0; cols.len()]; rows.len()];
        for (x, row) in rows.iter().enumerate() {
            for (y, col) in cols.iter().enumerate() {
                let left = row[0..y]
                    .iter()
                    .rev()
                    .position(|h| h >= &row[y])
                    .map(|p| p + 1)
                    .unwrap_or(y);
                let top = col[0..x]
                    .iter()
                    .rev()
                    .position(|h| h >= &row[y])
                    .map(|p| p + 1)
                    .unwrap_or(x);
                let right = row[y + 1..]
                    .iter()
                    .position(|h| h >= &row[y])
                    .map(|p| p + 1)
                    .unwrap_or(row.len() - y - 1);
                let bottom = col[x + 1..]
                    .iter()
                    .position(|h| h >= &row[y])
                    .map(|p| p + 1)
                    .unwrap_or(col.len() - x - 1);
                score[x][y] = left * top * right * bottom;
            }
        }
        *score.iter().flatten().max().unwrap()
    }
}
