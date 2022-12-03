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
                // split each line into 2 equal parts
                let mid = rs.len() / 2;
                (rs[..mid].to_vec(), rs[mid..].to_vec())
            })
            .flat_map(|rs| {
                // only keep items from first half that appear in second half (and dedup)
                rs.0.into_iter().filter(move |e| rs.1.contains(e)).dedup()
            })
            .map(|e| e as usize) // cast to usize for summing
            .sum::<usize>()
    }

    type Output2 = usize;

    /// Ugly first version
    /// took 0.4504ms
    /// ```
    /// fn part_2(input: &Self::Input) -> Self::Output2 {
    ///     let dedup = input
    ///         .iter()
    ///         .map(|rs| rs.iter().sorted().dedup().collect_vec()) // dedup each line
    ///         .collect_vec();
    ///     let test = dedup
    ///         .chunks_exact(3) // loop in groups of 3
    ///         .map(|gr| {
    ///             // get how many times each item appears in the concatenated 3 lines
    ///             let mut counts = gr.concat().into_iter().counts();
    ///             counts.retain(|_, v| v == &3); // only 1 item should appear thrice
    ///             let item = counts.keys().next().unwrap(); // the key of that single item is the result
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
        // we loop in groups of 3
        for gr in input
            .iter()
            .map(|rs| HashSet::from_iter(rs.iter())) // we create hashsets to dedup
            .collect_vec() // vec of hashsets
            .chunks_exact(3)
        {
            // intersection of the first two sets, collected into another hashset
            let common: HashSet<_> = gr[0].intersection(&gr[1]).cloned().collect();
            // intersection with the last set (no collecting, so it's an iterator)
            let mut common = common.intersection(&gr[2]);
            // first item in the iterator is the value that was present in all 3 sets
            total += **(common.next().unwrap()) as usize
        }
        total
    }
}
