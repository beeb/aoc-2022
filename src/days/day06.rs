use itertools::Itertools;
use nom::IResult;

use crate::days::Day;

pub struct Day06;

/// First easy and clean solution
#[allow(unused)]
fn pos_of_unique_pattern(input: &[u8], len: usize) -> usize {
    assert!(len > 0);
    for (i, seq) in input.windows(len).enumerate() {
        if seq.iter().all_unique() {
            return i + len;
        }
    }
    0 // sentinel value for not found (minimum is otherwise `len`)
}

/// Faster solution that doesn't rely on HashSet (which `all_unique` above uses internally)
fn pos_of_unique_pattern2(input: &[u8], len: usize) -> usize {
    assert!(len > 0);
    for (i, seq) in input.windows(len).enumerate() {
        if seq
            .iter()
            .enumerate()
            .all(|(idx, c)| !seq[..idx].contains(c))
        {
            return i + len;
        }
    }
    0 // sentinel value for not found (minimum is otherwise `len`)
}

impl Day for Day06 {
    type Input = String;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        Ok(("", String::from(input)))
    }

    type Output1 = usize;

    /// Part 1 took 0.006ms
    fn part_1(input: &Self::Input) -> Self::Output1 {
        pos_of_unique_pattern2(input.as_bytes(), 4)
    }

    type Output2 = usize;

    /// Part 2 took 0.0231ms
    fn part_2(input: &Self::Input) -> Self::Output2 {
        pos_of_unique_pattern2(input.as_bytes(), 14)
    }
}
