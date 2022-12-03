use itertools::Itertools;
use nom::{
    character::complete::{alpha1, line_ending},
    combinator::map,
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

pub struct Day03;

impl Day for Day03 {
    type Input = Vec<Vec<u8>>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(
            line_ending,
            map(alpha1, |s: &str| {
                s.chars()
                    .map(|c| {
                        if c.is_uppercase() {
                            return (c as u8) - 38;
                        }
                        (c as u8) - 96
                    })
                    .collect_vec()
            }),
        )(input)
    }

    type Output1 = usize;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        input
            .iter()
            .map(|rs| {
                let mid = rs.len() / 2;
                (rs[..mid].to_vec(), rs[mid..].to_vec())
            })
            .flat_map(|rs| rs.0.into_iter().filter(move |e| rs.1.contains(e)).dedup())
            .map(|e| e as usize)
            .sum::<usize>()
    }

    type Output2 = usize;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let dedup = input
            .iter()
            .map(|rs| rs.iter().sorted().dedup().collect_vec())
            .collect_vec();
        let test = dedup
            .chunks_exact(3)
            .map(|gr| {
                let mut counts = gr.concat().into_iter().counts();
                counts.retain(|_, v| v == &3);
                let item = counts.keys().next().unwrap();
                **item
            })
            .collect_vec();
        test.iter().map(|e| *e as usize).sum()
    }
}
