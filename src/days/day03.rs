use std::collections::HashSet;

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

    /// Cleaner-looking but less performant version
    /// took 0.2175ms
    /// ```
    /// fn part_1(input: &Self::Input) -> Self::Output1 {
    ///     let mut total = 0;
    ///     for rs in input {
    ///         let half = rs.len() / 2;
    ///         let left: HashSet<&u8> = HashSet::from_iter(rs.iter().take(half));
    ///         let right: HashSet<&u8> = HashSet::from_iter(rs.iter().skip(half));
    ///         let mut common = left.intersection(&right);
    ///         total += **(common.next().unwrap()) as usize
    ///     }
    ///     total
    /// }
    /// ```
    ///
    ///
    /// First ugly version but quite fast
    /// Part 1 took 0.057ms
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

    /// Ugly first version
    /// took 0.4504ms
    /// ```
    /// fn part_2(input: &Self::Input) -> Self::Output2 {
    ///     let dedup = input
    ///         .iter()
    ///         .map(|rs| rs.iter().sorted().dedup().collect_vec())
    ///         .collect_vec();
    ///     let test = dedup
    ///         .chunks_exact(3)
    ///         .map(|gr| {
    ///             let mut counts = gr.concat().into_iter().counts();
    ///             counts.retain(|_, v| v == &3);
    ///             let item = counts.keys().next().unwrap();
    ///             **item
    ///         })
    ///         .collect_vec();
    ///     test.iter().map(|e| *e as usize).sum()
    /// }
    /// ```
    ///
    /// After some optimization, here hashset is beneficial
    /// Part 2 took 0.2253ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut total = 0;
        for gr in input
            .iter()
            .map(|rs| HashSet::from_iter(rs.iter()))
            .collect_vec()
            .chunks_exact(3)
        {
            let common: HashSet<_> = gr[0].intersection(&gr[1]).cloned().collect();
            let mut common = common.intersection(&gr[2]);
            total += **(common.next().unwrap()) as usize
        }
        total
    }
}
