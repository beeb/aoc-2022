use nom::{
    character::complete::{i64, line_ending},
    multi::separated_list0,
    IResult,
};

use crate::days::Day;

const LENGTH: usize = 5000;

pub struct Day20;

fn mix(val: &mut [i64; LENGTH], idx: &mut [usize; LENGTH]) {
    for i in 0..LENGTH {
        let x = idx.iter().position(|&idx| idx == i).unwrap();
        let shift = val[x] % (LENGTH as i64 - 1);
        let dir = shift.signum();
        let stop = (x as i64) + shift;
        let mut j = x as i64;
        while j != stop {
            let j_wrap = j.rem_euclid(LENGTH as i64) as usize;
            let j_next = (j + dir).rem_euclid(LENGTH as i64) as usize;
            val.swap(j_wrap, j_next);
            idx.swap(j_wrap, j_next);

            j += dir;
        }
    }
}

fn calculate_output(val: &[i64; LENGTH]) -> i64 {
    let zero_offset = val.iter().position(|&v| v == 0).unwrap();
    (1..=3)
        .map(|x| {
            let idx = (zero_offset + 1000 * x) % LENGTH;
            val[idx]
        })
        .sum()
}

impl Day for Day20 {
    type Input = Vec<i64>;

    fn parse(input: &str) -> IResult<&str, Self::Input> {
        separated_list0(line_ending, i64)(input)
    }

    type Output1 = i64;

    fn part_1(input: &Self::Input) -> Self::Output1 {
        let mut val: [i64; LENGTH] = input[..LENGTH].try_into().expect("wrong array length");
        let mut idx: [usize; LENGTH] = (0..LENGTH).collect::<Vec<_>>().try_into().unwrap();
        mix(&mut val, &mut idx);
        calculate_output(&val)
    }

    type Output2 = i64;

    fn part_2(input: &Self::Input) -> Self::Output2 {
        let mut val: [i64; LENGTH] = input.iter().map(|v| v * 811589153).collect::<Vec<_>>()
            [..LENGTH]
            .try_into()
            .expect("wrong array length");
        let mut idx: [usize; LENGTH] = (0..LENGTH).collect::<Vec<_>>().try_into().unwrap();
        for _ in 0..10 {
            mix(&mut val, &mut idx);
        }
        calculate_output(&val)
    }
}
